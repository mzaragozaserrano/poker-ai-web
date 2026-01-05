#!/bin/bash
# Security Verification Script
# Verifica que la API está configurada correctamente para localhost-only

set -e

echo "=== Poker AI Security Verification ==="
echo ""

# 1. Verificar configuración de settings
echo "[1/5] Verificando configuración..."
SETTINGS_FILE="$(dirname "$0")/../app/config/settings.py"

if grep -q 'api_host: str = "127.0.0.1"' "$SETTINGS_FILE"; then
    echo "  ✓ api_host configurado a 127.0.0.1"
else
    echo "  ✗ api_host NO está configurado correctamente"
    exit 1
fi

# 2. Verificar que no hay 0.0.0.0 en el código
echo "[2/5] Verificando que no hay binding a 0.0.0.0..."
MAIN_FILE="$(dirname "$0")/../app/main.py"

if grep -q '0.0.0.0' "$MAIN_FILE"; then
    echo "  ✗ ADVERTENCIA: Se encontró 0.0.0.0 en main.py"
    exit 1
else
    echo "  ✓ No se encontró 0.0.0.0 en el código"
fi

# 3. Verificar que LocalhostOnlyMiddleware existe
echo "[3/5] Verificando middleware de seguridad..."
MIDDLEWARE_FILE="$(dirname "$0")/../app/middleware/security.py"

if [ -f "$MIDDLEWARE_FILE" ]; then
    echo "  ✓ LocalhostOnlyMiddleware existe"
    
    if grep -q 'class LocalhostOnlyMiddleware' "$MIDDLEWARE_FILE"; then
        echo "  ✓ LocalhostOnlyMiddleware implementado"
    else
        echo "  ✗ LocalhostOnlyMiddleware no implementado correctamente"
        exit 1
    fi
else
    echo "  ✗ Middleware de seguridad no encontrado"
    exit 1
fi

# 4. Verificar que middleware está registrado en main.py
echo "[4/5] Verificando registro de middleware..."
if grep -q 'LocalhostOnlyMiddleware' "$MAIN_FILE"; then
    echo "  ✓ LocalhostOnlyMiddleware registrado en main.py"
else
    echo "  ✗ LocalhostOnlyMiddleware NO está registrado"
    exit 1
fi

# 5. Verificar CORS origins
echo "[5/5] Verificando configuración de CORS..."
if grep -q -E 'localhost|127\.0\.0\.1' "$MAIN_FILE"; then
    echo "  ✓ CORS configurado para localhost"
else
    echo "  ✗ CORS no está configurado correctamente"
    exit 1
fi

if grep -q 'allow_origins.*\["\*"\]' "$MAIN_FILE"; then
    echo "  ✗ ADVERTENCIA: CORS permite wildcard (*)"
    exit 1
fi

echo ""
echo "=== Verificación Completada ==="
echo "✓ Todas las verificaciones pasaron"
echo ""
echo "Próximos pasos:"
echo "  1. Iniciar el servidor: poetry run python -m app.main"
echo "  2. Verificar binding: ss -tlnp | grep 8000"
echo "  3. Ejecutar tests: poetry run pytest tests/test_security.py -v"
echo ""

