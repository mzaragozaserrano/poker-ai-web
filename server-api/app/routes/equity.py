"""
Router para endpoints de cálculo de equity.
"""

from time import time
from fastapi import APIRouter, HTTPException
from app.models.equity import (
    EquityCalculateRequest,
    EquityCalculateResponse,
    EquityMultiwayRequest,
)
from app.bridge import calculate_equity, calculate_equity_multiway, is_ffi_available

router = APIRouter(prefix="/api/v1/equity", tags=["equity"])


@router.post("/calculate", response_model=EquityCalculateResponse)
async def calculate_hand_equity(request: EquityCalculateRequest) -> EquityCalculateResponse:
    """
    Calcula la equity de una mano contra otra usando simulación Monte Carlo.
    
    **Algoritmo:**
    - Monte Carlo paralelizado con Rayon (16 threads)
    - Optimización SIMD AVX2 para evaluación de manos
    - Early stopping con convergencia < 0.1%
    
    **Performance esperado:**
    - 100,000 simulaciones: ~100ms
    - Aprovecha los 16 hilos del Ryzen 3800X
    
    **Formato de cartas:**
    - Rank: A, K, Q, J, T, 9-2
    - Suit: h (hearts), d (diamonds), c (clubs), s (spades)
    - Ejemplo: "AhKd" = As de corazones + Rey de diamantes
    
    **Board:**
    - Vacío ("") para preflop
    - 3 cartas para flop: "Qh7s2c"
    - 4 cartas para turn: "Qh7s2cKd"
    - 5 cartas para river: "Qh7s2cKd3h"
    
    **Ejemplos de uso:**
    ```
    # AA vs KK preflop
    POST /api/v1/equity/calculate
    {
      "hero_cards": "AsAh",
      "villain_cards": "KsKh",
      "board": "",
      "iterations": 50000
    }
    
    # Top pair vs overpair en flop
    POST /api/v1/equity/calculate
    {
      "hero_cards": "AhKh",
      "villain_cards": "QsQd",
      "board": "Kd7c2h",
      "iterations": 100000
    }
    ```
    """
    
    # Verificar que el módulo FFI esté disponible
    if not is_ffi_available():
        raise HTTPException(
            status_code=503,
            detail={
                "error": "FFI_MODULE_UNAVAILABLE",
                "message": "El módulo de cálculo nativo no está disponible. "
                           "Ejecutar 'maturin develop' en backend/ffi.",
            }
        )
    
    try:
        # Medir tiempo de ejecución
        start_time = time()
        
        # Llamar al módulo Rust via FFI
        result = calculate_equity(
            hero_cards=request.hero_cards,
            villain_cards=request.villain_cards,
            board=request.board,
            iterations=request.iterations,
        )
        
        elapsed_ms = int((time() - start_time) * 1000)
        
        # Construir response
        return EquityCalculateResponse(
            hero_equity=result["hero_equity"],
            villain_equity=result["villain_equity"],
            tie_equity=result["tie_equity"],
            hero_percent=result["hero_percent"],
            villain_percent=result["villain_percent"],
            simulations_run=result["simulations_run"],
            converged_early=result["converged_early"],
            standard_error=result["standard_error"],
            elapsed_ms=elapsed_ms,
        )
    
    except ValueError as e:
        # Error de validación de cartas (código 201 del FFI)
        raise HTTPException(
            status_code=400,
            detail={
                "error": "INVALID_CARDS",
                "message": str(e),
            }
        )
    
    except RuntimeError as e:
        # Error de timeout o simulación (código 202)
        error_msg = str(e)
        if "excedió" in error_msg.lower() or "timeout" in error_msg.lower():
            raise HTTPException(
                status_code=408,  # Request Timeout
                detail={
                    "error": "SIMULATION_TIMEOUT",
                    "message": "La simulación excedió el tiempo límite de 500ms. "
                               "Intente con menos iteraciones.",
                }
            )
        else:
            raise HTTPException(
                status_code=500,
                detail={
                    "error": "SIMULATION_ERROR",
                    "message": f"Error en la simulación: {error_msg}",
                }
            )
    
    except Exception as e:
        # Error inesperado
        raise HTTPException(
            status_code=500,
            detail={
                "error": "INTERNAL_ERROR",
                "message": f"Error interno del servidor: {str(e)}",
            }
        )


@router.post("/calculate/multiway", response_model=dict)
async def calculate_multiway_equity(request: EquityMultiwayRequest) -> dict:
    """
    Calcula equity para 3 o más jugadores (multiway pot).
    
    **Uso:**
    - Mínimo 2 manos, máximo 10
    - Retorna un array con la equity de cada jugador
    - La suma de todas las equities es 1.0
    
    **Performance:**
    - 3 jugadores: ~150ms (50,000 iteraciones)
    - El tiempo aumenta linealmente con el número de jugadores
    
    **Ejemplo:**
    ```
    POST /api/v1/equity/calculate/multiway
    {
      "hands": ["AhKd", "QsQh", "8c8d"],
      "board": "",
      "iterations": 50000
    }
    
    Response:
    {
      "equities": [0.35, 0.42, 0.23],
      "simulations_run": 50000,
      "elapsed_ms": 145
    }
    ```
    """
    
    if not is_ffi_available():
        raise HTTPException(
            status_code=503,
            detail={
                "error": "FFI_MODULE_UNAVAILABLE",
                "message": "El módulo de cálculo nativo no está disponible.",
            }
        )
    
    try:
        start_time = time()
        
        # Llamar al módulo Rust
        equities = calculate_equity_multiway(
            hands=request.hands,
            board=request.board,
            iterations=request.iterations,
        )
        
        elapsed_ms = int((time() - start_time) * 1000)
        
        return {
            "equities": equities,
            "simulations_run": request.iterations,
            "elapsed_ms": elapsed_ms,
        }
    
    except ValueError as e:
        raise HTTPException(
            status_code=400,
            detail={
                "error": "INVALID_INPUT",
                "message": str(e),
            }
        )
    
    except Exception as e:
        raise HTTPException(
            status_code=500,
            detail={
                "error": "INTERNAL_ERROR",
                "message": str(e),
            }
        )

