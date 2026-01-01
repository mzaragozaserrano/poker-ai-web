"""
Router para endpoints de consulta de manos.
"""

from typing import List, Optional
from datetime import datetime
from fastapi import APIRouter, HTTPException, Query
from app.models.hands import (
    HandSummaryResponse,
    HandDetailResponse,
    HandAction,
    HandPlayer,
)

router = APIRouter(prefix="/api/v1/hands", tags=["hands"])


@router.get("/recent", response_model=List[HandSummaryResponse])
async def get_recent_hands(
    limit: int = Query(50, ge=1, le=500, description="Número máximo de manos"),
    offset: int = Query(0, ge=0, description="Offset para paginación"),
    start_date: Optional[datetime] = Query(None, description="Fecha inicio"),
    end_date: Optional[datetime] = Query(None, description="Fecha fin"),
    stake: Optional[str] = Query(None, description="Nivel de ciegas"),
    hero_only: bool = Query(False, description="Solo manos donde participó thesmoy"),
) -> List[HandSummaryResponse]:
    """
    Obtiene las últimas N manos jugadas.
    
    Retorna un listado de manos ordenadas por timestamp descendente.
    Se puede filtrar por fecha, stakes y si el héroe participó.
    
    **Paginación:**
    - Usar `limit` y `offset` para paginar resultados
    - Máximo 500 manos por request
    
    **Filtros:**
    - `start_date` / `end_date`: Rango de fechas
    - `stake`: Filtrar por nivel (ej: 'NL10')
    - `hero_only`: Solo manos donde 'thesmoy' participó
    """
    
    # Validar fechas
    if start_date and end_date and end_date < start_date:
        raise HTTPException(
            status_code=400,
            detail="end_date debe ser posterior a start_date"
        )
    
    # TODO: Implementar consulta a DuckDB
    # query = build_hands_query(limit, offset, start_date, end_date, stake, hero_only)
    # result = execute_duckdb_query(query)
    
    # Mock data
    mock_hands = [
        HandSummaryResponse(
            hand_id=f"20231215-1234567{i}",
            timestamp=datetime(2023, 12, 15, 14, 30 + i),
            table_name=f"Monaco #{i}",
            player_count=6,
            hero_played=i % 2 == 0,
            total_pot_cents=1000 + (i * 100),
            stake="NL10",
        )
        for i in range(min(limit, 10))  # Limitar mock a 10 manos
    ]
    
    # Aplicar filtro hero_only en mock
    if hero_only:
        mock_hands = [h for h in mock_hands if h.hero_played]
    
    return mock_hands


@router.get("/{hand_id}", response_model=HandDetailResponse)
async def get_hand_detail(hand_id: str) -> HandDetailResponse:
    """
    Obtiene el detalle completo de una mano específica.
    
    Retorna:
    - Información de la mesa y configuración
    - Lista de jugadores con posiciones y stacks
    - Secuencia completa de acciones por street
    - Board cards (si hay showdown)
    - Pot total y rake
    
    **Formato hand_id:**
    - Formato: "YYYYMMDD-XXXXXXXX"
    - Ejemplo: "20231215-12345678"
    """
    
    # Validar formato del hand_id
    if not hand_id or len(hand_id) < 10:
        raise HTTPException(
            status_code=400,
            detail="hand_id inválido. Formato esperado: YYYYMMDD-XXXXXXXX"
        )
    
    # TODO: Implementar consulta a DuckDB
    # hand = get_hand_from_db(hand_id)
    # if not hand:
    #     raise HTTPException(404, f"Mano no encontrada: {hand_id}")
    
    # Mock data
    # Por ahora retornamos un 404 para cualquier ID que no sea el ejemplo
    if hand_id != "20231215-12345678":
        raise HTTPException(
            status_code=404,
            detail=f"Mano no encontrada: {hand_id}"
        )
    
    return HandDetailResponse(
        hand_id=hand_id,
        timestamp=datetime(2023, 12, 15, 14, 30, 0),
        table_name="Monaco",
        stake="NL10",
        button_position=3,
        players=[
            HandPlayer(
                name="thesmoy",
                position="BTN",
                stack_cents=1000,
                cards=["Ah", "Kd"]
            ),
            HandPlayer(
                name="Villain1",
                position="SB",
                stack_cents=950,
                cards=None
            ),
            HandPlayer(
                name="Villain2",
                position="BB",
                stack_cents=1100,
                cards=None
            ),
        ],
        actions=[
            HandAction(
                player="Villain1",
                street="Preflop",
                action_type="Call",
                amount_cents=5,
                is_hero=False
            ),
            HandAction(
                player="Villain2",
                street="Preflop",
                action_type="Check",
                amount_cents=0,
                is_hero=False
            ),
            HandAction(
                player="thesmoy",
                street="Preflop",
                action_type="Raise",
                amount_cents=30,
                is_hero=True
            ),
        ],
        board=["Kh", "7s", "2c", "Qd", "3h"],
        total_pot_cents=1250,
        rake_cents=50,
    )

