# Sistema de Auditoría y Logging Estructurado

## Visión General

El sistema de logging implementa un enfoque de logging estructurado en JSON para facilitar la auditoría, debugging y monitoreo de la aplicación. Los logs se rotan automáticamente y NO contienen información personal identificable (PII) ni contenido de manos.

## Arquitectura

### Python (FastAPI)

**Biblioteca**: `structlog` + `python-json-logger`

**Archivos de Log**:
- `logs/api.log` - Todos los logs de la aplicación
- `logs/audit.log` - Solo logs de auditoría (seguridad y operaciones críticas)

**Configuración**: `app/utils/logger.py`

### Rust (Parsers)

**Biblioteca**: `tracing` + `tracing-subscriber` + `tracing-appender`

**Archivos de Log**:
- `logs/parser-YYYY-MM-DD.log` - Logs del parser y file watcher

**Configuración**: `backend/parsers/src/logging.rs`

## Formato de Logs

### Estructura JSON

Todos los logs siguen este formato:

```json
{
  "timestamp": "2026-01-05T12:34:56.789Z",
  "level": "INFO",
  "component": "api",
  "event": "api_request_completed",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "method": "POST",
  "path": "/api/v1/equity/calculate",
  "status_code": 200,
  "duration_ms": 45.23
}
```

### Campos Comunes

- `timestamp` (string): ISO 8601 con timezone UTC
- `level` (string): ERROR | WARN | INFO | DEBUG
- `component` (string): api | parser | ffi | db
- `event` (string): Identificador del evento
- `request_id` (string): UUID único por request (solo API)

### Campos Específicos por Componente

#### API (`component: "api"`)
- `method`: Método HTTP (GET, POST, etc.)
- `path`: Ruta del endpoint
- `status_code`: Código de respuesta HTTP
- `duration_ms`: Duración de la request en milisegundos

#### Parser (`component: "parser"`)
- `file_path`: Nombre del archivo (solo filename, no ruta completa)
- `hands_count`: Número de manos procesadas
- `elapsed_ms`: Tiempo de procesamiento

#### Database (`component: "db"`)
- `query_type`: SELECT | INSERT | UPDATE
- `rows_affected`: Número de filas afectadas
- `duration_ms`: Duración de la query

## Niveles de Log

### ERROR
Errores que requieren atención inmediata. La aplicación puede continuar pero algo falló.

**Ejemplos**:
- Error al conectar con DuckDB
- Fallo al parsear un archivo
- Excepción no manejada en endpoint

### WARN
Situaciones anómalas que no son errores pero requieren atención.

**Ejemplos**:
- Archivo bloqueado, reintentando
- FFI no disponible
- Directorio de historiales no encontrado

### INFO
Eventos importantes del ciclo de vida de la aplicación.

**Ejemplos**:
- Aplicación iniciada
- Nuevo archivo detectado
- Request completada exitosamente

### DEBUG
Información detallada para debugging. Solo visible cuando `LOG_LEVEL=DEBUG`.

**Ejemplos**:
- Archivo siendo procesado
- Hash MD5 calculado
- Notificación de WebSocket enviada

## Rotación de Logs

### Python (RotatingFileHandler)
- **Tamaño máximo**: 100MB por archivo
- **Archivos históricos**: 5 (total 500MB)
- **Rotación**: Automática al alcanzar el tamaño

### Rust (tracing-appender)
- **Rotación**: Diaria (nuevo archivo por día)
- **Archivos históricos**: 5 días
- **Formato**: `parser-YYYY-MM-DD.log`

## Seguridad y Privacidad

### Datos que NO se loguean

1. **PII (Personally Identifiable Information)**
   - Direcciones IP completas
   - Emails
   - Información de pago

2. **Contenido de Manos**
   - Cartas específicas
   - Acciones detalladas de jugadores
   - Chat del juego

3. **Credenciales**
   - Passwords
   - Tokens
   - API Keys

### Sanitización Automática

El sistema detecta y redacta automáticamente:
- Campos con nombres: `password`, `token`, `secret`, `hand_content`
- Rutas completas de archivos (solo se mantiene el filename)

**Ejemplo**:
```python
logger.info("user_login", password="secret123", username="thesmoy")
# Output: {"event": "user_login", "password": "[REDACTED]", "username": "thesmoy"}
```

## Logs de Auditoría

Los logs de auditoría se escriben en `logs/audit.log` separado y capturan:

### API Requests
```python
audit_log(
    "api_request_completed",
    request_id="550e8400...",
    method="POST",
    path="/api/v1/equity/calculate",
    status_code=200,
    duration_ms=45.23
)
```

