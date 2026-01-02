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
# FASE 4: Optimización, Seguridad y Lanzamiento
$issues = @(
    @{ 
        Title = @"
4.1.1 Implementar generador de manos sintéticas para pruebas de carga
"@
        Body = @"
Crear un generador de manos sintéticas en Rust para poblar la base de datos con millones de manos de prueba.

## Contexto
- Fase 4.1: Rendimiento y Escalabilidad
- Necesario para pruebas de carga con 10M de manos
- Debe generar datos realistas y válidos

## Tareas
- [ ] Crear módulo backend/parsers/src/synthetic_generator.rs
- [ ] Implementar generación de manos con distribución realista
- [ ] Generar acciones válidas (preflop, flop, turn, river)
- [ ] Generar stacks y apuestas coherentes con el stake
- [ ] Usar Rayon para generación paralela (16 threads)
- [ ] Implementar CLI para generar N manos
- [ ] Añadir semilla para reproducibilidad
- [ ] Integrar con Parquet writer para persistencia

## Criterios de Aceptación
- Genera 1M manos en < 60 segundos
- Datos cumplen con el esquema de DuckDB
- Distribución de posiciones y acciones realista
- Generación determinista con semilla

## Parámetros de Generación
- Número de manos
- Rango de fechas
- Stakes (NL2, NL5, NL10, NL25, NL50)
- Seed para reproducibilidad
"@
        Labels = "enhancement,backend,fase-4,testing" 
    },
    @{ 
        Title = @"
4.1.2 Ejecutar pruebas de carga masiva con 10M de manos
"@
        Body = @"
Ejecutar y documentar pruebas de carga con una base de datos de 10 millones de manos.

## Contexto
- Fase 4.1: Rendimiento y Escalabilidad
- Hardware: Ryzen 7 3800X (16 threads) + 64GB RAM
- Medir rendimiento de consultas y API

## Tareas
- [ ] Generar dataset de 10M manos con generador sintético
- [ ] Medir tiempo de carga inicial en DuckDB
- [ ] Benchmark de consultas típicas:
  - [ ] Estadísticas por jugador (VPIP, PFR, 3Bet)
  - [ ] Filtrado por fecha y stake
  - [ ] Agregaciones por posición
- [ ] Medir latencia de endpoints API
- [ ] Medir uso de memoria durante consultas
- [ ] Identificar cuellos de botella
- [ ] Documentar resultados en docs/specs/performance-benchmark.md

## Criterios de Aceptación
- Carga de 10M manos en < 5 minutos
- Consultas de stats < 500ms
- Uso de RAM < 32GB durante consultas
- Sin OOM (Out of Memory) errors

## Métricas a Capturar
- Tiempo de ingesta (manos/segundo)
- Latencia p50, p95, p99 de queries
- Memoria pico durante operaciones
- Throughput de API (requests/segundo)
"@
        Labels = "enhancement,backend,fase-4,testing,database" 
    },
    @{ 
        Title = @"
4.1.3 Configurar Huge Pages para optimización de memoria
"@
        Body = @"
Configurar el sistema operativo para usar Huge Pages y optimizar el rendimiento de memoria.

## Contexto
- Fase 4.1: Rendimiento y Escalabilidad
- 64GB RAM disponibles
- Huge Pages reducen TLB misses

## Tareas
- [ ] Documentar configuración de Huge Pages en Windows/Linux
- [ ] Crear script de configuración para Windows (PowerShell)
- [ ] Crear script de configuración para Linux (bash)
- [ ] Configurar DuckDB para usar Huge Pages
- [ ] Benchmark comparativo (con/sin Huge Pages)
- [ ] Documentar impacto en rendimiento
- [ ] Añadir instrucciones en docs/specs/sys-ops.md

## Criterios de Aceptación
- Scripts de configuración funcionan
- Mejora medible en benchmarks (>10%)
- Documentación clara para el usuario
- Configuración reversible

## Configuración Recomendada
- Huge Pages: 1GB pages si disponible, sino 2MB
- Reservar 32GB para Huge Pages (deja 32GB para SO)
- Deshabilitar swap o configurar swappiness=10
"@
        Labels = "enhancement,devops,fase-4,performance" 
    },
    @{ 
        Title = @"
4.1.4 Tuning de DuckDB para consultas vectorizadas masivas
"@
        Body = @"
Optimizar la configuración de DuckDB para máximo rendimiento en consultas analíticas.

## Contexto
- Fase 4.1: Rendimiento y Escalabilidad
- DuckDB opera in-memory
- 64GB RAM, 16 threads disponibles

## Tareas
- [ ] Configurar threads = 16 (Ryzen 3800X)
- [ ] Ajustar memory_limit para aprovechar RAM
- [ ] Configurar preserve_insertion_order = false
- [ ] Optimizar checkpoint_threshold
- [ ] Evaluar configuración de compression para Parquet
- [ ] Crear índices para columnas de filtrado frecuente
- [ ] Implementar particionamiento por fecha
- [ ] Benchmark antes/después de tuning
- [ ] Documentar configuración óptima

## Criterios de Aceptación
- Consultas 2x más rápidas post-tuning
- Uso eficiente de los 16 threads
- Sin degradación en inserciones
- Configuración documentada

## Configuraciones a Evaluar
- threads: 16
- memory_limit: 48GB
- temp_directory: /path/to/ssd
- checkpoint_threshold: 256MB
- enable_progress_bar: false (producción)
"@
        Labels = "enhancement,database,fase-4,performance" 
    },
    @{ 
        Title = @"
4.2.1 Verificar y asegurar que API escucha solo en localhost
"@
        Body = @"
Verificar y reforzar que la API REST y WebSocket solo escuchan en 127.0.0.1.

## Contexto
- Fase 4.2: Cumplimiento y Seguridad
- Privacidad: Datos nunca deben salir de localhost
- Requisito crítico del proyecto

## Tareas
- [ ] Auditar configuración de Uvicorn en server-api
- [ ] Verificar bind a 127.0.0.1 (no 0.0.0.0)
- [ ] Añadir tests de seguridad que verifican binding
- [ ] Configurar CORS para solo aceptar localhost origins
- [ ] Bloquear headers de forwarding (X-Forwarded-For)
- [ ] Documentar configuración de seguridad
- [ ] Añadir warning si se detecta intento de bind a 0.0.0.0
- [ ] Crear checklist de seguridad en docs/specs/security.md

## Criterios de Aceptación
- API rechaza conexiones no-localhost
- Tests automáticos verifican binding
- CORS configurado restrictivamente
- Documentación de seguridad completa

## Verificaciones
- netstat/ss muestra bind a 127.0.0.1:8000
- Conexión desde otra IP es rechazada
- Headers de proxy no son honrados
"@
        Labels = "enhancement,backend,fase-4,security" 
    },
    @{ 
        Title = @"
4.2.2 Implementar sistema de auditoría de logs
"@
        Body = @"
Implementar sistema de logging estructurado para auditoría y debugging.

## Contexto
- Fase 4.2: Cumplimiento y Seguridad
- Asegurar trazabilidad de operaciones
- No debe haber interacción con proceso de Winamax

## Tareas
- [ ] Configurar logging estructurado en Python (structlog)
- [ ] Configurar logging en Rust (tracing crate)
- [ ] Definir niveles de log (ERROR, WARN, INFO, DEBUG)
- [ ] Implementar rotación de logs (max 100MB por archivo)
- [ ] Añadir contexto a logs (request_id, timestamp, user)
- [ ] Crear logs de auditoría para:
  - [ ] Acceso a endpoints
  - [ ] Operaciones de base de datos
  - [ ] Detección de archivos nuevos
- [ ] Almacenar logs en directorio local (logs/)
- [ ] NO loguear contenido de manos (solo metadatos)

## Criterios de Aceptación
- Logs estructurados en JSON
- Rotación automática funciona
- No hay PII en logs
- Logs de auditoría separados de logs de debug

## Formato de Log
- timestamp: ISO 8601
- level: ERROR/WARN/INFO/DEBUG
- component: api/parser/db
- message: descripción
- context: { request_id, duration_ms, ... }
"@
        Labels = "enhancement,backend,fase-4,security,devops" 
    },
    @{ 
        Title = @"
4.2.3 Crear empaquetado de aplicación como ejecutable local
"@
        Body = @"
Empaquetar la aplicación completa como un ejecutable local simplificado.

## Contexto
- Fase 4.2: Cumplimiento y Seguridad
- Objetivo: Instalación sin dependencias externas
- Soporte Windows (prioridad) y Linux

## Tareas
- [ ] Crear script de build para backend Rust (release optimizado)
- [ ] Compilar poker-ffi como wheel de Python
- [ ] Bundlear frontend con Vite (npm run build)
- [ ] Crear script de empaquetado (PowerShell para Windows)
- [ ] Incluir DuckDB embebido
- [ ] Crear launcher script (inicia backend + abre browser)
- [ ] Generar instalador o ZIP autocontenido
- [ ] Documentar proceso de build en README
- [ ] Crear GitHub Actions para builds automáticos

## Criterios de Aceptación
- Instalación en < 5 minutos
- No requiere Python/Node instalado por usuario
- Funciona offline después de instalación
- Tamaño total < 500MB

## Estructura del Paquete
poker-analyzer/
  - poker-analyzer.exe (launcher)
  - backend/ (binarios Rust + Python embebido)
  - frontend/ (archivos estáticos)
  - data/ (directorio para Parquet)
  - logs/
  - config.toml
"@
        Labels = "enhancement,devops,fase-4" 
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
