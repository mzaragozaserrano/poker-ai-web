<#
.SYNOPSIS
    Crea issues en GitHub en lote basándose en una lista definida.
    Este script es modificado automáticamente por el Agente de Cursor antes de su ejecución.
    
.NOTES
    IMPORTANTE - CODIFICACIÓN DEL ARCHIVO:
    Este archivo DEBE estar guardado en UTF-8 sin BOM para que los caracteres especiales
    (ñ, á, é, í, ó, ú) se muestren correctamente en GitHub.
    
    Si el archivo está en Windows-1252 u otra codificación, el script intentará corregir
    automáticamente los caracteres corruptos, pero es mejor guardar el archivo en UTF-8.
    
    Sintaxis recomendada para títulos y bodies:
    - Usa Here-String (@"..."@) para facilitar la lectura
    - Puedes escribir tildes y acentos directamente
    - Consulta .github/docs/windows_utf8_setup.md para más detalles
#>

# 1. CARGA LA CONFIGURACIÓN UTF-8
# $PSScriptRoot es la carpeta donde está este script.
# Si el fichero Enable-Utf8.ps1 no existe, ignoramos el error para no romper nada.
if (Test-Path "$PSScriptRoot/Enable-Utf8.ps1") {
    . "$PSScriptRoot/Enable-Utf8.ps1"
}

