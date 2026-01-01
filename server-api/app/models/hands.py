"""
Modelos Pydantic para información de manos.
"""

from typing import Optional, List
from datetime import datetime
from pydantic import BaseModel, Field


class HandAction(BaseModel):
    """Acción individual en una mano."""
    
    player: str = Field(..., description="Nombre del jugador")
    street: str = Field(..., description="Etapa de la mano (Preflop, Flop, Turn, River)")
    action_type: str = Field(..., description="Tipo de acción (Fold, Call, Raise, Bet, Check)")
    amount_cents: int = Field(..., ge=0, description="Cantidad en centavos")
    is_hero: bool = Field(..., description="Si es la acción del héroe")


class HandPlayer(BaseModel):
    """Información de un jugador en la mano."""
    
    name: str = Field(..., description="Nombre del jugador")
    position: str = Field(..., description="Posición en la mesa (BTN, SB, BB, MP, CO, UTG)")
    stack_cents: int = Field(..., ge=0, description="Stack inicial en centavos")
    cards: Optional[List[str]] = Field(None, description="Cartas del jugador (si hay showdown)")


class HandSummaryResponse(BaseModel):
    """Resumen de una mano (para listados)."""
    
    hand_id: str = Field(..., description="ID único de la mano")
    timestamp: datetime = Field(..., description="Fecha/hora de la mano")
    table_name: str = Field(..., description="Nombre de la mesa")
    player_count: int = Field(..., ge=2, le=10, description="Número de jugadores")
    hero_played: bool = Field(..., description="Si thesmoy participó")
    total_pot_cents: int = Field(..., ge=0, description="Pot total en centavos")
    stake: Optional[str] = Field(None, description="Nivel de ciegas")
    
    class Config:
        json_schema_extra = {
            "example": {
                "hand_id": "20231215-12345678",
                "timestamp": "2023-12-15T14:30:00Z",
                "table_name": "Monaco",
                "player_count": 6,
                "hero_played": True,
                "total_pot_cents": 1250,
                "stake": "NL10"
            }
        }


class HandDetailResponse(BaseModel):
    """Detalle completo de una mano."""
    
    hand_id: str = Field(..., description="ID único de la mano")
    timestamp: datetime = Field(..., description="Fecha/hora de la mano")
    table_name: str = Field(..., description="Nombre de la mesa")
    stake: str = Field(..., description="Nivel de ciegas")
    button_position: int = Field(..., ge=0, le=9, description="Posición del botón")
    players: List[HandPlayer] = Field(..., description="Jugadores en la mano")
    actions: List[HandAction] = Field(..., description="Secuencia de acciones")
    board: Optional[List[str]] = Field(None, description="Cartas comunitarias")
    total_pot_cents: int = Field(..., ge=0, description="Pot total")
    rake_cents: int = Field(0, ge=0, description="Rake cobrado")
    
    class Config:
        json_schema_extra = {
            "example": {
                "hand_id": "20231215-12345678",
                "timestamp": "2023-12-15T14:30:00Z",
                "table_name": "Monaco",
                "stake": "NL10",
                "button_position": 3,
                "players": [
                    {
                        "name": "thesmoy",
                        "position": "BTN",
                        "stack_cents": 1000,
                        "cards": ["Ah", "Kd"]
                    }
                ],
                "actions": [
                    {
                        "player": "thesmoy",
                        "street": "Preflop",
                        "action_type": "Raise",
                        "amount_cents": 30,
                        "is_hero": True
                    }
                ],
                "board": ["Kh", "7s", "2c", "Qd", "3h"],
                "total_pot_cents": 1250,
                "rake_cents": 50
            }
        }


class HandFilters(BaseModel):
    """Filtros para consultas de manos."""
    
    limit: int = Field(50, ge=1, le=500, description="Número máximo de manos a retornar")
    offset: int = Field(0, ge=0, description="Offset para paginación")
    start_date: Optional[datetime] = Field(None, description="Fecha inicio")
    end_date: Optional[datetime] = Field(None, description="Fecha fin")
    stake: Optional[str] = Field(None, description="Nivel de ciegas")
    hero_only: bool = Field(False, description="Solo manos donde participó thesmoy")

