"""
Modelos Pydantic para validación de requests y responses.

Este módulo define los esquemas de datos para:
- Estadísticas de jugadores
- Información de manos
- Cálculos de equity
- Validación de query parameters
"""

from .stats import (
    PlayerStatsResponse,
    PositionalStats,
    PlayerStatsSummary,
    StatsFilters,
)
from .hands import (
    HandSummaryResponse,
    HandDetailResponse,
    HandAction,
    HandPlayer,
    HandFilters,
)
from .equity import (
    EquityCalculateRequest,
    EquityCalculateResponse,
    EquityMultiwayRequest,
)
from .websocket import (
    NewHandMessage,
    HeartbeatMessage,
    ErrorMessage,
    ConnectionAckMessage,
)

__all__ = [
    # Stats models
    "PlayerStatsResponse",
    "PositionalStats",
    "PlayerStatsSummary",
    "StatsFilters",
    # Hands models
    "HandSummaryResponse",
    "HandDetailResponse",
    "HandAction",
    "HandPlayer",
    "HandFilters",
    # Equity models
    "EquityCalculateRequest",
    "EquityCalculateResponse",
    "EquityMultiwayRequest",
    # WebSocket models
    "NewHandMessage",
    "HeartbeatMessage",
    "ErrorMessage",
    "ConnectionAckMessage",
]

