"""
Bridge module for Rust FFI integration.

Este módulo proporciona la interfaz entre Python/FastAPI y el núcleo Rust
mediante PyO3. Las funciones nativas de alto rendimiento se exponen aquí
para ser consumidas por los endpoints de la API.

Dependencias:
- poker_ffi: Módulo nativo compilado con maturin desde backend/ffi

Uso:
    from app.bridge import parse_files, calculate_equity

    # Parsear archivos
    result = parse_files(["path/to/history.txt"])

    # Calcular equity
    equity = calculate_equity("AhKd", "QsQh", "", 100000)
"""

from typing import List


def _get_ffi_module():
    """
    Importa el módulo FFI nativo.
    
    Intenta importar poker_ffi. Si no está disponible (módulo no compilado),
    retorna None y las funciones usarán fallbacks o lanzarán errores claros.
    """
    try:
        import poker_ffi
        return poker_ffi
    except ImportError:
        return None


# Intentar cargar el módulo FFI al iniciar
_ffi = _get_ffi_module()


def is_ffi_available() -> bool:
    """Verifica si el módulo FFI nativo está disponible."""
    return _ffi is not None


def parse_files(files: List[str]) -> dict:
    """
    Parsea archivos de historial Winamax usando el parser Rust.
    
    Args:
        files: Lista de rutas a archivos de historial
        
    Returns:
        dict con:
            - total_hands: Número de manos parseadas
            - successful_files: Archivos procesados exitosamente
            - failed_files: Archivos con errores
            - elapsed_ms: Tiempo de procesamiento
            
    Raises:
        RuntimeError: Si el módulo FFI no está disponible
        IOError: Si no se puede acceder a algún archivo
    """
    if _ffi is None:
        raise RuntimeError(
            "Módulo FFI no disponible. "
            "Ejecutar 'maturin develop' en backend/ffi para compilar."
        )
    
    result = _ffi.parse_winamax_files(files)
    return {
        "total_hands": result.total_hands,
        "successful_files": result.successful_files,
        "failed_files": result.failed_files,
        "elapsed_ms": result.elapsed_ms,
    }


def parse_files_with_details(files: List[str]) -> List[dict]:
    """
    Parsea archivos y retorna detalles de cada mano.
    
    Args:
        files: Lista de rutas a archivos
        
    Returns:
        Lista de dicts con información de cada mano:
            - hand_id: ID único de la mano
            - timestamp: Fecha/hora
            - table_name: Nombre de la mesa
            - player_count: Número de jugadores
            - hero_played: Si thesmoy participó
            - total_pot_cents: Pot total en centavos
    """
    if _ffi is None:
        raise RuntimeError("Módulo FFI no disponible")
    
    summaries = _ffi.parse_winamax_with_details(files)
    return [
        {
            "hand_id": s.hand_id,
            "timestamp": s.timestamp,
            "table_name": s.table_name,
            "player_count": s.player_count,
            "hero_played": s.hero_played,
            "total_pot_cents": s.total_pot_cents,
        }
        for s in summaries
    ]


def calculate_equity(
    hero_cards: str,
    villain_cards: str,
    board: str = "",
    iterations: int = 100000
) -> dict:
    """
    Calcula equity usando simulación Monte Carlo en Rust.
    
    Args:
        hero_cards: Cartas del héroe (ej: "AhKd")
        villain_cards: Cartas del villano (ej: "QsQh")
        board: Cartas comunitarias (ej: "Qh7s2c")
        iterations: Número de simulaciones
        
    Returns:
        dict con:
            - hero_equity: Probabilidad de ganar (0.0 - 1.0)
            - villain_equity: Probabilidad de perder
            - tie_equity: Probabilidad de empate
            - simulations_run: Simulaciones ejecutadas
            - converged_early: Si convergió temprano
            - standard_error: Error estándar
    """
    if _ffi is None:
        raise RuntimeError("Módulo FFI no disponible")
    
    result = _ffi.calculate_equity(hero_cards, villain_cards, board, iterations)
    return {
        "hero_equity": result.hero_equity,
        "villain_equity": result.villain_equity,
        "tie_equity": result.tie_equity,
        "simulations_run": result.simulations_run,
        "converged_early": result.converged_early,
        "standard_error": result.standard_error,
        "hero_percent": result.hero_equity * 100.0,
        "villain_percent": result.villain_equity * 100.0,
    }


def calculate_equity_multiway(
    hands: List[str],
    board: str = "",
    iterations: int = 50000
) -> List[float]:
    """
    Calcula equity multiway (3+ jugadores).
    
    Args:
        hands: Lista de manos (cada una como "XxYy")
        board: Cartas comunitarias
        iterations: Número de simulaciones
        
    Returns:
        Lista de equities para cada jugador
    """
    if _ffi is None:
        raise RuntimeError("Módulo FFI no disponible")
    
    return _ffi.calculate_equity_multiway(hands, board, iterations)


def is_simd_available() -> bool:
    """Verifica si SIMD AVX2 está disponible."""
    if _ffi is None:
        return False
    return _ffi.is_simd_available()


def get_version() -> str:
    """Retorna la versión del módulo FFI."""
    if _ffi is None:
        return "FFI no disponible"
    return _ffi.version()


def get_system_info() -> str:
    """Retorna información del sistema y configuración FFI."""
    if _ffi is None:
        return "Módulo FFI no compilado. Ejecutar 'maturin develop' en backend/ffi."
    return _ffi.system_info()


# Exports
__all__ = [
    "is_ffi_available",
    "parse_files",
    "parse_files_with_details", 
    "calculate_equity",
    "calculate_equity_multiway",
    "is_simd_available",
    "get_version",
    "get_system_info",
]
