<#
.SYNOPSIS
    Script de prueba para verificar que las issues se crean correctamente con caracteres UTF-8.
    Esta issue será eliminada después de la verificación.
#>

# Cargar configuración UTF-8
$PSScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
if (Test-Path "$PSScriptRoot/Enable-Utf8.ps1") {
    . "$PSScriptRoot/Enable-Utf8.ps1"
}

Write-Host "`n[Creando issue de prueba con caracteres especiales...]" -ForegroundColor Cyan

# Issue de prueba con múltiples caracteres especiales
# Usamos Here-String con tildes directas (como recomienda la documentación)
# Si el archivo .ps1 está guardado en UTF-8, esto debería funcionar correctamente
$testIssue = @{
    Title = "Prueba UTF-8: Validaci$([char]0x00F3)n de caracteres especiales ($([char]0x00F1), $([char]0x00E1), $([char]0x00E9), $([char]0x00ED), $([char]0x00F3), $([char]0x00FA))"
    Body = @"
Esta es una issue de prueba para validar que los caracteres especiales se muestran correctamente.

## Caracteres a probar:
- **Minúsculas:** ñ, á, é, í, ó, ú
- **Mayúsculas:** Ñ, Á, É, Í, Ó, Ú

## Palabras de ejemplo:
- Configuración
- Implementación
- Validación
- Estadísticas
- Gráficos
- Precisión

## Frase completa:
Esta es una prueba de validación para verificar que la codificación UTF-8 funciona correctamente en los títulos y descripciones de las issues de GitHub.

**Esta issue será eliminada después de la verificación.**
"@
    Labels = "documentation"
}

# Función para corregir la codificación del string si está corrupto
# Detecta si el string tiene caracteres corruptos (como "Ã±" en lugar de "ñ")
# y los corrige asumiendo que fueron leídos como Windows-1252 pero deberían ser UTF-8
function Fix-StringEncoding {
    param([string]$InputString)
    
    # Si el string contiene patrones de corrupción típicos (Windows-1252 mal interpretado como UTF-8)
    # Convertimos de Windows-1252 a UTF-8
    $windows1252 = [System.Text.Encoding]::GetEncoding(1252)
    $utf8 = New-Object System.Text.UTF8Encoding $false
    
    # Convertir el string a bytes usando Windows-1252 (asumiendo que fue leído incorrectamente)
    $bytes = $windows1252.GetBytes($InputString)
    # Convertir esos bytes a string usando UTF-8
    $utf8.GetString($bytes)
}

Write-Host "Título: $($testIssue.Title)" -ForegroundColor Yellow
Write-Host "Creando issue..." -NoNewline

# Crear archivo temporal con codificación UTF-8 sin BOM para el body
# CRÍTICO: Corregir la codificación del string si está corrupto antes de escribirlo
$tempBodyFile = [System.IO.Path]::GetTempFileName()
$utf8NoBom = New-Object System.Text.UTF8Encoding $false

# Intentar corregir la codificación del body si está corrupto
# Esto maneja el caso donde el archivo .ps1 no está guardado en UTF-8
$correctedBody = Fix-StringEncoding -InputString $testIssue.Body

# Usar StreamWriter para escribir con UTF-8 sin BOM
$streamWriter = New-Object System.IO.StreamWriter($tempBodyFile, $false, $utf8NoBom)
try {
    $streamWriter.Write($correctedBody)
} finally {
    $streamWriter.Close()
}

try {
    # Crear la issue usando --body-file para asegurar UTF-8 correcto
    # Título sin comillas adicionales, body desde archivo UTF-8
    $result = gh issue create --title $testIssue.Title --body-file $tempBodyFile --label $testIssue.Labels 2>&1
} finally {
    # Limpiar archivo temporal
    Remove-Item $tempBodyFile -Force -ErrorAction SilentlyContinue
}

if ($LASTEXITCODE -eq 0) {
    Write-Host " OK" -ForegroundColor Green
    # Extraer el número de issue de la salida
    if ($result -match 'https://github\.com/[^/]+/[^/]+/issues/(\d+)') {
        $issueNumber = $matches[1]
        Write-Host "`nIssue creada exitosamente: #$issueNumber" -ForegroundColor Green
        Write-Host "URL: $result" -ForegroundColor Cyan
        Write-Host "`nPor favor, verifica en GitHub que los caracteres especiales se muestran correctamente." -ForegroundColor Yellow
        Write-Host "Para eliminar esta issue, ejecuta: gh issue close $issueNumber" -ForegroundColor Gray
    } else {
        Write-Host "`nIssue creada, pero no se pudo extraer el número." -ForegroundColor Yellow
        Write-Host "Salida: $result" -ForegroundColor Gray
    }
} else {
    Write-Host " ERROR" -ForegroundColor Red
    Write-Host $result -ForegroundColor Yellow
    exit 1
}

Write-Host "`n[Prueba completada.]" -ForegroundColor Cyan

