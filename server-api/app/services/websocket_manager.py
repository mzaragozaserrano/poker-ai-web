"""
WebSocket connection manager.

Gestiona las conexiones WebSocket activas y el broadcasting de mensajes.
"""

import asyncio
import uuid
from datetime import datetime
from typing import Dict, Set, Optional
from fastapi import WebSocket
import logging

from app.models.websocket import (
    NewHandMessage,
    HeartbeatMessage,
    ErrorMessage,
    ConnectionAckMessage,
)

logger = logging.getLogger(__name__)


class WebSocketManager:
    """
    Gestor centralizado de conexiones WebSocket.
    
    Maneja múltiples clientes conectados simultáneamente,
    broadcasting de mensajes y heartbeat automático.
    """

    def __init__(self, heartbeat_interval: int = 30):
        """
        Inicializa el gestor de WebSocket.
        
        Args:
            heartbeat_interval: Intervalo en segundos entre heartbeats
        """
        self.active_connections: Dict[str, WebSocket] = {}
        self.heartbeat_interval = heartbeat_interval
        self._heartbeat_task: Optional[asyncio.Task] = None
        self._event_queue: asyncio.Queue = asyncio.Queue()
        self._broadcast_task: Optional[asyncio.Task] = None
        logger.info("WebSocketManager initialized")

    async def connect(self, websocket: WebSocket) -> str:
        """
        Acepta una nueva conexión WebSocket.
        
        Args:
            websocket: Instancia de WebSocket de FastAPI
            
        Returns:
            client_id: ID único asignado al cliente
        """
        await websocket.accept()
        client_id = str(uuid.uuid4())
        self.active_connections[client_id] = websocket
        
        # Enviar mensaje de confirmación
        ack_message = ConnectionAckMessage(client_id=client_id)
        await websocket.send_text(ack_message.model_dump_json())
        
        logger.info(f"Client {client_id} connected. Total connections: {len(self.active_connections)}")
        
        # Iniciar heartbeat si es la primera conexión
        if len(self.active_connections) == 1:
            self._start_heartbeat()
            self._start_broadcast_worker()
        
        return client_id

    def disconnect(self, client_id: str) -> None:
        """
        Desconecta un cliente.
        
        Args:
            client_id: ID del cliente a desconectar
        """
        if client_id in self.active_connections:
            del self.active_connections[client_id]
            logger.info(f"Client {client_id} disconnected. Total connections: {len(self.active_connections)}")
        
        # Detener heartbeat si no hay más conexiones
        if len(self.active_connections) == 0:
            self._stop_heartbeat()
            self._stop_broadcast_worker()

    async def send_personal_message(self, message: str, client_id: str) -> bool:
        """
        Envía un mensaje a un cliente específico.
        
        Args:
            message: Mensaje a enviar (JSON string)
            client_id: ID del cliente destino
            
        Returns:
            True si se envió correctamente, False en caso de error
        """
        websocket = self.active_connections.get(client_id)
        if websocket:
            try:
                await websocket.send_text(message)
                return True
            except Exception as e:
                logger.error(f"Error sending message to {client_id}: {e}")
                self.disconnect(client_id)
                return False
        return False

    async def broadcast(self, message: str) -> int:
        """
        Envía un mensaje a todos los clientes conectados.
        
        Args:
            message: Mensaje a enviar (JSON string)
            
        Returns:
            Número de clientes que recibieron el mensaje exitosamente
        """
        if not self.active_connections:
            return 0
        
        success_count = 0
        failed_clients = []
        
        for client_id, websocket in self.active_connections.items():
            try:
                await websocket.send_text(message)
                success_count += 1
            except Exception as e:
                logger.error(f"Error broadcasting to {client_id}: {e}")
                failed_clients.append(client_id)
        
        # Limpiar conexiones fallidas
        for client_id in failed_clients:
            self.disconnect(client_id)
        
        return success_count

    def queue_broadcast(self, message: str) -> None:
        """
        Encola un mensaje para broadcast asíncrono.
        
        Útil cuando se llama desde un thread diferente (ej: desde Rust FFI).
        
        Args:
            message: Mensaje a enviar (JSON string)
        """
        try:
            self._event_queue.put_nowait(message)
        except asyncio.QueueFull:
            logger.warning("Event queue is full, dropping message")

    async def notify_new_hand(
        self,
        hand_id: str,
        timestamp: datetime,
        hero_result: Optional[float] = None,
        hero_position: Optional[str] = None,
        stakes: str = "0.05/0.10",
    ) -> int:
        """
        Notifica a todos los clientes sobre una nueva mano detectada.
        
        Args:
            hand_id: ID de la mano
            timestamp: Timestamp de la mano
            hero_result: Resultado del hero (opcional)
            hero_position: Posición del hero (opcional)
            stakes: Límites de la mesa
            
        Returns:
            Número de clientes notificados
        """
        message = NewHandMessage(
            hand_id=hand_id,
            timestamp=timestamp,
            hero_result=hero_result,
            hero_position=hero_position,
            stakes=stakes,
        )
        return await self.broadcast(message.model_dump_json())

    def _start_heartbeat(self) -> None:
        """Inicia el task de heartbeat."""
        if self._heartbeat_task is None or self._heartbeat_task.done():
            self._heartbeat_task = asyncio.create_task(self._heartbeat_loop())
            logger.info("Heartbeat task started")

    def _stop_heartbeat(self) -> None:
        """Detiene el task de heartbeat."""
        if self._heartbeat_task and not self._heartbeat_task.done():
            self._heartbeat_task.cancel()
            logger.info("Heartbeat task stopped")

    def _start_broadcast_worker(self) -> None:
        """Inicia el worker de broadcast."""
        if self._broadcast_task is None or self._broadcast_task.done():
            self._broadcast_task = asyncio.create_task(self._broadcast_worker())
            logger.info("Broadcast worker started")

    def _stop_broadcast_worker(self) -> None:
        """Detiene el worker de broadcast."""
        if self._broadcast_task and not self._broadcast_task.done():
            self._broadcast_task.cancel()
            logger.info("Broadcast worker stopped")

    async def _heartbeat_loop(self) -> None:
        """
        Loop de heartbeat que se ejecuta en background.
        
        Envía mensajes de heartbeat periódicamente a todos los clientes.
        """
        try:
            while True:
                await asyncio.sleep(self.heartbeat_interval)
                if self.active_connections:
                    heartbeat = HeartbeatMessage()
                    count = await self.broadcast(heartbeat.model_dump_json())
                    logger.debug(f"Heartbeat sent to {count} clients")
        except asyncio.CancelledError:
            logger.info("Heartbeat loop cancelled")

    async def _broadcast_worker(self) -> None:
        """
        Worker que procesa mensajes de la cola de eventos.
        
        Permite que threads externos (ej: Rust FFI) encolen mensajes
        para broadcast sin bloquear.
        """
        try:
            while True:
                message = await self._event_queue.get()
                await self.broadcast(message)
                self._event_queue.task_done()
        except asyncio.CancelledError:
            logger.info("Broadcast worker cancelled")

    def get_stats(self) -> dict:
        """
        Retorna estadísticas del gestor.
        
        Returns:
            Dict con métricas del gestor
        """
        return {
            "active_connections": len(self.active_connections),
            "heartbeat_interval": self.heartbeat_interval,
            "heartbeat_active": self._heartbeat_task is not None and not self._heartbeat_task.done(),
            "broadcast_worker_active": self._broadcast_task is not None and not self._broadcast_task.done(),
            "queue_size": self._event_queue.qsize(),
        }


# Singleton global del gestor
_ws_manager: Optional[WebSocketManager] = None


def get_ws_manager() -> WebSocketManager:
    """
    Obtiene la instancia global del WebSocketManager.
    
    Returns:
        Instancia singleton del WebSocketManager
    """
    global _ws_manager
    if _ws_manager is None:
        _ws_manager = WebSocketManager()
    return _ws_manager

