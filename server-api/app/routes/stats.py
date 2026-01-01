"""
Router para endpoints de estadísticas de jugadores.
"""

from typing import Optional
from datetime import datetime
from fastapi import APIRouter, HTTPException, Query
from app.models.stats import PlayerStatsResponse, PlayerStatsSummary, PositionalStats

router = APIRouter(prefix="/api/v1/stats", tags=["statistics"])


@router.get("/player/{player_name}", response_model=PlayerStatsResponse)
async def get_player_stats(
    player_name: str,
    start_date: Optional[datetime] = Query(None, description="Fecha inicio (ISO-8601)"),
    end_date: Optional[datetime] = Query(None, description="Fecha fin (ISO-8601)"),
    stake: Optional[str] = Query(None, description="Nivel de ciegas (ej: 'NL2', 'NL10')"),
    game_type: str = Query("NLHE", description="Tipo de juego (NLHE, PLO)"),
    min_hands: int = Query(1, ge=1, description="Mínimo de manos para incluir"),
) -> PlayerStatsResponse:
    """
    Obtiene las métricas agregadas de un jugador.
    
    Retorna estadísticas globales y desglosadas por posición (6-max).
    Los datos se calculan desde DuckDB filtrando por los criterios especificados.
    
    **Nota:** En mesas de 5 jugadores, se omite UTG (EP) según la especialización 6-max.
    
    **Posiciones disponibles:**
    - BTN: Button
    - SB: Small Blind
    - BB: Big Blind
    - MP: Middle Position
    - CO: Cut Off
    
    **Métricas principales:**
    - VPIP: Voluntarily Put money In Pot
    - PFR: Pre-Flop Raise
    - 3-bet: Three-bet frequency
    - WTSD: Went To ShowDown
    - AF: Aggression Factor
    - bb/100: Winrate en big blinds por 100 manos
    """
    
    # TODO: Implementar consulta a DuckDB mediante FFI
    # Por ahora retornamos datos mock para desarrollo
    
    # Validar nombre del jugador
    if not player_name or len(player_name.strip()) == 0:
        raise HTTPException(
            status_code=400,
            detail="El nombre del jugador no puede estar vacío"
        )
    
    # Validar fechas
    if start_date and end_date and end_date < start_date:
        raise HTTPException(
            status_code=400,
            detail="end_date debe ser posterior a start_date"
        )
    
    # Validar game_type
    if game_type not in ["NLHE", "PLO"]:
        raise HTTPException(
            status_code=400,
            detail="game_type debe ser 'NLHE' o 'PLO'"
        )
    
    # Mock data - en producción esto vendrá de DuckDB
    is_hero = player_name.lower() == "thesmoy"
    
    # TODO: Consultar DuckDB con filtros
    # query = build_stats_query(player_name, start_date, end_date, stake, game_type, min_hands)
    # result = execute_duckdb_query(query)
    
    # Mock response
    return PlayerStatsResponse(
        player=player_name,
        is_hero=is_hero,
        summary=PlayerStatsSummary(
            hands=1540,
            vpip=24.5,
            pfr=20.1,
            three_bet=8.2,
            wtsd=28.4,
            af=2.5,
            net_won_cents=4500,
            bb_100=15.2,
            ev_bb_100=12.8,
        ),
        positional={
            "BTN": PositionalStats(vpip=45.0, pfr=38.0, three_bet=12.0, hands=250),
            "SB": PositionalStats(vpip=32.0, pfr=28.0, three_bet=10.0, hands=250),
            "BB": PositionalStats(vpip=12.0, pfr=0.0, three_bet=5.0, hands=250),
            "MP": PositionalStats(vpip=18.0, pfr=15.0, three_bet=6.0, hands=250),
            "CO": PositionalStats(vpip=26.0, pfr=22.0, three_bet=9.0, hands=250),
        }
    )

