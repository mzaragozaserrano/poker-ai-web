#!/usr/bin/env pwsh
# Script para crear ZIP autocontenido del paquete

param(
    [string]$SourceDir = "release/poker-analyzer",
    [string]$OutputZip = "release/poker-analyzer-windows-x64.zip",
    [switch]$Validate
)

$ErrorActionPreference = "Stop"

Write-Host "=== Crear Paquete ZIP ===" -ForegroundColor Cyan

# Verificar que existe el directorio fuente
if (-not (Test-Path $SourceDir)) {
    Write-Error "ERROR: Directorio $SourceDir no encontrado. Ejecute package-windows.ps1 primero"
    exit 1
}

# Crear directorio de salida
$outputDir = Split-Path -Parent $OutputZip
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null

# Eliminar ZIP anterior si existe
if (Test-Path $OutputZip) {
    Write-Host "Eliminando ZIP anterior..." -ForegroundColor Yellow
    Remove-Item $OutputZip -Force
}

# Crear ZIP
Write-Host "Comprimiendo $SourceDir..." -ForegroundColor Yellow
Compress-Archive -Path "$SourceDir/*" -DestinationPath $OutputZip -CompressionLevel Optimal

if (Test-Path $OutputZip) {
    $zipSize = [math]::Round((Get-Item $OutputZip).Length / 1MB, 2)
    
    Write-Host ""
    Write-Host "=== ZIP Creado ===" -ForegroundColor Green
    Write-Host "Ubicación: $OutputZip" -ForegroundColor Cyan
    Write-Host "Tamaño: $zipSize MB" -ForegroundColor Gray
    
    # Validación
    if ($Validate) {
        Write-Host ""
        Write-Host "Validando ZIP..." -ForegroundColor Yellow
        
        if ($zipSize -gt 500) {
            Write-Warning "El ZIP excede los 500MB objetivo ($zipSize MB)"
            Write-Host "  Considere optimizar los siguientes componentes:" -ForegroundColor Yellow
            Write-Host "    - Python embebido (puede reducirse eliminando bibliotecas no usadas)" -ForegroundColor Gray
            Write-Host "    - Frontend assets (comprimir imágenes, eliminar source maps)" -ForegroundColor Gray
        } else {
            Write-Host "  ✓ Tamaño dentro del objetivo (< 500MB)" -ForegroundColor Green
        }
        
        # Listar contenido del ZIP
        Write-Host ""
        Write-Host "Contenido del ZIP:" -ForegroundColor Yellow
        $zipContent = Get-ChildItem -Path $SourceDir -Directory
        foreach ($item in $zipContent) {
            Write-Host "  - $($item.Name)/" -ForegroundColor Gray
        }
        
        $zipFiles = Get-ChildItem -Path $SourceDir -File
        foreach ($file in $zipFiles) {
            Write-Host "  - $($file.Name)" -ForegroundColor Gray
        }
    }
    
    Write-Host ""
    Write-Host "Paquete listo para distribución" -ForegroundColor Green
    Write-Host ""
    Write-Host "Instrucciones de instalación:" -ForegroundColor Cyan
    Write-Host "  1. Extraer ZIP en cualquier directorio" -ForegroundColor White
    Write-Host "  2. Ejecutar poker-analyzer.exe" -ForegroundColor White
    Write-Host "  3. Configurar ruta de historiales en config.toml" -ForegroundColor White
    Write-Host ""
} else {
    Write-Error "Error creando ZIP"
    exit 1
}