# --- ZONA EDITABLE POR EL AGENTE ---
# El agente rellenará este array basándose en el Roadmap.
# IMPORTANTE AGENTE: 
# 1. Usa Here-String (@"..."@) para títulos y bodies
# 2. Puedes escribir tildes directamente: ñ, á, é, í, ó, ú
# 3. Consulta .github/docs/labels_convention.md para asignar labels correctamente.
#    Formato recomendado: "tipo,tecnologia,fase-X" (ej: "task,frontend,fase-7")
$issues = @(
    @{ 
        Title = "7.1 Database Schema: Tablas imported_files y player_stats"
        Body = @"
Crear el esquema de base de datos en Neon para el módulo Stats Tracker.

## Tareas
- [ ] Generar migración SQL para crear tabla `imported_files`:
  - `id`: UUID (PK)
  - `user_id`: UUID (FK)
  - `file_hash`: String (MD5 para prevenir duplicados)
  - `hand_count`: Integer
  - `created_at`: Timestamp
- [ ] Generar migración SQL para crear tabla `player_stats`:
  - `id`: UUID (PK)
  - `user_id`: UUID (FK)
  - `format`: String (ej: '6-max', 'HU')
  - `total_hands`: Integer
  - `vpip_numerator`: Integer
  - `pfr_numerator`: Integer
  - `three_bet_numerator`: Integer
  - `three_bet_opportunity`: Integer
  - `won_showdown_numerator`: Integer
  - `went_showdown_numerator`: Integer
  - `positional_stats`: JSONB
  - `updated_at`: Timestamp
- [ ] Crear interfaces TypeScript en `src/types/stats.ts` que reflejen el esquema de DB
- [ ] Definir tipos para estructuras de conteo en memoria (AggregatedStats)

## Criterios de Aceptación
- Las tablas se crean correctamente en Neon con los índices necesarios
- Los tipos TypeScript reflejan fielmente el esquema de base de datos
- El sistema puede prevenir duplicados mediante `file_hash`
"@
        Labels = "task,database,Fase 7" 
    },
    @{ 
        Title = "7.2 Stat Engine: Calculadora de estadísticas con Web Workers"
        Body = @"
Implementar el motor de cálculo de estadísticas (VPIP, PFR, 3Bet, WTSD, W$SD) con procesamiento en Web Workers.

## Tareas
- [ ] Crear `src/lib/stats/statCalculator.ts`:
  - Función `calculateStats(hands: HandHistory[]): AggregatedStats`
  - Implementar lógica de detección de oportunidades para 3-Bet (crucial)
  - Distinguir estrictamente entre Acción y Oportunidad
  - Implementar fórmulas: VPIP, PFR, 3-Bet, WTSD, W$SD
- [ ] Crear `src/workers/parsingWorker.ts`:
  - Importar `WinamaxParser` existente
  - Importar nuevo `statCalculator`
  - Flujo: Recibir archivos de texto -> Parsear -> Calcular -> Devolver Stats Agregadas
- [ ] Manejar casos de borde (ej: walks en BB, all-in forzado antes de 3-bet opportunity)

## Criterios de Aceptación
- El cálculo de estadísticas es correcto según las fórmulas definidas
- El Web Worker procesa archivos sin bloquear la UI principal
- La detección de oportunidades 3-bet funciona correctamente
- Se pueden procesar 50+ archivos simultáneos sin problemas de rendimiento
"@
        Labels = "task,frontend,Fase 7" 
    },
    @{ 
        Title = "7.3 API Integration: Endpoints para stats y verificación de duplicados"
        Body = @"
Implementar endpoints backend y frontend para la integración del módulo Stats Tracker.

## Tareas Backend
- [ ] Crear `POST /api/stats/check-dupes`:
  - Recibe array de hashes MD5 de archivos
  - Devuelve cuáles ya existen en `imported_files`
  - Permite prevenir re-procesamiento
- [ ] Crear `POST /api/stats/upload`:
  - Recibe JSON con stats agregadas
  - Actualiza `player_stats` usando `ON CONFLICT` o lógica de suma para incrementar contadores
  - Maneja actualización de `imported_files` con `file_hash` y `hand_count`

## Tareas Frontend
- [ ] Actualizar `src/lib/apiClient.ts` con nuevos endpoints:
  - `checkDuplicateFiles(hashes: string[]): Promise<string[]>`
  - `uploadStats(stats: AggregatedStats): Promise<void>`
- [ ] Integrar verificación de duplicados antes de procesar archivos

## Criterios de Aceptación
- Los endpoints funcionan correctamente con autenticación
- La verificación de duplicados previene re-procesamiento eficientemente
- La actualización de stats incrementa contadores correctamente
- El frontend maneja errores de red y del servidor apropiadamente
"@
        Labels = "task,backend,Fase 7" 
    },
    @{ 
        Title = "7.4 UI Components: Dashboard con gráficos y detección de leaks"
        Body = @"
Crear componentes UI para visualización de estadísticas y detección de leaks.

## Tareas
- [ ] Crear `src/config/optimal-ranges.ts`:
  - Definir umbrales GTO/Reg para 6-max NLHE
  - Estructura: VPIP, PFR, ThreeBet, WTSD, W$SD con min/max
- [ ] Crear `src/components/stats/RangeBandChart.tsx`:
  - Usar Recharts (`ComposedChart`)
  - Implementar `Area` para rango min/max (Success Green con opacidad 20%)
  - Implementar `Line` para métricas del usuario (Primary Violet)
  - Lógica de `dot` personalizado: Verde si está dentro de banda, Rojo si está fuera
- [ ] Crear componente `StatCard`:
  - Mostrar KPIs individuales (VPIP, PFR, etc.)
  - Colores condicionales según `ui-foundations.md`
  - Badge de "Leak" si está fuera de rango óptimo
- [ ] Crear componente de Dropzone para subida masiva con feedback de progreso

## Criterios de Aceptación
- Los gráficos muestran correctamente las bandas óptimas vs métricas del usuario
- Los colores siguen la paleta definida en `ui-foundations.md`
- Los badges de "Leak" aparecen cuando las métricas están fuera de rango
- El Dropzone muestra progreso durante el procesamiento de archivos
"@
        Labels = "task,frontend,Fase 7" 
    },
    @{ 
        Title = "7.5 StatsPage: Dashboard completo con integración"
        Body = @"
Crear la página principal del módulo Stats Tracker con integración completa.

## Tareas
- [ ] Crear `src/pages/StatsPage.tsx`:
  - Layout con Upload Zone (Dropzone grande con feedback de progreso)
  - Stats Grid con tarjetas de KPIs (VPIP, PFR, 3Bet, WTSD, W$SD)
  - Charts Section con gráficos de "Corridor of Success"
  - Gráfico de línea de Net Won (Currency)
  - Gráfico de bandas para VPIP por posición
- [ ] Integrar Dropzone conectado al Web Worker:
  - Manejar múltiples archivos simultáneamente
  - Mostrar progreso de procesamiento
  - Verificar duplicados antes de procesar
- [ ] Usar TanStack Query para fetch de stats:
  - Hook `usePlayerStats()` para obtener estadísticas del usuario
  - Invalidación automática tras subida de nuevos archivos
- [ ] Renderizar gráficos y tarjetas con datos reales

## Criterios de Aceptación
- La página carga y muestra estadísticas correctamente
- El procesamiento de archivos funciona sin bloquear la UI
- Los gráficos se actualizan automáticamente tras nuevas subidas
- La experiencia de usuario es fluida y clara
- Se manejan correctamente estados de carga y error
"@
        Labels = "task,frontend,Fase 7" 
    },
    @{ 
        Title = "7.6 Testing & Validation: Unit tests y pruebas de rendimiento"
        Body = @"
Implementar tests unitarios y validar rendimiento del módulo Stats Tracker.

## Tareas Testing
- [ ] Crear tests unitarios para `statCalculator.ts`:
  - Verificar cálculo correcto de PFR en casos de borde (ej: walks en BB)
  - Verificar cálculo correcto de 3-Bet con detección de oportunidades
  - Validar fórmulas de VPIP, WTSD, W$SD
- [ ] Crear tests para `parsingWorker.ts`:
  - Verificar procesamiento de archivos Winamax
  - Validar que no bloquea la UI principal

## Tareas Performance
- [ ] Probar subida de 50+ archivos simultáneos:
  - Verificar que el Web Worker no bloquea la UI principal
  - Medir tiempo de procesamiento
  - Validar uso de memoria
- [ ] Optimizar si es necesario:
  - Chunking de archivos grandes
  - Límites de procesamiento simultáneo

## Criterios de Aceptación
- Todos los tests unitarios pasan correctamente
- El procesamiento de 50+ archivos no bloquea la UI
- Los tiempos de procesamiento son aceptables (< 30s para 50 archivos)
- No hay memory leaks durante el procesamiento masivo
"@
        Labels = "task,testing,Fase 7" 
    }
)
# -----------------------------------

