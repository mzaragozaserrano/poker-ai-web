#!/usr/bin/env pwsh
# Script para compilar poker-ffi como Python wheel usando Maturin
# Genera un paquete .whl que puede ser instalado con pip

param(
    [string]$OutputDir = "dist/wheels",
    [string]$PythonVersion = "3.11",
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

Write-Host "=== Build poker-ffi Python Wheel ===" -ForegroundColor Cyan

# Verificar que estamos en la raíz del proyecto
if (-not (Test-Path "backend/ffi/Cargo.toml")) {
    Write-Error "ERROR: Debe ejecutar este script desde la raíz del proyecto"
    exit 1
}

# Crear directorio de salida
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# Verificar que maturin está instalado
Write-Host "Verificando maturin..." -ForegroundColor Yellow
$maturinVersion = & maturin --version 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Error "ERROR: maturin no está instalado. Instale con: pip install maturin"
    exit 1
}
Write-Host "  $maturinVersion" -ForegroundColor Gray

# Configurar RUSTFLAGS para optimización específica de Ryzen
$env:RUSTFLAGS = "-C target-cpu=znver2 -C target-feature=+avx2"

Write-Host "Configuración de compilación:" -ForegroundColor Yellow
Write-Host "  - Target CPU: znver2 (Ryzen 3000 series)"
Write-Host "  - Features: AVX2 SIMD"
Write-Host "  - Python: $PythonVersion"
Write-Host "  - Strip symbols: Enabled"
Write-Host ""

# Cambiar al directorio ffi
Push-Location backend/ffi

try {
    # Limpiar builds anteriores
    Write-Host "Limpiando builds anteriores..." -ForegroundColor Yellow
    cargo clean --release

    # Compilar wheel con Maturin
    Write-Host "Compilando wheel con Maturin (esto puede tomar varios minutos)..." -ForegroundColor Yellow
    
    $maturinArgs = @(
        "build",
        "--release",
        "--strip",
        "--out", "../../$OutputDir"
    )

    if ($Verbose) {
        $maturinArgs += "--verbose"
    }

    & maturin @maturinArgs

    if ($LASTEXITCODE -ne 0) {
        throw "Error en compilación de wheel"
    }

    Write-Host "Compilación exitosa" -ForegroundColor Green
    Write-Host ""

    # Listar wheels generadas
    Write-Host "Wheels generadas:" -ForegroundColor Yellow
    $wheels = Get-ChildItem -Path "../../$OutputDir" -Filter "*.whl"
    foreach ($wheel in $wheels) {
        $size = [math]::Round($wheel.Length / 1MB, 2)
        Write-Host "  - $($wheel.Name) ($size MB)" -ForegroundColor Gray
    }

    Write-Host ""
    Write-Host "=== Build FFI Completado ===" -ForegroundColor Green
    Write-Host "Wheels en: $OutputDir" -ForegroundColor Green
    Write-Host ""
    Write-Host "Para instalar localmente:" -ForegroundColor Cyan
    Write-Host "  pip install $OutputDir/poker_ffi-*.whl" -ForegroundColor White

} finally {
    Pop-Location
}

