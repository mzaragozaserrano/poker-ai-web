#!/usr/bin/env pwsh
# Script para compilar el backend Rust en modo release optimizado
# Optimizado para Ryzen 7 3800X (16 threads, AVX2)

param(
    [string]$OutputDir = "dist/backend",
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

Write-Host "=== Build Rust Backend (Release) ===" -ForegroundColor Cyan

# Verificar que estamos en la raíz del proyecto
if (-not (Test-Path "backend/Cargo.toml")) {
    Write-Error "ERROR: Debe ejecutar este script desde la raíz del proyecto"
    exit 1
}

# Crear directorio de salida
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# Configurar RUSTFLAGS para optimización específica de Ryzen (AVX2, Zen 2)
$env:RUSTFLAGS = "-C target-cpu=znver2 -C target-feature=+avx2"

Write-Host "Configuración de compilación:" -ForegroundColor Yellow
Write-Host "  - Target CPU: znver2 (Ryzen 3000 series)"
Write-Host "  - Features: AVX2 SIMD"
Write-Host "  - LTO: Enabled"
Write-Host "  - Strip: Enabled"
Write-Host ""

# Cambiar al directorio backend
Push-Location backend

try {
    # Limpiar builds anteriores
    Write-Host "Limpiando builds anteriores..." -ForegroundColor Yellow
    cargo clean --release

    # Compilar workspace completo en release
    Write-Host "Compilando workspace Rust (esto puede tomar varios minutos)..." -ForegroundColor Yellow
    if ($Verbose) {
        cargo build --release --workspace --verbose
    } else {
        cargo build --release --workspace
    }

    if ($LASTEXITCODE -ne 0) {
        throw "Error en compilación de Rust"
    }

    Write-Host "Compilación exitosa" -ForegroundColor Green

    # Copiar binarios al directorio de salida
    Write-Host "Copiando artefactos a $OutputDir..." -ForegroundColor Yellow
    
    # Copiar librerías compiladas (si existen binarios)
    $targetDir = "target/release"
    
    # Buscar archivos .dll (Windows) y .so (Linux)
    $libraries = Get-ChildItem -Path $targetDir -Filter "*.dll" -ErrorAction SilentlyContinue
    foreach ($lib in $libraries) {
        Copy-Item $lib.FullName -Destination "../$OutputDir/" -Force
        Write-Host "  Copiado: $($lib.Name)" -ForegroundColor Gray
    }

    # Copiar ejecutables si existen (ej: demos, benchmarks)
    $executables = Get-ChildItem -Path $targetDir -Filter "*.exe" -ErrorAction SilentlyContinue
    foreach ($exe in $executables) {
        Copy-Item $exe.FullName -Destination "../$OutputDir/" -Force
        Write-Host "  Copiado: $($exe.Name)" -ForegroundColor Gray
    }

    Write-Host ""
    Write-Host "=== Build Rust Completado ===" -ForegroundColor Green
    Write-Host "Artefactos en: $OutputDir" -ForegroundColor Green

} finally {
    Pop-Location
}

