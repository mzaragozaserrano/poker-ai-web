# Hook useWebSocket - Documentación

Hook personalizado de React para gestionar la conexión WebSocket en tiempo real con el backend de Poker AI.

## Endpoint

```
ws://127.0.0.1:8000/ws
```

## Instalación y Uso

### Uso Básico

```typescript
import { useWebSocket } from './hooks'

function MyComponent() {
  const websocket = useWebSocket()

  return (
    <div>
      <p>Estado: {websocket.status}</p>
      <p>Cliente ID: {websocket.clientId}</p>
    </div>
  )
}
```

### Uso con Callbacks

```typescript
import { useWebSocket } from './hooks'
import { NewHandMessage } from './types/api'

function HandNotifications() {
  const [hands, setHands] = useState<NewHandMessage[]>([])

  const websocket = useWebSocket({
    onNewHand: (message) => {
      console.log('Nueva mano detectada:', message.hand_id)
      setHands((prev) => [message, ...prev])
    },
    onConnectionAck: (message) => {
      console.log('Conectado con ID:', message.client_id)
    },
    onError: (message) => {
      console.error('Error WebSocket:', message.message)
    },
  })

  return (
    <div>
      {hands.map((hand) => (
        <div key={hand.hand_id}>
          {hand.game_type} - {hand.stakes} - Resultado: {hand.hero_result}
        </div>
      ))}
    </div>
  )
}
```

### Control Manual de Conexión

```typescript
function ManualConnection() {
  const websocket = useWebSocket({
    autoConnect: false, // No conectar automáticamente
  })

  return (
    <div>
      <button onClick={() => websocket.connect()}>Conectar</button>
      <button onClick={() => websocket.disconnect()}>Desconectar</button>
      <p>Estado: {websocket.status}</p>
    </div>
  )
}
```

## API del Hook

### Opciones (UseWebSocketOptions)

| Opción          | Tipo                                    | Default                        | Descripción                                      |
| --------------- | --------------------------------------- | ------------------------------ | ------------------------------------------------ |
| `url`           | `string`                                | `ws://127.0.0.1:8000/ws`       | URL del WebSocket                                |
| `clientName`    | `string`                                | `react_client`                 | Nombre del cliente (query param)                 |
| `autoConnect`   | `boolean`                               | `true`                         | Conectar automáticamente al montar               |
| `autoReconnect` | `boolean`                               | `true`                         | Reconectar automáticamente si se pierde conexión |
| `onNewHand`     | `(message: NewHandMessage) => void`     | `undefined`                    | Callback cuando se detecta nueva mano            |
| `onConnectionAck` | `(message: ConnectionAckMessage) => void` | `undefined`                  | Callback cuando se confirma conexión             |
| `onHeartbeat`   | `(message: HeartbeatMessage) => void`   | `undefined`                    | Callback en cada heartbeat                       |
| `onError`       | `(message: ErrorMessage) => void`       | `undefined`                    | Callback cuando hay error                        |
| `onMessage`     | `(message: WebSocketMessage) => void`   | `undefined`                    | Callback para cualquier mensaje (debug)          |

### Valores de Retorno (UseWebSocketReturn)

| Propiedad        | Tipo                  | Descripción                                 |
| ---------------- | --------------------- | ------------------------------------------- |
| `status`         | `ConnectionStatus`    | Estado actual: `connecting`, `connected`, `disconnected`, `reconnecting` |
| `isConnected`    | `boolean`             | `true` si está conectado                    |
| `lastMessage`    | `WebSocketMessage \| null` | Último mensaje recibido                |
| `clientId`       | `string \| null`      | ID del cliente asignado por el servidor     |
| `messageHistory` | `WebSocketMessage[]`  | Historial de mensajes (últimos 50)          |
| `connect`        | `() => void`          | Conectar manualmente                        |
| `disconnect`     | `() => void`          | Desconectar manualmente                     |
| `send`           | `(data: string) => void` | Enviar mensaje (para uso futuro)         |

## Tipos de Mensajes

### ConnectionAckMessage

Confirmación de conexión enviada por el servidor.

