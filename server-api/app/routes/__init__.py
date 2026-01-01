"""
API Routes module.

Expone los routers de la API:
- stats: Estadísticas de jugadores
- hands: Consulta de manos históricas
- equity: Cálculos de equity Monte Carlo
"""

from . import stats, hands, equity

__all__ = ["stats", "hands", "equity"]

