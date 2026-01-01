"""
API Routes module.

Expone los routers de la API:
- stats: Estadísticas de jugadores
- hands: Consulta de manos históricas
- equity: Cálculos de equity Monte Carlo
- websocket: Conexiones WebSocket para notificaciones en tiempo real
"""

from . import stats, hands, equity, websocket

__all__ = ["stats", "hands", "equity", "websocket"]

