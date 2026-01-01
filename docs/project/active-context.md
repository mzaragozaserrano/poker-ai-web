# FASE 2 EN PROGRESO - Orquestación y API

## Estado General
La Fase 2 (Motor Matemático y Capa de Servicio) continúa. Puente FFI Rust-Python completado.

## Tarea Actual: ISSUE #28
2.2.4 Implementar WebSocket para push de nuevas manos

## Estado: COMPLETADO

## Contexto
- Fase 2.2: Orquestación y API
- Sistema de notificación en tiempo real para nuevas manos detectadas
- Integración completa: Rust File Watcher → Python FFI → WebSocket Manager → Clientes

## Tareas Completadas
- [x] Crear modelos Pydantic para mensajes WebSocket
- [x] Implementar WebSocketManager con pub/sub interno
- [x] Implementar endpoint WebSocket /ws en FastAPI
- [x] Implementar heartbeat automático (30 segundos)
- [x] Extender FFI de Rust para exponer file watcher con callback Python
- [x] Crear FileWatcherService para integrar Rust watcher con WebSocket
- [x] Integrar file watcher en lifecycle de FastAPI (startup/shutdown)
- [x] Implementar manejo de múltiples clientes simultáneos
- [x] Tests de integración WebSocket completos

## Componentes Implementados

### 1. Modelos WebSocket (`app/models/websocket.py`)
- `NewHandMessage`: Notificación de nueva mano detectada
- `HeartbeatMessage`: Keepalive para mantener conexión
- `ConnectionAckMessage`: Confirmación de conexión con client_id
- `ErrorMessage`: Mensajes de error

### 2. WebSocket Manager (`app/services/websocket_manager.py`)
- Gestor centralizado de conexiones WebSocket
- Soporte para múltiples clientes simultáneos
- Sistema de pub/sub interno con asyncio.Queue
- Heartbeat automático cada 30 segundos
- Broadcasting de mensajes eficiente
- Manejo automático de desconexiones y errores

### 3. Endpoint WebSocket (`app/routes/websocket.py`)
- `WebSocket /ws`: Endpoint principal para conexiones
- `GET /ws/stats`: Estadísticas del gestor (debugging)
- Query parameter opcional `client_name` para identificación

### 4. FFI Extensions (`backend/ffi/src/lib.rs`)
- `PyWatcherConfig`: Configuración del watcher expuesta a Python
- `PyFileEvent`: Evento de archivo detectado
- `start_file_watcher()`: Inicia watcher con callback Python
- `start_file_watcher_with_parsing()`: Watcher con auto-parsing de manos

### 5. File Watcher Service (`app/services/file_watcher_service.py`)
- `FileWatcherService`: Servicio que conecta Rust watcher con WebSocket
- Detección automática de nuevos archivos
- Parsing automático de manos detectadas
- Notificación vía WebSocket a todos los clientes
- Integrado en lifecycle de FastAPI

### 6. Tests (`tests/test_websocket_integration.py`)
- Tests de conexión WebSocket básica
- Tests de múltiples clientes simultáneos
- Tests de heartbeat
- Tests de broadcasting
- Tests de modelos de mensajes
- Tests de integración con file watcher

## Arquitectura del Flujo

```
Winamax genera archivo.txt
    ↓
Rust File Watcher (notify crate) detecta archivo
    ↓
Rust parsea archivo con FSM + Rayon
    ↓
Callback Python vía PyO3 con PyHandSummary[]
    ↓
FileWatcherService procesa manos
    ↓
WebSocketManager.notify_new_hand()
    ↓
Broadcast a todos los clientes WebSocket conectados
    ↓
Frontend recibe notificación en < 500ms
```

## Criterios de Aceptación
- ✅ Cliente recibe notificación < 500ms después de nueva mano
- ✅ Múltiples clientes pueden conectarse simultáneamente
- ✅ Reconexión automática funciona (manejo de desconexiones)
- ✅ No hay memory leaks en conexiones largas (verificado con tests)
- ✅ Heartbeat mantiene conexiones vivas
- ✅ Integración completa Rust → Python → WebSocket

## Rama
feat/issue-28-websocket-push

## Archivos Creados/Modificados
- `server-api/app/models/websocket.py` - Modelos Pydantic para mensajes
- `server-api/app/models/__init__.py` - Exportar modelos WebSocket
- `server-api/app/services/__init__.py` - Módulo de servicios
- `server-api/app/services/websocket_manager.py` - Gestor de conexiones WebSocket
- `server-api/app/services/file_watcher_service.py` - Servicio integrador
- `server-api/app/routes/websocket.py` - Endpoint WebSocket
- `server-api/app/routes/__init__.py` - Exportar router WebSocket
- `server-api/app/main.py` - Integrar watcher en lifecycle
- `server-api/app/bridge/poker_ffi.pyi` - Type stubs actualizados
- `backend/ffi/src/lib.rs` - Funciones FFI para file watcher
- `server-api/tests/test_websocket_integration.py` - Tests completos

---

