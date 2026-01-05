#!/usr/bin/env pwsh
# Script maestro de empaquetado para Windows
# Orquesta todos los builds y crea el paquete autocontenido

param(
    [string]$OutputDir = "release/poker-analyzer",
    [string]$PythonEmbeddedUrl = "https://www.python.org/ftp/python/3.11.9/python-3.11.9-embed-amd64.zip",
    [switch]$SkipBuild,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Poker Analyzer - Empaquetado Windows" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Verificar que estamos en la raíz del proyecto
if (-not (Test-Path "backend/Cargo.toml")) {
    Write-Error "ERROR: Debe ejecutar este script desde la raíz del proyecto"
    exit 1
}

# Crear directorio de salida
Write-Host "Creando estructura de directorios..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
New-Item -ItemType Directory -Force -Path "$OutputDir/backend" | Out-Null
New-Item -ItemType Directory -Force -Path "$OutputDir/frontend" | Out-Null
New-Item -ItemType Directory -Force -Path "$OutputDir/data" | Out-Null
New-Item -ItemType Directory -Force -Path "$OutputDir/logs" | Out-Null
Write-Host "  Estructura creada" -ForegroundColor Green

# ============================================
# FASE 1: Build de componentes
# ============================================

if (-not $SkipBuild) {
    Write-Host ""
    Write-Host "=== FASE 1: Compilando Componentes ===" -ForegroundColor Cyan
    
    # Build Backend Rust
    Write-Host ""
    & ./packaging/build-rust.ps1 -OutputDir "dist/backend" -Verbose:$Verbose
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Error en build de Rust"
        exit 1
    }
    
    # Build FFI Wheel
    Write-Host ""
    & ./packaging/build-ffi.ps1 -OutputDir "dist/wheels" -Verbose:$Verbose
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Error en build de FFI"
        exit 1
    }
    
    # Build Frontend
    Write-Host ""
    & ./packaging/build-frontend.ps1 -OutputDir "dist/frontend" -Verbose:$Verbose
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Error en build de frontend"
        exit 1
    }
} else {
    Write-Host "Saltando builds (usando artefactos existentes)" -ForegroundColor Yellow
}

# ============================================
# FASE 2: Descargar Python Embebido
# ============================================

Write-Host ""
Write-Host "=== FASE 2: Python Embebido ===" -ForegroundColor Cyan

$pythonZip = "dist/python-embed.zip"
$pythonDir = "$OutputDir/backend/python311"

if (-not (Test-Path $pythonDir)) {
    Write-Host "Descargando Python embebido..." -ForegroundColor Yellow
    
    if (-not (Test-Path $pythonZip)) {
        Invoke-WebRequest -Uri $PythonEmbeddedUrl -OutFile $pythonZip
        Write-Host "  Descargado: $pythonZip" -ForegroundColor Gray
    } else {
        Write-Host "  Usando cache: $pythonZip" -ForegroundColor Gray
    }
    
    Write-Host "Extrayendo Python embebido..." -ForegroundColor Yellow
    Expand-Archive -Path $pythonZip -DestinationPath $pythonDir -Force
    Write-Host "  Python extraído en: $pythonDir" -ForegroundColor Green
    
    # Configurar Python embebido para permitir imports
    $pthFile = "$pythonDir/python311._pth"
    if (Test-Path $pthFile) {
        $content = Get-Content $pthFile
        $content = $content -replace "#import site", "import site"
        Set-Content -Path $pthFile -Value $content
        Write-Host "  Configurado site-packages" -ForegroundColor Gray
    }
} else {
    Write-Host "Python embebido ya existe" -ForegroundColor Gray
}

# ============================================
# FASE 3: Instalar dependencias Python
# ============================================

Write-Host ""
Write-Host "=== FASE 3: Dependencias Python ===" -ForegroundColor Cyan

$pythonExe = "$pythonDir/python.exe"

# Instalar pip en Python embebido
if (-not (Test-Path "$pythonDir/Scripts/pip.exe")) {
    Write-Host "Instalando pip..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://bootstrap.pypa.io/get-pip.py" -OutFile "dist/get-pip.py"
    & $pythonExe "dist/get-pip.py" --no-warn-script-location
    Write-Host "  pip instalado" -ForegroundColor Green
}

# Instalar wheel de poker-ffi
Write-Host "Instalando poker-ffi wheel..." -ForegroundColor Yellow
$ffiWheel = Get-ChildItem -Path "dist/wheels" -Filter "poker_ffi-*.whl" | Select-Object -First 1
if ($ffiWheel) {
    & $pythonExe -m pip install $ffiWheel.FullName --no-warn-script-location --target "$pythonDir/Lib/site-packages"
    Write-Host "  poker-ffi instalado" -ForegroundColor Green
} else {
    Write-Error "No se encontró wheel de poker-ffi"
    exit 1
}

# Instalar dependencias del server-api desde pyproject.toml
Write-Host "Instalando dependencias de server-api..." -ForegroundColor Yellow
Push-Location server-api
try {
    # Leer dependencias del pyproject.toml
    $dependencies = @(
        "fastapi",
        "uvicorn[standard]",
        "pydantic",
        "structlog",
        "python-multipart"
    )
    
    foreach ($dep in $dependencies) {
        Write-Host "  Instalando $dep..." -ForegroundColor Gray
        & $pythonExe -m pip install $dep --no-warn-script-location --target "$pythonDir/Lib/site-packages"
    }
    Write-Host "  Dependencias instaladas" -ForegroundColor Green
} finally {
    Pop-Location
}