Write-Host "`n[Iniciando creación de lote de issues...]" -ForegroundColor Cyan

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

# Verificación de seguridad básica
if ($issues.Count -eq 0) {
    Write-Warning "La lista de issues está vacía. No hay nada que crear."
    exit
}

foreach ($issue in $issues) {
    # Mostramos en pantalla el título
    Write-Host "Creando: $($issue.Title)..." -NoNewline
    
    # Ejecutamos el comando de GitHub CLI
    # CRÍTICO: Usar archivo temporal para el body con codificación UTF-8 explícita
    # Esto asegura que los caracteres especiales (ñ, á, é, í, ó, ú) se preserven correctamente
    # No usar comillas adicionales alrededor de las variables para el título y labels
    $title = $issue.Title
    $body = $issue.Body
    $labels = $issue.Labels
    
    # Crear archivo temporal con codificación UTF-8 sin BOM para el body
    # CRÍTICO: Corregir la codificación del string si está corrupto antes de escribirlo
    # Esto maneja el caso donde el archivo .ps1 no está guardado en UTF-8
    $tempBodyFile = [System.IO.Path]::GetTempFileName()
    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    
    # Intentar corregir la codificación del body si está corrupto
    $correctedBody = Fix-StringEncoding -InputString $body
    
    # Usar StreamWriter para escribir con UTF-8 sin BOM
    $streamWriter = New-Object System.IO.StreamWriter($tempBodyFile, $false, $utf8NoBom)
    try {
        $streamWriter.Write($correctedBody)
    } finally {
        $streamWriter.Close()
    }
    
    try {
        # Crear issue usando --body-file para asegurar UTF-8 correcto
        $result = gh issue create --title $title --body-file $tempBodyFile --label $labels 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host " OK" -ForegroundColor Green
        } else {
            Write-Host " ERROR" -ForegroundColor Red
            Write-Host $result -ForegroundColor Yellow
        }
    } finally {
        # Limpiar archivo temporal
        Remove-Item $tempBodyFile -Force -ErrorAction SilentlyContinue
    }
    
    # Pequeña pausa de 500ms
    Start-Sleep -Milliseconds 500 
}

Write-Host "`n[Proceso finalizado.]" -ForegroundColor Cyan
