# Security Verification Script
# Verifica que la API está configurada correctamente para localhost-only

$ErrorActionPreference = "Stop"

Write-Host "=== Poker AI Security Verification ===" -ForegroundColor Cyan
Write-Host ""

# 1. Verificar configuración de settings
Write-Host "[1/5] Verificando configuración..." -ForegroundColor Yellow
$settingsFile = Join-Path $PSScriptRoot ".." "app" "config" "settings.py"
$settingsContent = Get-Content $settingsFile -Raw

if ($settingsContent -match 'api_host:\s*str\s*=\s*"127\.0\.0\.1"') {
    Write-Host "  ✓ api_host configurado a 127.0.0.1" -ForegroundColor Green
} else {
    Write-Host "  ✗ api_host NO está configurado correctamente" -ForegroundColor Red
    exit 1
}

# 2. Verificar que no hay 0.0.0.0 en el código
Write-Host "[2/5] Verificando que no hay binding a 0.0.0.0..." -ForegroundColor Yellow
$mainFile = Join-Path $PSScriptRoot ".." "app" "main.py"
$mainContent = Get-Content $mainFile -Raw

if ($mainContent -match '0\.0\.0\.0') {
    Write-Host "  ✗ ADVERTENCIA: Se encontró 0.0.0.0 en main.py" -ForegroundColor Red
    exit 1
} else {
    Write-Host "  ✓ No se encontró 0.0.0.0 en el código" -ForegroundColor Green
}

# 3. Verificar que LocalhostOnlyMiddleware existe
Write-Host "[3/5] Verificando middleware de seguridad..." -ForegroundColor Yellow
$middlewareFile = Join-Path $PSScriptRoot ".." "app" "middleware" "security.py"

if (Test-Path $middlewareFile) {
    Write-Host "  ✓ LocalhostOnlyMiddleware existe" -ForegroundColor Green
    
    $middlewareContent = Get-Content $middlewareFile -Raw
    if ($middlewareContent -match 'class LocalhostOnlyMiddleware') {
        Write-Host "  ✓ LocalhostOnlyMiddleware implementado" -ForegroundColor Green
    } else {
        Write-Host "  ✗ LocalhostOnlyMiddleware no implementado correctamente" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  ✗ Middleware de seguridad no encontrado" -ForegroundColor Red
    exit 1
}

# 4. Verificar que middleware está registrado en main.py
Write-Host "[4/5] Verificando registro de middleware..." -ForegroundColor Yellow
if ($mainContent -match 'LocalhostOnlyMiddleware') {
    Write-Host "  ✓ LocalhostOnlyMiddleware registrado en main.py" -ForegroundColor Green
} else {
    Write-Host "  ✗ LocalhostOnlyMiddleware NO está registrado" -ForegroundColor Red
    exit 1
}

# 5. Verificar CORS origins
Write-Host "[5/5] Verificando configuración de CORS..." -ForegroundColor Yellow
if ($mainContent -match 'localhost|127\.0\.0\.1') {
    Write-Host "  ✓ CORS configurado para localhost" -ForegroundColor Green
} else {
    Write-Host "  ✗ CORS no está configurado correctamente" -ForegroundColor Red
    exit 1
}

if ($mainContent -match 'allow_origins\s*=\s*\["\*"\]') {
    Write-Host "  ✗ ADVERTENCIA: CORS permite wildcard (*)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "=== Verificación Completada ===" -ForegroundColor Cyan
Write-Host "✓ Todas las verificaciones pasaron" -ForegroundColor Green
Write-Host ""
Write-Host "Próximos pasos:" -ForegroundColor Yellow
Write-Host "  1. Iniciar el servidor: .\run.ps1" -ForegroundColor White
Write-Host "  2. Verificar binding: netstat -an | Select-String '8000'" -ForegroundColor White
Write-Host "  3. Ejecutar tests: poetry run pytest tests/test_security.py -v" -ForegroundColor White
Write-Host ""

