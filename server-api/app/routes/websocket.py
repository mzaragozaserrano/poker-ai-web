"""
WebSocket endpoint para notificaciones en tiempo real.

Maneja conexiones WebSocket para push de eventos de nuevas manos.
"""

import logging
from typing import Optional
from fastapi import APIRouter, WebSocket, WebSocketDisconnect, Query
from app.services.websocket_manager import get_ws_manager

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/ws", tags=["websocket"])


@router.websocket("")
async def websocket_endpoint(
    websocket: WebSocket,
    client_name: Optional[str] = Query(None, description="Nombre opcional del cliente para logging"),
) -> None:
    """
    Endpoint WebSocket para notificaciones en tiempo real.
    
    **Flujo de conexión:**
    1. Cliente se conecta a /ws
    2. Servidor envía mensaje de confirmación con client_id
    3. Servidor envía heartbeats cada 30 segundos
    4. Servidor pushea eventos de nuevas manos cuando son detectadas
    
    **Tipos de mensajes:**
    - `connection_ack`: Confirmación de conexión con client_id
    - `heartbeat`: Mantiene la conexión viva
    - `new_hand`: Notificación de nueva mano detectada
    - `error`: Mensaje de error
    
    Args:
        websocket: Instancia de WebSocket de FastAPI
        client_name: Nombre opcional del cliente (para logging/debug)
    """
    ws_manager = get_ws_manager()
    client_id = None
    
    try:
        # Establecer conexión
        client_id = await ws_manager.connect(websocket)
        logger.info(f"WebSocket client {client_id} connected (name: {client_name or 'anonymous'})")
        
        # Loop principal: esperar mensajes del cliente (aunque no esperamos muchos)
        while True:
            try:
                # Recibir mensajes del cliente (ej: ping, keepalive)
                data = await websocket.receive_text()
                logger.debug(f"Received from {client_id}: {data}")
                
                # Por ahora no procesamos mensajes del cliente
                # En el futuro podríamos agregar comandos como "pause", "resume", etc.
                
            except WebSocketDisconnect:
                logger.info(f"Client {client_id} disconnected normally")
                break
            except Exception as e:
                logger.error(f"Error in WebSocket loop for {client_id}: {e}")
                break
                
    except Exception as e:
        logger.error(f"Error in WebSocket connection: {e}")
    finally:
        # Limpiar conexión
        if client_id:
            ws_manager.disconnect(client_id)


@router.get("/stats")
async def websocket_stats() -> dict:
    """
    Obtiene estadísticas del gestor de WebSocket.
    
    Útil para debugging y monitoring.
    
    Returns:
        Estadísticas del WebSocketManager
    """
    ws_manager = get_ws_manager()
    return ws_manager.get_stats()

