# WebSocket Real-Time Notifications

Sistema de notificaciones en tiempo real para nuevas manos detectadas por el file watcher.

## Arquitectura

```
Winamax → File Watcher (Rust) → FFI (PyO3) → WebSocket Manager → Clientes
```

## Uso del Cliente

### Conexión Básica (JavaScript/TypeScript)

```javascript
// Conectar al WebSocket
const ws = new WebSocket('ws://localhost:8000/ws?client_name=my_client');

// Evento: conexión establecida
ws.onopen = () => {
    console.log('Conectado al servidor');
};

// Evento: mensaje recibido
ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    
    switch (message.type) {
        case 'connection_ack':
            console.log('Conexión confirmada:', message.client_id);
            break;
            
        case 'new_hand':
            console.log('Nueva mano detectada:', message.hand_id);
            console.log('Resultado hero:', message.hero_result);
            console.log('Posición:', message.hero_position);
            break;
            
        case 'heartbeat':
            console.log('Heartbeat recibido');
            break;
            
        case 'error':
            console.error('Error:', message.message);
            break;
    }
};

// Evento: error de conexión
ws.onerror = (error) => {
    console.error('WebSocket error:', error);
};

// Evento: conexión cerrada
ws.onclose = () => {
    console.log('Desconectado del servidor');
    // Implementar reconexión automática aquí
    setTimeout(() => {
        // Reconectar después de 5 segundos
        connectWebSocket();
    }, 5000);
};
```

### Ejemplo con Python (Cliente)

```python
import asyncio
import websockets
import json

async def connect_websocket():
    uri = "ws://localhost:8000/ws?client_name=python_client"
    
    async with websockets.connect(uri) as websocket:
        print("Conectado al servidor")
        
        # Recibir mensajes
        async for message in websocket:
            data = json.loads(message)
            
            if data["type"] == "connection_ack":
                print(f"Conexión confirmada: {data['client_id']}")
                
            elif data["type"] == "new_hand":
                print(f"Nueva mano: {data['hand_id']}")
                print(f"Resultado: {data['hero_result']}")
                
            elif data["type"] == "heartbeat":
                print("Heartbeat recibido")

# Ejecutar
asyncio.run(connect_websocket())
```

## Tipos de Mensajes

### 1. ConnectionAckMessage

Mensaje de confirmación de conexión (recibido al conectar).

```json
{
    "type": "connection_ack",
    "client_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2024-01-01T12:00:00.000Z"
}
```

### 2. NewHandMessage

Notificación de nueva mano detectada.

```json
{
    "type": "new_hand",
    "hand_id": "20240101120000_TableName_Hand123",
    "timestamp": "2024-01-01T12:00:00.000Z",
    "hero_result": 5.50,
    "hero_position": "BTN",
    "game_type": "NLH",
    "stakes": "0.05/0.10"
}
```

**Campos:**
- `hand_id`: ID único de la mano
- `timestamp`: Momento en que se jugó la mano
- `hero_result`: Resultado del hero (positivo = ganancia, negativo = pérdida, null = no participó)
- `hero_position`: Posición del hero (BTN, SB, BB, UTG, MP, CO)
- `game_type`: Tipo de juego (NLH, PLO, etc.)
- `stakes`: Límites de la mesa

### 3. HeartbeatMessage

Mensaje de keepalive (enviado cada 30 segundos).

```json
{
    "type": "heartbeat",
    "timestamp": "2024-01-01T12:00:00.000Z"
}
```

### 4. ErrorMessage

Mensaje de error.

```json
{
    "type": "error",
    "message": "Descripción del error",
    "timestamp": "2024-01-01T12:00:00.000Z"
}
```

## Endpoint de Estadísticas

Para debugging y monitoring, el servidor expone un endpoint REST con estadísticas:

```bash
curl http://localhost:8000/ws/stats
```

**Respuesta:**
```json
{
    "active_connections": 3,
    "heartbeat_interval": 30,
    "heartbeat_active": true,
    "broadcast_worker_active": true,
    "queue_size": 0
}
```

## Configuración del Servidor

### Variables de Entorno

El file watcher se configura automáticamente desde `settings.py`:

```python
WINAMAX_HISTORY_PATH = "/path/to/winamax/history"
```

### Inicio Manual del File Watcher

Si necesitas iniciar el file watcher manualmente (para testing):

```python
from app.services.file_watcher_service import get_file_watcher_service

# Inicializar y arrancar
watcher = get_file_watcher_service("/path/to/history")
watcher.start()
```

## Reconexión Automática

El cliente debe implementar reconexión automática en caso de pérdida de conexión:

```javascript
class WebSocketClient {
    constructor(url) {
        this.url = url;
        this.reconnectDelay = 5000; // 5 segundos
        this.connect();
    }
    
    connect() {
        this.ws = new WebSocket(this.url);
        
        this.ws.onopen = () => {
            console.log('Conectado');
            this.reconnectDelay = 5000; // Reset delay
        };
        
        this.ws.onmessage = (event) => {
            this.handleMessage(JSON.parse(event.data));
        };
        
        this.ws.onerror = (error) => {
            console.error('Error:', error);
        };
        
        this.ws.onclose = () => {
            console.log('Desconectado, reconectando...');
            setTimeout(() => this.connect(), this.reconnectDelay);
            // Backoff exponencial
            this.reconnectDelay = Math.min(this.reconnectDelay * 2, 60000);
        };
    }
    
    handleMessage(message) {
        // Procesar mensajes
    }
}

// Uso
const client = new WebSocketClient('ws://localhost:8000/ws');
```

## Performance

### Latencia

El sistema está optimizado para latencia < 500ms desde la detección del archivo hasta la notificación al cliente:

- **File detection**: ~10ms (notify crate con inotify/kqueue)
- **Parsing**: ~50-200ms (Rust FSM + Rayon)
- **FFI callback**: ~1ms (PyO3 zero-copy)
- **WebSocket broadcast**: ~5-10ms (asyncio)
- **Total**: ~70-220ms ✅

### Múltiples Clientes

El `WebSocketManager` soporta múltiples clientes simultáneos sin degradación de performance:

- Broadcasting eficiente con asyncio
- No hay bloqueo entre clientes
- Manejo automático de desconexiones

### Memory Management

- Conexiones fallidas se limpian automáticamente
- Heartbeat mantiene solo conexiones vivas
- Queue de eventos con límite para evitar memory leaks

## Testing

Ejecutar tests de integración:

```bash
cd server-api
poetry run pytest tests/test_websocket_integration.py -v
```

Tests incluidos:
- Conexión básica
- Múltiples clientes simultáneos
- Heartbeat
- Broadcasting
- Integración con file watcher
- Serialización de mensajes

## Troubleshooting

### Cliente no recibe mensajes

1. Verificar que el servidor está corriendo:
   ```bash
   curl http://localhost:8000/health
   ```

2. Verificar estadísticas del WebSocket:
   ```bash
   curl http://localhost:8000/ws/stats
   ```

3. Verificar que el file watcher está activo:
   ```bash
   # Revisar logs del servidor
   tail -f server.log | grep "FileWatcherService"
   ```

### File watcher no detecta archivos

1. Verificar que la ruta es correcta:
   ```python
   from app.config.settings import Settings
   settings = Settings()
   print(settings.winamax_history_path)
   ```

2. Verificar que el FFI está disponible:
   ```python
   from app.bridge import is_ffi_available
   print(is_ffi_available())
   ```

3. Verificar permisos del directorio:
   ```bash
   ls -la /path/to/winamax/history
   ```

## Roadmap

### Mejoras Futuras

- [ ] Filtros de suscripción (solo manos donde hero participó, solo stakes específicos, etc.)
- [ ] Compression de mensajes (para reducir bandwidth)
- [ ] Autenticación de clientes (JWT tokens)
- [ ] Rate limiting por cliente
- [ ] Replay de eventos perdidos (event store)
- [ ] Métricas de Prometheus para monitoring

## Referencias

- [FastAPI WebSockets](https://fastapi.tiangolo.com/advanced/websockets/)
- [PyO3 Guide](https://pyo3.rs/)
- [WebSocket Protocol RFC 6455](https://datatracker.ietf.org/doc/html/rfc6455)