```typescript
{
  type: 'connection_ack',
  client_id: 'uuid-generado-por-servidor',
  timestamp: '2024-01-01T12:00:00.000Z'
}
```

### HeartbeatMessage

Mensaje de keepalive enviado cada 30 segundos.

```typescript
{
  type: 'heartbeat',
  timestamp: '2024-01-01T12:00:00.000Z'
}
```

### NewHandMessage

Notificación de nueva mano detectada por el file watcher.

```typescript
{
  type: 'new_hand',
  hand_id: '20240101120000_TableName_Hand123',
  timestamp: '2024-01-01T12:00:00.000Z',
  hero_result: 5.50,        // En euros (null si hero no participó)
  hero_position: 'BTN',     // Posición del hero
  game_type: 'NLH',         // Tipo de juego
  stakes: '0.05/0.10'       // Límites de la mesa
}
```

### ErrorMessage

Mensaje de error del servidor.

```typescript
{
  type: 'error',
  message: 'Descripción del error',
  timestamp: '2024-01-01T12:00:00.000Z'
}
```

## Características

### Reconexión Automática

El hook implementa reconexión automática con **backoff exponencial**:

- **Delay inicial:** 5 segundos
- **Delay máximo:** 60 segundos
- **Multiplicador:** 2x en cada intento

Ejemplo de secuencia de reconexión:
1. Primer intento: 5s
2. Segundo intento: 10s
3. Tercer intento: 20s
4. Cuarto intento: 40s
5. Quinto intento: 60s (máximo)

### Gestión de Memoria

- **Cleanup automático:** Al desmontar el componente, se cierra la conexión y se cancelan temporizadores.
- **Historial limitado:** Solo se mantienen los últimos 50 mensajes para evitar memory leaks.
- **No memory leaks:** Todas las referencias se limpian correctamente.

### Estados de Conexión

| Estado         | Descripción                                    |
| -------------- | ---------------------------------------------- |
| `connecting`   | Iniciando conexión                             |
| `connected`    | Conectado y listo                              |
| `disconnected` | Desconectado (manual o sin reconexión)         |
| `reconnecting` | Intentando reconectar automáticamente          |

## Demo

Visita la página de demostración para ver el hook en acción:

```
http://localhost:5173/api-demo
```

La demo muestra:
- Estado de conexión en tiempo real
- Botones de conectar/desconectar
- Notificaciones de nuevas manos
- Historial de mensajes (debug)

## Troubleshooting

### El WebSocket no conecta

1. Verificar que el backend está corriendo:
   ```bash
   curl http://127.0.0.1:8000/health
   ```

2. Verificar estadísticas del WebSocket:
   ```bash
   curl http://127.0.0.1:8000/ws/stats
   ```

### No se reciben mensajes

1. Verificar que el file watcher está activo (ver logs del backend)
2. Verificar que `autoConnect` está en `true`
3. Revisar el historial de mensajes en la demo

### Reconexión continua

- Verificar que el backend no está rechazando la conexión
- Revisar logs del navegador para errores específicos

## Performance

### Latencia Total

Desde detección del archivo hasta notificación en el frontend:

- **File detection:** ~10ms
- **Parsing (Rust):** ~50-200ms
- **FFI callback:** ~1ms
- **WebSocket broadcast:** ~5-10ms
- **React render:** ~5-10ms
- **Total:** ~70-230ms ✅

### Recursos

- **Memory:** ~100KB por conexión
- **CPU:** Mínimo (solo eventos push)
- **Network:** ~1KB cada 30s (heartbeat)

## Seguridad

⚠️ **Nota de Seguridad:** Esta implementación es para uso local (127.0.0.1). Para producción:
- Implementar autenticación (JWT tokens)
- Usar WSS (WebSocket Secure)
- Validar origen de conexiones
- Rate limiting por cliente

## Referencias

- [Backend WebSocket Docs](../server-api/WEBSOCKET_README.md)
- [FastAPI WebSockets](https://fastapi.tiangolo.com/advanced/websockets/)
- [MDN WebSocket API](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)