# ============================================
# FASE 4: Copiar artefactos
# ============================================

Write-Host ""
Write-Host "=== FASE 4: Copiando Artefactos ===" -ForegroundColor Cyan

# Copiar server-api
Write-Host "Copiando server-api..." -ForegroundColor Yellow
Copy-Item -Path "server-api/app" -Destination "$OutputDir/backend/" -Recurse -Force
Write-Host "  server-api copiado" -ForegroundColor Green

# Copiar frontend
Write-Host "Copiando frontend..." -ForegroundColor Yellow
if (Test-Path "dist/frontend") {
    Copy-Item -Path "dist/frontend/*" -Destination "$OutputDir/frontend/" -Recurse -Force
    Write-Host "  frontend copiado" -ForegroundColor Green
} else {
    Write-Warning "No se encontró build de frontend en dist/frontend"
}

# Copiar configuración
Write-Host "Copiando configuración..." -ForegroundColor Yellow
Copy-Item -Path "packaging/config.template.toml" -Destination "$OutputDir/config.toml" -Force
Write-Host "  config.toml copiado" -ForegroundColor Green

# Copiar README y LICENSE
if (Test-Path "README.md") {
    Copy-Item -Path "README.md" -Destination "$OutputDir/" -Force
}
if (Test-Path "LICENSE") {
    Copy-Item -Path "LICENSE" -Destination "$OutputDir/" -Force
}

# ============================================
# FASE 5: Crear launcher
# ============================================

Write-Host ""
Write-Host "=== FASE 5: Creando Launcher ===" -ForegroundColor Cyan
Write-Host "El launcher se creará en la siguiente fase..." -ForegroundColor Yellow

# TODO: Crear launcher Rust o script PowerShell compilado
# Por ahora, crear un script PowerShell simple

$launcherScript = @"
#!/usr/bin/env pwsh
# Launcher de Poker Analyzer

`$ErrorActionPreference = "Stop"

Write-Host "=== Iniciando Poker Analyzer ===" -ForegroundColor Cyan

# Obtener directorio de instalación
`$installDir = `$PSScriptRoot

# Iniciar backend
Write-Host "Iniciando servidor backend..." -ForegroundColor Yellow
`$pythonExe = "`$installDir\backend\python311\python.exe"
`$serverScript = "`$installDir\backend\app\main.py"

Start-Process -FilePath `$pythonExe -ArgumentList `$serverScript -NoNewWindow -PassThru | Out-Null

# Esperar a que el servidor esté listo
Write-Host "Esperando servidor..." -ForegroundColor Yellow
Start-Sleep -Seconds 3

# Abrir navegador
Write-Host "Abriendo navegador..." -ForegroundColor Yellow
Start-Process "http://127.0.0.1:8000"

Write-Host ""
Write-Host "=== Poker Analyzer Iniciado ===" -ForegroundColor Green
Write-Host "Servidor: http://127.0.0.1:8000" -ForegroundColor Cyan
Write-Host "Presione Ctrl+C para detener" -ForegroundColor Gray
Write-Host ""

# Mantener el script corriendo
while (`$true) {
    Start-Sleep -Seconds 1
}
"@

Set-Content -Path "$OutputDir/start-poker-analyzer.ps1" -Value $launcherScript
Write-Host "  Launcher script creado: start-poker-analyzer.ps1" -ForegroundColor Green

# ============================================
# FASE 6: Calcular tamaño final
# ============================================

Write-Host ""
Write-Host "=== FASE 6: Validación Final ===" -ForegroundColor Cyan

$totalSize = (Get-ChildItem -Path $OutputDir -Recurse | Measure-Object -Property Length -Sum).Sum
$totalSizeMB = [math]::Round($totalSize / 1MB, 2)

Write-Host "Tamaño total del paquete: $totalSizeMB MB" -ForegroundColor Yellow

if ($totalSizeMB -gt 500) {
    Write-Warning "El paquete excede los 500MB objetivo ($totalSizeMB MB)"
} else {
    Write-Host "  Tamaño dentro del objetivo (< 500MB)" -ForegroundColor Green
}

# Listar contenido principal
Write-Host ""
Write-Host "Contenido del paquete:" -ForegroundColor Yellow
Get-ChildItem -Path $OutputDir -Directory | ForEach-Object {
    $dirSize = (Get-ChildItem -Path $_.FullName -Recurse | Measure-Object -Property Length -Sum).Sum
    $dirSizeMB = [math]::Round($dirSize / 1MB, 2)
    Write-Host "  - $($_.Name): $dirSizeMB MB" -ForegroundColor Gray
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Empaquetado Completado" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Paquete generado en: $OutputDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "Para ejecutar:" -ForegroundColor Yellow
Write-Host "  cd $OutputDir" -ForegroundColor White
Write-Host "  .\start-poker-analyzer.ps1" -ForegroundColor White
Write-Host ""

