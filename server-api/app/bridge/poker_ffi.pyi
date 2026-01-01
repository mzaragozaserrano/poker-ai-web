"""
Type stubs para el módulo poker_ffi (Rust FFI via PyO3).

Este archivo proporciona type hints para IDEs y herramientas de análisis
estático como mypy, pyright, etc.

El módulo real es compilado desde Rust con PyO3/maturin.
"""

from typing import List, Callable


class PyParseResult:
    """Resultado del parsing de archivos Winamax."""
    
    total_hands: int
    """Número total de manos parseadas."""
    
    successful_files: int
    """Número de archivos procesados exitosamente."""
    
    failed_files: int
    """Número de archivos con errores."""
    
    elapsed_ms: int
    """Tiempo de procesamiento en milisegundos."""
    
    def __repr__(self) -> str: ...


class PyEquityResult:
    """Resultado del cálculo de equity Monte Carlo."""
    
    hero_equity: float
    """Equity del héroe (0.0 - 1.0)."""
    
    villain_equity: float
    """Equity del villano (0.0 - 1.0)."""
    
    tie_equity: float
    """Probabilidad de empate (0.0 - 1.0)."""
    
    simulations_run: int
    """Número de simulaciones ejecutadas."""
    
    converged_early: bool
    """Si convergió antes de completar todas las simulaciones."""
    
    standard_error: float
    """Error estándar estimado de la equity."""
    
    def __repr__(self) -> str: ...
    
    def hero_percent(self) -> float:
        """Retorna la equity del héroe como porcentaje (0-100)."""
        ...
    
    def villain_percent(self) -> float:
        """Retorna la equity del villano como porcentaje (0-100)."""
        ...


class PyHandSummary:
    """Resumen de una mano parseada."""
    
    hand_id: str
    """ID único de la mano."""
    
    timestamp: str
    """Timestamp de la mano."""
    
    table_name: str
    """Nombre de la mesa."""
    
    player_count: int
    """Número de jugadores."""
    
    hero_played: bool
    """Si el héroe (thesmoy) participó."""
    
    total_pot_cents: int
    """Pot total en centavos."""
    
    def __repr__(self) -> str: ...


class PyDbStats:
    """Estadísticas de la base de datos."""
    
    player_count: int
    """Número de jugadores."""
    
    hand_count: int
    """Número de manos."""
    
    action_count: int
    """Número de acciones."""
    
    session_count: int
    """Número de sesiones de cash."""
    
    tournament_count: int
    """Número de torneos."""
    
    def __repr__(self) -> str: ...


class PyWatcherConfig:
    """Configuración del file watcher."""
    
    watch_path: str
    """Ruta del directorio a monitorear."""
    
    max_retries: int
    """Número máximo de reintentos para archivos bloqueados."""
    
    retry_delay_ms: int
    """Delay inicial para retry en milisegundos."""
    
    def __init__(
        self,
        watch_path: str,
        max_retries: int = 3,
        retry_delay_ms: int = 100
    ) -> None: ...
    
    def __repr__(self) -> str: ...


class PyFileEvent:
    """Evento de archivo detectado por el watcher."""
    
    path: str
    """Ruta del archivo detectado."""
    
    hash: str
    """Hash MD5 del archivo."""
    
    timestamp_secs: int
    """Timestamp UNIX de detección."""
    
    def __repr__(self) -> str: ...


# Funciones de parsing

def parse_winamax_files(files: List[str]) -> PyParseResult:
    """
    Parsea archivos de historial Winamax en paralelo.
    
    Utiliza Rayon con 16 threads para procesar múltiples archivos
    simultáneamente.
    
    Args:
        files: Lista de rutas a archivos de historial
        
    Returns:
        PyParseResult con estadísticas del procesamiento
        
    Raises:
        IOError: Si no se puede acceder a algún archivo (código 101)
        RuntimeError: Si hay errores de parsing (código 102)
    """
    ...


def parse_winamax_with_details(files: List[str]) -> List[PyHandSummary]:
    """
    Parsea archivos y retorna resúmenes de manos.
    
    Args:
        files: Lista de rutas a archivos de historial
        
    Returns:
        Lista de PyHandSummary con información de cada mano
    """
    ...


# Funciones de equity

def calculate_equity(
    hero_cards: str,
    villain_cards: str,
    board: str = "",
    iterations: int = 100000
) -> PyEquityResult:
    """
    Calcula la equity de una mano contra otra usando Monte Carlo.
    
    Args:
        hero_cards: Cartas del héroe (ej: "AhKd")
        villain_cards: Cartas del villano (ej: "QsQh")
        board: Cartas comunitarias (ej: "Qh7s2c")
        iterations: Número de simulaciones
        
    Returns:
        PyEquityResult con las probabilidades calculadas
        
    Raises:
        ValueError: Si las cartas tienen formato inválido (código 201)
        RuntimeError: Si la simulación excede 500ms (código 202)
    """
    ...


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
    ...


# Funciones de utilidad

def is_simd_available() -> bool:
    """
    Verifica si SIMD AVX2 está disponible en el sistema.
    
    Returns:
        True si AVX2 está disponible
    """
    ...


def version() -> str:
    """
    Retorna la versión del módulo FFI.
    
    Returns:
        String con la versión (ej: "0.1.0")
    """
    ...


def system_info() -> str:
    """
    Retorna información del sistema y configuración FFI.
    
    Returns:
        String con información detallada del sistema
    """
    ...


# Funciones de file watcher

def start_file_watcher(
    config: PyWatcherConfig,
    callback: Callable[[PyFileEvent], None]
) -> None:
    """
    Inicia el file watcher con un callback Python.
    
    Esta función NO bloquea. Inicia el watcher en un thread separado
    y retorna inmediatamente. El callback se ejecutará cuando se detecten
    nuevos archivos.
    
    Args:
        config: Configuración del watcher
        callback: Función que se llamará con cada archivo detectado
        
    Raises:
        IOError: Si el directorio no existe (código 101)
        
    Example:
        >>> def on_file(event: PyFileEvent):
        ...     print(f"Detectado: {event.path}")
        >>> config = PyWatcherConfig("/path/to/history")
        >>> start_file_watcher(config, on_file)
    """
    ...


def start_file_watcher_with_parsing(
    config: PyWatcherConfig,
    callback: Callable[[List[PyHandSummary]], None]
) -> None:
    """
    Inicia el file watcher con auto-procesamiento.
    
    Similar a start_file_watcher pero procesa automáticamente los archivos
    usando el parser de Rust y llama al callback con los resúmenes de manos.
    
    Args:
        config: Configuración del watcher
        callback: Función que recibe lista de PyHandSummary
        
    Raises:
        IOError: Si el directorio no existe (código 101)
        
    Example:
        >>> def on_hands(hands: List[PyHandSummary]):
        ...     for hand in hands:
        ...         print(f"Hand {hand.hand_id}")
        >>> config = PyWatcherConfig("/path/to/history")
        >>> start_file_watcher_with_parsing(config, on_hands)
    """
    ...

