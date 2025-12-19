<#
.SYNOPSIS
    Sincroniza y valida todas las etiquetas definidas en project_labels.json con GitHub.
#>

param(
    [switch]$DryRun,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# CONFIGURACIÓN
$projectLabelsPath = ".github/docs/project_labels.json"

Write-Host "`n[Sincronizando Etiquetas de Proyecto]" -ForegroundColor Cyan

# VALIDACIÓN
if (-not (Test-Path $projectLabelsPath)) {
    Write-Host "✗ ERROR: No se encuentra $projectLabelsPath" -ForegroundColor Red
    exit 1
}

# LECTURA DEL JSON (Forzando UTF-8)
Write-Host "Leyendo configuración..." -ForegroundColor Cyan
try {
    # IMPORTANTE: -Encoding UTF8 asegura que las tildes en 'description' se lean bien
    $jsonContent = Get-Content $projectLabelsPath -Raw -Encoding UTF8
    $labelsConfig = $jsonContent | ConvertFrom-Json
    Write-Host "✓ Configuración cargada exitosamente" -ForegroundColor Green
} catch {
    Write-Host "✗ ERROR: JSON inválido" -ForegroundColor Red
    Write-Host "  $($_.Exception.Message)" -ForegroundColor Yellow
    exit 1
}

# VERIFICAR GH CLI
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "✗ ERROR: GitHub CLI (gh) no instalado." -ForegroundColor Red
    exit 1
}

# PROCESAR ETIQUETAS
$allLabels = @()
foreach ($categoryName in $labelsConfig.categories.PSObject.Properties.Name) {
    $category = $labelsConfig.categories.$categoryName
    foreach ($label in $category.labels) {
        $allLabels += $label
    }
}

Write-Host "Procesando $($allLabels.Count) etiquetas..." -ForegroundColor Cyan

# SINCRONIZACIÓN
foreach ($label in $allLabels) {
    $displayName = $label.name
    $desc = $label.description
    $color = $label.color
    
    if ($DryRun) {
        Write-Host "  [DRY] Crear/Update: $displayName ($color)" -ForegroundColor Gray
    } else {
        # Usamos comillas dobles para que PowerShell interpole correctamente
        # gh label create maneja UTF-8 nativamente si la consola lo soporta
        $result = gh label create "$displayName" --color "$color" --description "$desc" --force 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  ✓ $displayName" -ForegroundColor Green
        } else {
            Write-Host "  ✗ $displayName - ERROR" -ForegroundColor Red
            if ($Verbose) { Write-Host $result -ForegroundColor Yellow }
        }
    }
}

Write-Host "`n[Sincronización completada]" -ForegroundColor Cyan