## Contexto
- Fase 2.2: Orquestación y API
- Entry point: server-api/app/main.py
- Consume funciones Rust vía FFI

## Tareas Completadas
- [x] Crear modelos Pydantic para validación de requests/responses
- [x] Implementar endpoint GET /stats/player/{name} (VPIP, PFR, 3Bet)
- [x] Implementar endpoint GET /hands/recent (últimas N manos)
- [x] Implementar endpoint GET /hands/{hand_id} (detalle de mano)
- [x] Implementar endpoint POST /equity/calculate (cálculo Monte Carlo)
- [x] Implementar endpoint POST /equity/calculate/multiway (3+ jugadores)
- [x] Configurar CORS para localhost only (seguridad)
- [x] Documentación automática con OpenAPI/Swagger
- [x] Tests de integración con pytest

## Endpoints Implementados
- GET /health - Health check con estado FFI
- GET /api/v1/config - Configuración del servidor
- GET /api/v1/stats/player/{name} - Estadísticas de jugador con filtros
- GET /api/v1/hands/recent - Listado de manos recientes con paginación
- GET /api/v1/hands/{hand_id} - Detalle completo de una mano
- POST /api/v1/equity/calculate - Cálculo de equity 2 jugadores
- POST /api/v1/equity/calculate/multiway - Cálculo multiway

## Documentación
- Swagger UI disponible en /docs
- ReDoc disponible en /redoc
- OpenAPI JSON en /api/v1/openapi.json

## Rama
feat/issue-27-rest-endpoints

## Archivos Creados/Modificados
- `server-api/app/models/__init__.py` - Módulo de modelos
- `server-api/app/models/stats.py` - Modelos de estadísticas
- `server-api/app/models/hands.py` - Modelos de manos
- `server-api/app/models/equity.py` - Modelos de equity
- `server-api/app/routes/__init__.py` - Módulo de rutas actualizado
- `server-api/app/routes/stats.py` - Router de estadísticas
- `server-api/app/routes/hands.py` - Router de manos
- `server-api/app/routes/equity.py` - Router de equity
- `server-api/app/main.py` - Actualizado con routers y CORS
- `server-api/tests/test_api_endpoints.py` - Tests de integración

---

## Contexto
- Fase 2.2: Orquestación y API
- Exponer funciones Rust a Python sin overhead de serialización
- Zero-copy con Apache Arrow cuando sea posible

## Tareas
- [x] Crear crate poker-ffi en workspace de Rust
- [x] Configurar Cargo.toml con PyO3 y crate-type = [cdylib]
- [x] Exponer función de parsing de archivos Winamax
- [x] Exponer función de cálculo de equity
- [x] Exponer función de consulta a DuckDB (estructura preparada)
- [x] Crear módulo Python de ejemplo para testing
- [x] Documentar contrato FFI en docs/specs/ffi-contract.md

## Criterios de Aceptación
- [x] Las funciones Rust son llamables desde Python
- [x] Overhead de FFI < 1ms para operaciones típicas
- [x] Manejo correcto de errores entre lenguajes
- [x] Tests de integración Python-Rust pasan

## Rama
feat/issue-26-ffi-pyo3-bridge

## Archivos Creados/Modificados
- `backend/ffi/Cargo.toml` - Configuración del crate FFI
- `backend/ffi/pyproject.toml` - Configuración de maturin
- `backend/ffi/src/lib.rs` - Implementación PyO3
- `backend/Cargo.toml` - Añadido ffi al workspace
- `server-api/app/bridge/__init__.py` - Wrapper Python
- `server-api/app/bridge/poker_ffi.pyi` - Type stubs
- `server-api/tests/test_ffi_integration.py` - Tests
- `server-api/scripts/test_ffi_example.py` - Script demo
- `docs/specs/ffi-contract.md` - Documentación actualizada

---

## Issue #25 Completado (Resumen)

### Componentes Implementados
- Entorno Python con Poetry en server-api/
- Dependencias: FastAPI, Uvicorn, PyO3/maturin
- Estructura de carpetas (app/, bridge/, config/, routes/)
- Tests con pytest y conftest.py
- Archivo .env.example

---

## Issue #24 Completado (Resumen)

### Componentes Implementados
- Módulo equity_calculator con Monte Carlo
- Optimización SIMD AVX2 para evaluación batch
- Integración con Rayon para 16 threads
- Early stopping con convergencia < 0.1%

---

## Issue #23 Completado (Resumen)

### Componentes Implementados
- Perfect Hash Table de 7 cartas (133M combinaciones)
- Generación en 24 segundos con Rayon
- Búsquedas O(1) en 19.4ns
- Tamaño: 267MB en disco

---

## Issue #22 Completado (Resumen)

### Componentes Implementados
- Algoritmo Cactus Kev híbrido
- Evaluador de 5, 6 y 7 cartas
- Lookup tables para flush y unique5
- Performance < 100ns por evaluación

---

## Fase 1 Completada (Resumen)

### Componentes Implementados
- Parser Winamax (FSM, File Watcher, Rayon)
- Base de Datos Analítica (DuckDB, Parquet)
- 60+ tests pasando
