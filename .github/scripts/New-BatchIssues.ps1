<#
.SYNOPSIS
    Crea issues en GitHub en lote basándose en una lista definida.
    Este script es modificado automáticamente por el Agente de Cursor antes de su ejecución.
    
.NOTES
    IMPORTANTE:
    - Usa Here-Strings (@"..."@) para Títulos y Bodies.
    - Las tildes y caracteres especiales (ñ, á, é) se manejan nativamente gracias a la configuración del entorno.
#>

# Configurar salida de consola a UTF-8 por si acaso
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# --- ZONA EDITABLE POR EL AGENTE ---
# El agente rellenará este array basándose en el Roadmap.
# INSTRUCCIONES PARA EL AGENTE:
# 1. Usa SIEMPRE Here-Strings (@"..."@) para Title y Body.
# 2. Escribe tildes y ñ directamente (UTF-8). No uses códigos de escape.
# 3. Formato labels: "tipo,tecnologia,fase-X"
$issues = @(
    @{ 
        Title = @"
7.1 Database Schema: Tablas imported_files y player_stats
"@
        Body = @"
Crear el esquema de base de datos en Neon para el módulo Stats Tracker.

## Tareas
- [ ] Generar migración SQL para crear tabla `imported_files`
- [ ] Generar migración SQL para crear tabla `player_stats`

## Criterios de Aceptación
- Las tablas se crean correctamente en Neon
"@
        Labels = "task,database,Fase 7" 
    }
)
# -----------------------------------

Write-Host "`n[Iniciando creación de lote de issues...]" -ForegroundColor Cyan

if ($issues.Count -eq 0) {
    Write-Warning "La lista de issues está vacía."
    exit
}

foreach ($issue in $issues) {
    Write-Host "Creando: $($issue.Title)..." -NoNewline
    
    # Extraer valores
    $title = $issue.Title
    $body = $issue.Body
    $labels = $issue.Labels
    
    # Ejecutar gh issue create usando las variables directamente
    # PowerShell pasa el contenido de las variables (UTF-8) correctamente a gh cli
    $result = gh issue create --title "$title" --body "$body" --label "$labels" 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host " OK" -ForegroundColor Green
    } else {
        Write-Host " ERROR" -ForegroundColor Red
        Write-Host $result -ForegroundColor Yellow
    }
    
    Start-Sleep -Milliseconds 500 
}

Write-Host "`n[Proceso finalizado.]" -ForegroundColor Cyan