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
2.1.1 Implementar algoritmo de evaluación de manos en Rust
"@
        Body = @"
Implementar el algoritmo de evaluación de manos de poker (Cactus Kev o variante OMPEval) en Rust.

## Contexto
- Fase 2.1: Motor de Evaluación de Manos (Rust)
- Hardware objetivo: Ryzen 7 3800X (16 threads) + 64GB RAM
- Debe ser la base para el simulador Monte Carlo

## Tareas
- [ ] Investigar e implementar algoritmo Cactus Kev o OMPEval
- [ ] Crear módulo `hand_evaluator` en el workspace de Rust
- [ ] Implementar función para evaluar fuerza de mano de 5-7 cartas
- [ ] Crear tests unitarios con casos conocidos (Royal Flush, Straight, etc.)
- [ ] Benchmarks de rendimiento (objetivo: < 100ns por evaluación)

## Criterios de Aceptación
- El evaluador retorna correctamente el ranking de cualquier combinación de 5-7 cartas
- Tests pasan al 100%
- Performance < 100ns por evaluación en hardware objetivo

## Referencias
- Cactus Kev: https://suffe.cool/poker/evaluator.html
- OMPEval: https://github.com/zekyll/OMPEval
"@
        Labels = "enhancement,rust,fase-2" 
    },
    @{ 
        Title = @"
2.1.2 Pre-calcular Perfect Hash Table de 7 cartas
"@
        Body = @"
Generar y cargar en RAM la tabla hash perfecta para evaluación instantánea de manos de 7 cartas.

## Contexto
- Fase 2.1: Motor de Evaluación de Manos
- Aprox. 133 millones de combinaciones (C(52,7))
- Debe cargarse en memoria al inicio aprovechando los 64GB de RAM

## Tareas
- [ ] Implementar generador de la tabla hash perfecta
- [ ] Calcular todas las combinaciones C(52,7) y sus rankings
- [ ] Serializar tabla a formato binario compacto
- [ ] Implementar cargador lazy_static para inicialización única
- [ ] Verificar búsquedas O(1) con benchmarks

## Criterios de Aceptación
- Tabla se genera correctamente (puede tardar minutos, se hace una vez)
- Carga en RAM < 5 segundos al inicio de la aplicación
- Búsquedas son O(1) y < 50ns
- Tamaño en disco < 500MB (comprimido)

## Notas Técnicas
- Usar `lazy_static` o `once_cell` para inicialización única
- Considerar `bincode` o formato custom para serialización eficiente
"@
        Labels = "enhancement,rust,fase-2" 
    },
    @{ 
        Title = @"
2.1.3 Implementar simulador Monte Carlo con SIMD AVX2
"@
        Body = @"
Desarrollar el simulador Monte Carlo para cálculo de equities utilizando intrínsecos SIMD AVX2 del Ryzen 3800X.

## Contexto
- Fase 2.1: Motor de Evaluación de Manos
- Debe aprovechar AVX2 para paralelizar evaluaciones
- Integración con Rayon para multi-threading (16 hilos)

## Tareas
- [ ] Crear módulo `equity_calculator` en Rust
- [ ] Implementar simulación Monte Carlo básica
- [ ] Optimizar con intrínsecos SIMD AVX2 (`std::arch::x86_64`)
- [ ] Integrar con Rayon para paralelización en 16 threads
- [ ] Implementar early stopping cuando convergencia < 0.1%
- [ ] Benchmarks de rendimiento (objetivo: 100K sims/segundo)

## Criterios de Aceptación
- Calcula equity correctamente para escenarios conocidos (AA vs KK preflop ≈ 82%)
- Utiliza AVX2 verificable con profiling
- Escala linealmente hasta 16 threads
- Performance > 100K simulaciones/segundo en hardware objetivo

## Referencias Técnicas
- Rust SIMD: https://doc.rust-lang.org/std/arch/
- Rayon parallel iterators
"@
        Labels = "enhancement,rust,fase-2,performance" 
    },
    @{ 
        Title = @"
2.2.1 Configurar entorno Python con Poetry y PyO3
"@
        Body = @"
Configurar el entorno de desarrollo Python para FastAPI y el puente FFI con Rust.

## Contexto
- Fase 2.2: Orquestación y API
- Pendiente desde Fase 1.1
- Base para la capa de servicio

## Tareas
- [ ] Inicializar proyecto Python con Poetry en `server-api/`
- [ ] Configurar dependencias: FastAPI, Uvicorn, PyO3/maturin
- [ ] Crear estructura de carpetas (app/, tests/, etc.)
- [ ] Configurar pyproject.toml con versiones específicas
- [ ] Documentar setup en README del servidor

## Criterios de Aceptación
- `poetry install` funciona sin errores
- Python 3.11+ configurado correctamente
- Estructura de proyecto lista para desarrollo
- Documentación clara del setup

## Dependencias Clave
- fastapi >= 0.104.0
- uvicorn[standard] >= 0.24.0
- pydantic >= 2.0
- maturin >= 1.3.0 (para PyO3)
"@
        Labels = "task,python,fase-2" 
    },
    @{ 
        Title = @"
2.2.2 Crear puente FFI Rust-Python con PyO3
"@
        Body = @"
Desarrollar el puente FFI para exponer funciones de Rust a Python sin sobrecarga de serialización.

## Contexto
- Fase 2.2: Orquestación y API
- Permite llamar al parser y motor de evaluación desde FastAPI
- Zero-copy cuando sea posible

## Tareas
- [ ] Crear crate `poker-ffi` en workspace de Rust
- [ ] Configurar Cargo.toml con PyO3 y crate-type = ["cdylib"]
- [ ] Exponer función de parsing de archivos Winamax
- [ ] Exponer función de cálculo de equity
- [ ] Exponer función de consulta a DuckDB
- [ ] Crear módulo Python de ejemplo para testing
- [ ] Documentar contrato FFI en `docs/specs/ffi-contract.md`

## Criterios de Aceptación
- Las funciones Rust son llamables desde Python
- Overhead de FFI < 1ms para operaciones típicas
- Manejo correcto de errores entre lenguajes
- Tests de integración Python-Rust pasan

## Referencias
- PyO3 Guide: https://pyo3.rs/
- Maturin: https://github.com/PyO3/maturin
"@
        Labels = "enhancement,rust,python,fase-2" 
    },
    @{ 
        Title = @"
2.2.3 Implementar endpoints REST en FastAPI para estadísticas
"@
        Body = @"
Desarrollar los endpoints de la API REST para consultas de estadísticas y datos históricos.

## Contexto
- Fase 2.2: Orquestación y API
- Entry point: `server-api/app/main.py`
- Debe consumir funciones Rust vía FFI

## Tareas
- [ ] Crear app FastAPI base con configuración CORS (localhost only)
- [ ] Implementar endpoint GET /stats/player/{name} (VPIP, PFR, 3Bet)
- [ ] Implementar endpoint GET /hands/recent (últimas N manos)
- [ ] Implementar endpoint GET /hands/{hand_id} (detalle de mano)
- [ ] Implementar endpoint POST /equity/calculate (cálculo Monte Carlo)
- [ ] Agregar validación con Pydantic models
- [ ] Documentación automática con OpenAPI/Swagger
- [ ] Tests de integración con pytest

## Criterios de Aceptación
- Todos los endpoints responden correctamente
- Validación de inputs funciona
- Swagger UI accesible en /docs
- Tests de integración pasan
- API escucha SOLO en 127.0.0.1 (seguridad)

## Endpoints Mínimos
- GET /health
- GET /stats/player/{name}
- GET /hands/recent?limit=50
- GET /hands/{hand_id}
- POST /equity/calculate
"@
        Labels = "enhancement,python,backend,fase-2" 
    },
    @{ 
        Title = @"
2.2.4 Implementar WebSocket para push de nuevas manos
"@
        Body = @"
Desarrollar sistema WebSocket para notificar al frontend cuando se detectan nuevas manos.

## Contexto
- Fase 2.2: Orquestación y API
- El file watcher de Rust debe notificar a Python
- Python debe pushear a clientes conectados vía WebSocket

## Tareas
- [ ] Implementar endpoint WebSocket en FastAPI (/ws)
- [ ] Crear sistema de pub/sub interno para eventos de nuevas manos
- [ ] Integrar con el file watcher de Rust (vía FFI o canal)
- [ ] Implementar heartbeat para mantener conexiones vivas
- [ ] Manejar reconexión automática del cliente
- [ ] Tests de integración con cliente WebSocket de prueba

## Criterios de Aceptación
- Cliente recibe notificación < 500ms después de nueva mano
- Múltiples clientes pueden conectarse simultáneamente
- Reconexión automática funciona correctamente
- No hay memory leaks en conexiones largas

## Formato de Mensaje
Ejemplo JSON:
{ type: new_hand, hand_id: ..., timestamp: ..., hero_result: +5.50 }
"@
        Labels = "enhancement,python,backend,fase-2,websocket" 
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