### Operaciones de Base de Datos
```python
audit_log(
    "db_query_executed",
    query_type="SELECT",
    table="hands_actions",
    rows_affected=1000,
    duration_ms=12.5
)
```

### File Watcher
```python
audit_log(
    "file_watcher_started",
    watch_path="C:\\Users\\Miguel\\...",
    max_retries=3
)

audit_log(
    "new_hands_detected",
    hands_count=42
)
```

## Uso en Código

### Python

#### Logger Básico
```python
from app.utils.logger import get_logger

logger = get_logger(__name__)

logger.info("user_action", user_id=123, action="fold")
logger.error("processing_failed", file_path="history.txt", error=str(e))
```

#### Audit Log
```python
from app.utils.logger import audit_log

audit_log(
    "security_event",
    event_type="unauthorized_access",
    ip_address="127.0.0.1",
    path="/admin"
)
```

#### Contexto Persistente
```python
logger = get_logger(__name__)
logger = logger.bind(request_id="550e8400...")

logger.info("step_1")  # Incluye request_id
logger.info("step_2")  # Incluye request_id
```

### Rust

#### Logger Básico
```rust
use tracing::{info, error, warn, debug};

info!(
    file_path = %path.display(),
    hands_count = 42,
    "File processed successfully"
);

error!(
    file_path = %path.display(),
    error = %e,
    "Failed to process file"
);
```

#### Spans (Contexto)
```rust
use tracing::{info, span, Level};

let span = span!(Level::INFO, "processing_file", file_path = %path.display());
let _enter = span.enter();

info!("Starting processing");
// Todos los logs dentro del span incluyen file_path
info!("Processing complete");
```

## Inicialización

### Python (app/main.py)
```python
from app.config.settings import Settings
from app.utils.logger import setup_logging

settings = Settings()
setup_logging(settings)  # Inicializa al arrancar la app
```

### Rust (main.rs o tests)
```rust
use poker_parsers::logging::init_logging;

init_logging("logs", "INFO");
```

## Configuración via Environment

```env
# Nivel de log
LOG_LEVEL=INFO  # TRACE | DEBUG | INFO | WARN | ERROR

# Directorio de logs
LOG_DIR=logs

# Tamaño máximo por archivo (bytes)
LOG_MAX_BYTES=104857600  # 100MB

# Número de archivos históricos
LOG_BACKUP_COUNT=5

# Formato (json o console)
LOG_FORMAT=json  # console para desarrollo
```

## Monitoreo y Análisis

### Buscar Errores
```bash
# Errores en las últimas 24 horas
grep '"level":"ERROR"' logs/api.log | jq .

# Errores específicos de parser
grep 'component":"parser"' logs/api.log | grep 'ERROR'
```

### Analizar Performance
```bash
# Requests más lentas (>1000ms)
grep 'api_request_completed' logs/audit.log | jq 'select(.duration_ms > 1000)'

# Promedio de duración de requests
grep 'api_request_completed' logs/audit.log | jq -r '.duration_ms' | awk '{sum+=$1; count++} END {print sum/count}'
```

### Contar Eventos
```bash
# Número de manos detectadas
grep 'new_hands_detected' logs/audit.log | jq -r '.hands_count' | awk '{sum+=$1} END {print sum}'

# Número de archivos procesados
grep 'file_processed_successfully' logs/parser-*.log | wc -l
```

## Testing

### Python
```python
# tests/test_logging.py
def test_logging_format(temp_log_dir):
    setup_logging(settings)
    logger = get_logger("test")
    logger.info("test_event", count=42)
    
    # Verificar formato JSON
    log_file = temp_log_dir / "api.log"
    content = log_file.read_text()
    log_entry = json.loads(content)
    assert log_entry["event"] == "test_event"
    assert log_entry["count"] == 42
```

### Rust
```rust
// tests en logging.rs
#[test]
fn test_structured_logging() {
    init_test_logging();
    info!(file_path = "test.txt", hands_count = 100, "File processed");
}
```

## Troubleshooting

### Logs no se generan
1. Verificar que `logs/` directory tiene permisos de escritura
2. Verificar que `LOG_LEVEL` permite el nivel del log
3. Verificar que logging se inicializó (`setup_logging` llamado)

### Rotación no funciona
1. Verificar `LOG_MAX_BYTES` en configuración
2. Verificar que hay espacio en disco
3. Verificar permisos del directorio

### Logs contienen información sensible
1. Verificar que `sanitize_event_dict` está en los processors
2. Añadir el campo a `sensitive_keys` en `sanitize_event_dict`
3. Reportar como issue de seguridad

## Referencias

- [structlog documentation](https://www.structlog.org/)
- [tracing documentation](https://docs.rs/tracing/)
- [12-Factor App: Logs](https://12factor.net/logs)

