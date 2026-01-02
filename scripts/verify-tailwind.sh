#!/usr/bin/env bash

# Verificación de clases Tailwind CSS disponibles
# Script para validar que todas las clases definidas en tailwind.config.js están disponibles

echo "Verificando disponibilidad de clases Tailwind CSS..."
echo "=================================================="

TAILWIND_CONFIG="frontend/tailwind.config.js"
CSS_OUTPUT="frontend/dist/index.css"

# Verificar que el archivo de configuración existe
if [ ! -f "$TAILWIND_CONFIG" ]; then
  echo "ERROR: No se encontró $TAILWIND_CONFIG"
  exit 1
fi

echo "✓ Configuración de Tailwind encontrada"

# Clases a verificar
CLASSES_TO_CHECK=(
  "bg-slate-950"
  "bg-slate-800"
  "bg-slate-700"
  "bg-poker-raise"
  "bg-poker-call"
  "bg-poker-fold"
  "bg-poker-equity-high"
  "bg-accent-violet"
  "text-slate-200"
  "text-poker-raise"
  "text-poker-call"
  "border-slate-700"
  "btn"
  "btn-primary"
  "card"
  "badge-raise"
  "badge-call"
  "badge-fold"
  "badge-equity"
)

echo ""
echo "Verificando clases disponibles..."
echo ""

# Verificar presencia de clases en tailwind.config.js
for class in "${CLASSES_TO_CHECK[@]}"; do
  # Convertir clase a patrón de búsqueda
  if grep -q "$class" "$TAILWIND_CONFIG" frontend/src/index.css 2>/dev/null; then
    echo "✓ $class"
  else
    echo "✗ $class (NO ENCONTRADA)"
  fi
done

echo ""
echo "Verificación completada."
echo ""
echo "Próximo paso: ejecutar 'npm run build' para compilar Tailwind CSS"

