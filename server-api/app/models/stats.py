"""
Modelos Pydantic para estadísticas de jugadores.
"""

from typing import Optional
from datetime import datetime
from pydantic import BaseModel, Field, field_validator


class PositionalStats(BaseModel):
    """Estadísticas desglosadas por posición."""
    
    vpip: float = Field(..., ge=0, le=100, description="VPIP en porcentaje")
    pfr: float = Field(..., ge=0, le=100, description="PFR en porcentaje")
    three_bet: Optional[float] = Field(None, ge=0, le=100, description="3-bet en porcentaje")
    hands: int = Field(..., ge=0, description="Número de manos en esta posición")


class PlayerStatsSummary(BaseModel):
    """Resumen de estadísticas agregadas del jugador."""
    
    hands: int = Field(..., ge=0, description="Total de manos jugadas")
    vpip: float = Field(..., ge=0, le=100, description="VPIP global")
    pfr: float = Field(..., ge=0, le=100, description="PFR global")
    three_bet: float = Field(..., ge=0, le=100, description="3-bet global")
    wtsd: Optional[float] = Field(None, ge=0, le=100, description="Went to Showdown")
    af: Optional[float] = Field(None, ge=0, description="Aggression Factor")
    net_won_cents: int = Field(..., description="Ganancia neta en centavos")
    bb_100: Optional[float] = Field(None, description="Winrate en bb/100")
    ev_bb_100: Optional[float] = Field(None, description="EV winrate en bb/100")


class PlayerStatsResponse(BaseModel):
    """Response completa para estadísticas de jugador."""
    
    player: str = Field(..., min_length=1, description="Nombre del jugador")
    is_hero: bool = Field(..., description="Si es el jugador principal (thesmoy)")
    summary: PlayerStatsSummary = Field(..., description="Estadísticas agregadas")
    positional: dict[str, PositionalStats] = Field(
        ..., 
        description="Estadísticas por posición (BTN, SB, BB, MP, CO)"
    )
    
    class Config:
        json_schema_extra = {
            "example": {
                "player": "thesmoy",
                "is_hero": True,
                "summary": {
                    "hands": 1540,
                    "vpip": 24.5,
                    "pfr": 20.1,
                    "three_bet": 8.2,
                    "wtsd": 28.4,
                    "af": 2.5,
                    "net_won_cents": 4500,
                    "bb_100": 15.2,
                    "ev_bb_100": 12.8
                },
                "positional": {
                    "BTN": {"vpip": 45.0, "pfr": 38.0, "three_bet": 12.0, "hands": 250},
                    "SB": {"vpip": 32.0, "pfr": 28.0, "three_bet": 10.0, "hands": 250},
                    "BB": {"vpip": 12.0, "pfr": 0.0, "three_bet": 5.0, "hands": 250},
                    "MP": {"vpip": 18.0, "pfr": 15.0, "three_bet": 6.0, "hands": 250},
                    "CO": {"vpip": 26.0, "pfr": 22.0, "three_bet": 9.0, "hands": 250}
                }
            }
        }


class StatsFilters(BaseModel):
    """Filtros de query para estadísticas."""
    
    start_date: Optional[datetime] = Field(None, description="Fecha inicio (ISO-8601)")
    end_date: Optional[datetime] = Field(None, description="Fecha fin (ISO-8601)")
    stake: Optional[str] = Field(None, description="Nivel de ciegas (ej: 'NL2', 'NL10')")
    game_type: Optional[str] = Field(
        "NLHE", 
        description="Tipo de juego",
        pattern="^(NLHE|PLO)$"
    )
    min_hands: int = Field(1, ge=1, description="Mínimo de manos para incluir estadística")
    
    @field_validator('start_date', 'end_date')
    @classmethod
    def validate_dates(cls, v):
        """Valida que las fechas estén en UTC."""
        if v is not None and v.tzinfo is None:
            # Asumir UTC si no hay timezone
            return v.replace(tzinfo=None)
        return v
    
    @field_validator('end_date')
    @classmethod
    def validate_end_after_start(cls, v, values):
        """Valida que end_date sea posterior a start_date."""
        if v is not None and 'start_date' in values.data:
            start = values.data['start_date']
            if start is not None and v < start:
                raise ValueError("end_date debe ser posterior a start_date")
        return v

