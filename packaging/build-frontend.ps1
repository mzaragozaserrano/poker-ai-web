#!/usr/bin/env pwsh
# Script para compilar el frontend React con Vite
# Genera archivos estáticos optimizados para producción

param(
    [string]$OutputDir = "dist/frontend",
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

Write-Host "=== Build Frontend React (Vite) ===" -ForegroundColor Cyan

# Verificar que estamos en la raíz del proyecto
if (-not (Test-Path "frontend/package.json")) {
    Write-Error "ERROR: Debe ejecutar este script desde la raíz del proyecto"
    exit 1
}

# Verificar que node está instalado
Write-Host "Verificando Node.js..." -ForegroundColor Yellow
$nodeVersion = & node --version 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Error "ERROR: Node.js no está instalado"
    exit 1
}
Write-Host "  Node.js $nodeVersion" -ForegroundColor Gray

# Verificar que npm está instalado
$npmVersion = & npm --version 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Error "ERROR: npm no está instalado"
    exit 1
}
Write-Host "  npm $npmVersion" -ForegroundColor Gray
Write-Host ""

# Cambiar al directorio frontend
Push-Location frontend

try {
    # Instalar dependencias si no existen
    if (-not (Test-Path "node_modules")) {
        Write-Host "Instalando dependencias npm..." -ForegroundColor Yellow
        npm install
        if ($LASTEXITCODE -ne 0) {
            throw "Error instalando dependencias npm"
        }
    } else {
        Write-Host "Dependencias npm ya instaladas" -ForegroundColor Gray
    }

    # Limpiar build anterior
    if (Test-Path "dist") {
        Write-Host "Limpiando build anterior..." -ForegroundColor Yellow
        Remove-Item -Path "dist" -Recurse -Force
    }

    # Compilar con Vite
    Write-Host "Compilando frontend con Vite (optimizado para producción)..." -ForegroundColor Yellow
    
    if ($Verbose) {
        npm run build -- --mode production --logLevel info
    } else {
        npm run build -- --mode production
    }

    if ($LASTEXITCODE -ne 0) {
        throw "Error en compilación de frontend"
    }

    Write-Host "Compilación exitosa" -ForegroundColor Green
    Write-Host ""

    # Copiar build al directorio de salida
    Write-Host "Copiando artefactos a ../$OutputDir..." -ForegroundColor Yellow
    
    # Crear directorio de salida en la raíz del proyecto
    New-Item -ItemType Directory -Force -Path "../$OutputDir" | Out-Null
    
    # Copiar todo el contenido de dist
    Copy-Item -Path "dist/*" -Destination "../$OutputDir/" -Recurse -Force

    # Calcular tamaño del build
    $buildSize = (Get-ChildItem -Path "dist" -Recurse | Measure-Object -Property Length -Sum).Sum
    $buildSizeMB = [math]::Round($buildSize / 1MB, 2)

    Write-Host ""
    Write-Host "=== Build Frontend Completado ===" -ForegroundColor Green
    Write-Host "Artefactos en: $OutputDir" -ForegroundColor Green
    Write-Host "Tamaño total: $buildSizeMB MB" -ForegroundColor Gray
    Write-Host ""
    
    # Listar archivos principales
    Write-Host "Archivos principales:" -ForegroundColor Yellow
    $indexFile = Get-Item "../$OutputDir/index.html" -ErrorAction SilentlyContinue
    if ($indexFile) {
        Write-Host "  - index.html" -ForegroundColor Gray
    }
    
    $jsFiles = Get-ChildItem -Path "../$OutputDir/assets" -Filter "*.js" -ErrorAction SilentlyContinue
    Write-Host "  - $($jsFiles.Count) archivos JavaScript" -ForegroundColor Gray
    
    $cssFiles = Get-ChildItem -Path "../$OutputDir/assets" -Filter "*.css" -ErrorAction SilentlyContinue
    Write-Host "  - $($cssFiles.Count) archivos CSS" -ForegroundColor Gray

} finally {
    Pop-Location
}

