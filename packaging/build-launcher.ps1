#!/usr/bin/env pwsh
# Script para compilar el launcher en Rust

param(
    [string]$OutputDir = "dist/launcher",
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

Write-Host "=== Build Launcher ===" -ForegroundColor Cyan

# Verificar que estamos en la raíz del proyecto
if (-not (Test-Path "packaging/launcher/Cargo.toml")) {
    Write-Error "ERROR: packaging/launcher/Cargo.toml no encontrado"
    exit 1
}

# Crear directorio de salida
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# Cambiar al directorio del launcher
Push-Location packaging/launcher

try {
    # Compilar en release con optimización de tamaño
    Write-Host "Compilando launcher (optimizado para tamaño)..." -ForegroundColor Yellow
    
    if ($Verbose) {
        cargo build --release --verbose
    } else {
        cargo build --release
    }

    if ($LASTEXITCODE -ne 0) {
        throw "Error en compilación del launcher"
    }

    Write-Host "Compilación exitosa" -ForegroundColor Green

    # Copiar ejecutable
    $exePath = "target/release/poker-analyzer.exe"
    if (Test-Path $exePath) {
        Copy-Item -Path $exePath -Destination "../../$OutputDir/" -Force
        
        $exeSize = [math]::Round((Get-Item "../../$OutputDir/poker-analyzer.exe").Length / 1KB, 2)
        Write-Host ""
        Write-Host "=== Launcher Compilado ===" -ForegroundColor Green
        Write-Host "Ubicación: $OutputDir/poker-analyzer.exe" -ForegroundColor Cyan
        Write-Host "Tamaño: $exeSize KB" -ForegroundColor Gray
    } else {
        throw "No se encontró el ejecutable compilado"
    }

} finally {
    Pop-Location
}

