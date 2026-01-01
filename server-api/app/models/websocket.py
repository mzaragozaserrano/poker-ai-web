"""
WebSocket message models.

Define los tipos de mensajes que se envían a través del WebSocket.
"""

from datetime import datetime
from typing import Literal, Optional
from pydantic import BaseModel, Field


class NewHandMessage(BaseModel):
    """
    Mensaje de notificación de nueva mano detectada.
    """
    type: Literal["new_hand"] = "new_hand"
    hand_id: str = Field(..., description="Identificador único de la mano")
    timestamp: datetime = Field(..., description="Timestamp de la mano")
    hero_result: Optional[float] = Field(None, description="Resultado del hero (positivo = ganancia)")
    hero_position: Optional[str] = Field(None, description="Posición del hero (BTN, SB, BB, etc)")
    game_type: str = Field(default="NLH", description="Tipo de juego (NLH, PLO, etc)")
    stakes: str = Field(..., description="Límites de la mesa (ej: 0.05/0.10)")


class HeartbeatMessage(BaseModel):
    """
    Mensaje de heartbeat para mantener la conexión viva.
    """
    type: Literal["heartbeat"] = "heartbeat"
    timestamp: datetime = Field(default_factory=datetime.utcnow)


class ErrorMessage(BaseModel):
    """
    Mensaje de error.
    """
    type: Literal["error"] = "error"
    message: str = Field(..., description="Descripción del error")
    timestamp: datetime = Field(default_factory=datetime.utcnow)


class ConnectionAckMessage(BaseModel):
    """
    Mensaje de confirmación de conexión.
    """
    type: Literal["connection_ack"] = "connection_ack"
    client_id: str = Field(..., description="ID único del cliente")
    timestamp: datetime = Field(default_factory=datetime.utcnow)

