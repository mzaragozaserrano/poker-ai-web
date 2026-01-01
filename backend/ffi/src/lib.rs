//! # Poker FFI Bridge
//!
//! Puente FFI para exponer funciones de Rust a Python mediante PyO3.
//!
//! Este módulo proporciona bindings para:
//! - Parsing de historiales Winamax
//! - Cálculo de equity Monte Carlo
//! - Consultas a DuckDB
//!
//! ## Códigos de Error (según ffi-contract.md)
//!
//! | Código | Tipo | Descripción |
//! |--------|------|-------------|
//! | 101 | IO_ERROR | No se puede acceder al archivo |
//! | 102 | PARSER_ERROR | Formato irreconocible o corrupto |
//! | 201 | INVALID_RANGE | Error de sintaxis en rango |
//! | 202 | SIM_TIMEOUT | Simulación excedió 500ms |
//!
//! ## Uso desde Python
//!
//! ```python
//! import poker_ffi
//!
//! # Parsear archivos
//! count = poker_ffi.parse_winamax_files(["path/to/file.txt"])
//!
//! # Calcular equity
//! equity = poker_ffi.calculate_equity("AhKd", "QQ+,AKs", "Qh7s2c", 100000)
//! ```

use pyo3::exceptions::{PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

// Re-exports de crates internos
use poker_math::{calculate_equity as rust_calculate_equity, EquityResult};
use poker_parsers::{
    process_files_parallel, BatchProcessingResult, FileWatcher, ParsedHand, WatcherConfig,
};

// ============================================================================
// CÓDIGOS DE ERROR FFI (según docs/specs/ffi-contract.md)
// ============================================================================

/// Error de I/O - no se puede acceder al archivo
const ERR_IO_ERROR: i32 = 101;
/// Error de parsing - formato irreconocible
const ERR_PARSER_ERROR: i32 = 102;
/// Rango inválido - error de sintaxis
const ERR_INVALID_RANGE: i32 = 201;
/// Timeout de simulación - excedió 500ms
const ERR_SIM_TIMEOUT: i32 = 202;

/// Timeout máximo para simulaciones Monte Carlo (500ms según spec)
const SIM_TIMEOUT_MS: u64 = 500;

// ============================================================================
// ESTRUCTURAS DE DATOS PARA PYTHON
// ============================================================================

/// Resultado del parsing expuesto a Python
#[pyclass]
#[derive(Clone)]
pub struct PyParseResult {
    /// Número total de manos parseadas
    #[pyo3(get)]
    pub total_hands: usize,
    /// Número de archivos procesados exitosamente
    #[pyo3(get)]
    pub successful_files: usize,
    /// Número de archivos con errores
    #[pyo3(get)]
    pub failed_files: usize,
    /// Tiempo de procesamiento en milisegundos
    #[pyo3(get)]
    pub elapsed_ms: u64,
}

#[pymethods]
impl PyParseResult {
    fn __repr__(&self) -> String {
        format!(
            "ParseResult(hands={}, files={}, errors={}, elapsed_ms={})",
            self.total_hands, self.successful_files, self.failed_files, self.elapsed_ms
        )
    }
}

/// Resultado del cálculo de equity expuesto a Python
#[pyclass]
#[derive(Clone)]
pub struct PyEquityResult {
    /// Equity del héroe (0.0 - 1.0)
    #[pyo3(get)]
    pub hero_equity: f64,
    /// Equity del villano (0.0 - 1.0)
    #[pyo3(get)]
    pub villain_equity: f64,
    /// Probabilidad de empate (0.0 - 1.0)
    #[pyo3(get)]
    pub tie_equity: f64,
    /// Número de simulaciones ejecutadas
    #[pyo3(get)]
    pub simulations_run: u32,
    /// Si convergió antes de completar
    #[pyo3(get)]
    pub converged_early: bool,
    /// Error estándar estimado
    #[pyo3(get)]
    pub standard_error: f64,
}

#[pymethods]
impl PyEquityResult {
    fn __repr__(&self) -> String {
        format!(
            "EquityResult(hero={:.2}%, villain={:.2}%, tie={:.2}%, sims={})",
            self.hero_equity * 100.0,
            self.villain_equity * 100.0,
            self.tie_equity * 100.0,
            self.simulations_run
        )
    }

    /// Retorna la equity del héroe como porcentaje (0-100)
    fn hero_percent(&self) -> f64 {
        self.hero_equity * 100.0
    }

    /// Retorna la equity del villano como porcentaje (0-100)
    fn villain_percent(&self) -> f64 {
        self.villain_equity * 100.0
    }
}

/// Resumen de una mano parseada para Python
#[pyclass]
#[derive(Clone)]
pub struct PyHandSummary {
    /// ID único de la mano
    #[pyo3(get)]
    pub hand_id: String,
    /// Timestamp de la mano
    #[pyo3(get)]
    pub timestamp: String,
    /// Nombre de la mesa
    #[pyo3(get)]
    pub table_name: String,
    /// Número de jugadores
    #[pyo3(get)]
    pub player_count: usize,
    /// Si el héroe (thesmoy) participó
    #[pyo3(get)]
    pub hero_played: bool,
    /// Pot total en centavos
    #[pyo3(get)]
    pub total_pot_cents: i64,
}

#[pymethods]
impl PyHandSummary {
    fn __repr__(&self) -> String {
        format!(
            "HandSummary(id='{}', table='{}', players={}, hero={})",
            self.hand_id, self.table_name, self.player_count, self.hero_played
        )
    }
}

/// Estadísticas de la base de datos para Python
#[pyclass]
#[derive(Clone)]
pub struct PyDbStats {
    /// Número de jugadores
    #[pyo3(get)]
    pub player_count: i64,
    /// Número de manos
    #[pyo3(get)]
    pub hand_count: i64,
    /// Número de acciones
    #[pyo3(get)]
    pub action_count: i64,
    /// Número de sesiones de cash
    #[pyo3(get)]
    pub session_count: i64,
    /// Número de torneos
    #[pyo3(get)]
    pub tournament_count: i64,
}

#[pymethods]
impl PyDbStats {
    fn __repr__(&self) -> String {
        format!(
            "DbStats(players={}, hands={}, actions={}, sessions={}, tournaments={})",
            self.player_count,
            self.hand_count,
            self.action_count,
            self.session_count,
            self.tournament_count
        )
    }
}

// ============================================================================
// FUNCIONES FFI PRINCIPALES
// ============================================================================

/// Parsea archivos de historial Winamax en paralelo.
///
/// Utiliza Rayon con 16 threads para procesar múltiples archivos
/// simultáneamente, aprovechando el Ryzen 3800X.
///
/// # Argumentos
/// * `files` - Lista de rutas a archivos de historial
///
/// # Retorna
/// * `PyParseResult` con estadísticas del procesamiento
///
/// # Errores
/// * `IOError` (101) - Si no se puede acceder a algún archivo
/// * `RuntimeError` (102) - Si hay errores de parsing
///
/// # Ejemplo
/// ```python
/// result = parse_winamax_files(["history1.txt", "history2.txt"])
/// print(f"Parseadas {result.total_hands} manos en {result.elapsed_ms}ms")
/// ```
#[pyfunction]
#[pyo3(signature = (files))]
fn parse_winamax_files(files: Vec<String>) -> PyResult<PyParseResult> {
    if files.is_empty() {
        return Ok(PyParseResult {
            total_hands: 0,
            successful_files: 0,
            failed_files: 0,
            elapsed_ms: 0,
        });
    }

    // Convertir paths a PathBuf
    let paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();

    // Verificar que los archivos existen
    for path in &paths {
        if !path.exists() {
            return Err(PyIOError::new_err(format!(
                "[{}] No se puede acceder al archivo: {}",
                ERR_IO_ERROR,
                path.display()
            )));
        }
    }

    // Procesar en paralelo con Rayon
    let result: BatchProcessingResult = process_files_parallel(paths);

    // Verificar si hubo errores de parsing
    if result.failed_files > 0 && result.successful_files == 0 {
        return Err(PyRuntimeError::new_err(format!(
            "[{}] Error de parsing: todos los archivos fallaron",
            ERR_PARSER_ERROR
        )));
    }

    Ok(PyParseResult {
        total_hands: result.total_hands,
        successful_files: result.successful_files,
        failed_files: result.failed_files,
        elapsed_ms: result.elapsed_ms as u64,
    })
}

/// Parsea archivos y retorna resúmenes de manos.
///
/// Similar a `parse_winamax_files` pero retorna información detallada
/// de cada mano parseada.
///
/// # Argumentos
/// * `files` - Lista de rutas a archivos de historial
///
/// # Retorna
/// * Lista de `PyHandSummary` con información de cada mano
#[pyfunction]
#[pyo3(signature = (files))]
fn parse_winamax_with_details(files: Vec<String>) -> PyResult<Vec<PyHandSummary>> {
    if files.is_empty() {
        return Ok(vec![]);
    }

    let paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();

    // Verificar archivos
    for path in &paths {
        if !path.exists() {
            return Err(PyIOError::new_err(format!(
                "[{}] No se puede acceder al archivo: {}",
                ERR_IO_ERROR,
                path.display()
            )));
        }
    }

    let result = process_files_parallel(paths);

    // Convertir manos a PyHandSummary
    let summaries: Vec<PyHandSummary> = result
        .results
        .into_iter()
        .filter_map(|file_result| file_result.result.ok())
        .flat_map(|parse_result| parse_result.hands)
        .map(|hand| convert_hand_to_summary(&hand))
        .collect();

    Ok(summaries)
}

/// Calcula la equity de una mano contra otra usando Monte Carlo.
///
/// Ejecuta simulaciones Monte Carlo paralelizadas con Rayon y SIMD AVX2
/// para calcular la probabilidad de ganar.
///
/// # Argumentos
/// * `hero_cards` - Cartas del héroe (ej: "AhKd")
/// * `villain_cards` - Cartas del villano (ej: "QsQh")
/// * `board` - Cartas comunitarias (ej: "Qh7s2c" o "" para preflop)
/// * `iterations` - Número de simulaciones (default: 100000)
///
/// # Retorna
/// * `PyEquityResult` con las probabilidades calculadas
///
/// # Errores
/// * `ValueError` (201) - Si las cartas tienen formato inválido
/// * `RuntimeError` (202) - Si la simulación excede 500ms
///
/// # Ejemplo
/// ```python
/// # AA vs KK preflop
/// result = calculate_equity("AsAh", "KsKh", "", 50000)
/// print(f"AA tiene {result.hero_percent():.1f}% de equity")
/// ```
#[pyfunction]
#[pyo3(signature = (hero_cards, villain_cards, board = "", iterations = 100000))]
fn calculate_equity(
    hero_cards: &str,
    villain_cards: &str,
    board: &str,
    iterations: u32,
) -> PyResult<PyEquityResult> {
    // Validar formato de cartas
    let hero = parse_cards(hero_cards)?;
    let villain = parse_cards(villain_cards)?;
    let board_cards = if board.is_empty() {
        vec![]
    } else {
        parse_board(board)?
    };

    if hero.len() != 2 {
        return Err(PyValueError::new_err(format!(
            "[{}] Hero debe tener exactamente 2 cartas, recibido: {}",
            ERR_INVALID_RANGE,
            hero.len()
        )));
    }

    if villain.len() != 2 {
        return Err(PyValueError::new_err(format!(
            "[{}] Villain debe tener exactamente 2 cartas, recibido: {}",
            ERR_INVALID_RANGE,
            villain.len()
        )));
    }

    // Ejecutar con timeout
    let start = Instant::now();
    let timeout = Duration::from_millis(SIM_TIMEOUT_MS);

    // Convertir a slices de &str para la función Rust
    let hero_refs: Vec<&str> = hero.iter().map(|s| s.as_str()).collect();
    let villain_refs: Vec<&str> = villain.iter().map(|s| s.as_str()).collect();
    let board_refs: Vec<&str> = board_cards.iter().map(|s| s.as_str()).collect();

    // Calcular equity
    let result: EquityResult =
        rust_calculate_equity(&hero_refs, &villain_refs, &board_refs, iterations);

    // Verificar timeout
    if start.elapsed() > timeout {
        return Err(PyRuntimeError::new_err(format!(
            "[{}] Simulación excedió el tiempo límite de {}ms",
            ERR_SIM_TIMEOUT, SIM_TIMEOUT_MS
        )));
    }

    Ok(PyEquityResult {
        hero_equity: result.hero_equity,
        villain_equity: result.villain_equity,
        tie_equity: result.tie_equity,
        simulations_run: result.simulations_run,
        converged_early: result.converged_early,
        standard_error: result.standard_error,
    })
}

/// Calcula equity multiway (3+ jugadores).
///
/// # Argumentos
/// * `hands` - Lista de manos (cada una como "XxYy")
/// * `board` - Cartas comunitarias
/// * `iterations` - Número de simulaciones
///
/// # Retorna
/// * Lista de equities para cada jugador
#[pyfunction]
#[pyo3(signature = (hands, board = "", iterations = 50000))]
fn calculate_equity_multiway(
    hands: Vec<String>,
    board: &str,
    iterations: u32,
) -> PyResult<Vec<f64>> {
    if hands.len() < 2 {
        return Err(PyValueError::new_err(
            "Se necesitan al menos 2 manos para calcular equity multiway",
        ));
    }

    // Parsear todas las manos
    let parsed_hands: Vec<Vec<String>> = hands
        .iter()
        .map(|h| parse_cards(h))
        .collect::<PyResult<Vec<_>>>()?;

    let board_cards = if board.is_empty() {
        vec![]
    } else {
        parse_board(board)?
    };

    // Convertir a formato esperado por poker-math
    let hand_refs: Vec<Vec<&str>> = parsed_hands
        .iter()
        .map(|h| h.iter().map(|s| s.as_str()).collect())
        .collect();

    let hand_slices: Vec<&[&str]> = hand_refs.iter().map(|h| h.as_slice()).collect();
    let board_refs: Vec<&str> = board_cards.iter().map(|s| s.as_str()).collect();

    let result = poker_math::calculate_equity_multiway(&hand_slices, &board_refs, iterations);

    Ok(result)
}

/// Verifica si SIMD AVX2 está disponible en el sistema.
///
/// AVX2 acelera significativamente los cálculos de equity.
///
/// # Retorna
/// * `true` si AVX2 está disponible
#[pyfunction]
fn is_simd_available() -> bool {
    poker_math::is_avx2_available()
}

/// Retorna información de versión del módulo FFI.
#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Retorna información del sistema y configuración.
#[pyfunction]
fn system_info() -> PyResult<String> {
    let info = format!(
        "Poker FFI v{}\n\
         - SIMD AVX2: {}\n\
         - Threads disponibles: {}\n\
         - Timeout simulación: {}ms",
        env!("CARGO_PKG_VERSION"),
        if poker_math::is_avx2_available() {
            "Habilitado"
        } else {
            "No disponible"
        },
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1),
        SIM_TIMEOUT_MS
    );
    Ok(info)
}

// ============================================================================
// FUNCIONES AUXILIARES
// ============================================================================

/// Parsea un string de cartas (ej: "AhKd") a vector de cartas individuales
fn parse_cards(cards_str: &str) -> PyResult<Vec<String>> {
    let trimmed = cards_str.trim();
    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    // Validar longitud (debe ser múltiplo de 2)
    if trimmed.len() % 2 != 0 {
        return Err(PyValueError::new_err(format!(
            "[{}] Formato de cartas inválido: '{}'. Debe ser pares de RankSuit (ej: AhKd)",
            ERR_INVALID_RANGE, cards_str
        )));
    }

    let cards: Vec<String> = trimmed
        .as_bytes()
        .chunks(2)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    // Validar cada carta
    for card in &cards {
        if !is_valid_card(card) {
            return Err(PyValueError::new_err(format!(
                "[{}] Carta inválida: '{}'. Formato: RankSuit (ej: Ah, Kd, 2c)",
                ERR_INVALID_RANGE, card
            )));
        }
    }

    Ok(cards)
}

/// Parsea el board (puede tener 0, 3, 4 o 5 cartas)
fn parse_board(board_str: &str) -> PyResult<Vec<String>> {
    let cards = parse_cards(board_str)?;

    if !cards.is_empty() && cards.len() != 3 && cards.len() != 4 && cards.len() != 5 {
        return Err(PyValueError::new_err(format!(
            "[{}] Board debe tener 0, 3, 4 o 5 cartas, recibido: {}",
            ERR_INVALID_RANGE,
            cards.len()
        )));
    }

    Ok(cards)
}

/// Valida si una carta tiene formato correcto (ej: "Ah", "Kd", "2c")
fn is_valid_card(card: &str) -> bool {
    if card.len() != 2 {
        return false;
    }

    let chars: Vec<char> = card.chars().collect();
    let rank = chars[0];
    let suit = chars[1];

    let valid_ranks = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let valid_suits = ['h', 'd', 'c', 's'];

    valid_ranks.contains(&rank) && valid_suits.contains(&suit)
}

/// Convierte una mano parseada a PyHandSummary
fn convert_hand_to_summary(hand: &ParsedHand) -> PyHandSummary {
    let hero_played = hand.players.iter().any(|p| p.name == "thesmoy");

    PyHandSummary {
        hand_id: hand.hand_id.clone(),
        timestamp: hand.timestamp.clone(),
        table_name: hand.table_name.clone(),
        player_count: hand.players.len(),
        hero_played,
        total_pot_cents: hand.pot.total_cents,
    }
}

// ============================================================================
// FILE WATCHER CON CALLBACK PYTHON
// ============================================================================

/// Configuración del file watcher expuesta a Python
#[pyclass]
#[derive(Clone)]
pub struct PyWatcherConfig {
    /// Ruta del directorio a monitorear
    #[pyo3(get, set)]
    pub watch_path: String,
    /// Número máximo de reintentos para archivos bloqueados
    #[pyo3(get, set)]
    pub max_retries: u32,
    /// Delay inicial para retry en milisegundos
    #[pyo3(get, set)]
    pub retry_delay_ms: u64,
}

#[pymethods]
impl PyWatcherConfig {
    #[new]
    #[pyo3(signature = (watch_path, max_retries = 3, retry_delay_ms = 100))]
    fn new(watch_path: String, max_retries: u32, retry_delay_ms: u64) -> Self {
        Self {
            watch_path,
            max_retries,
            retry_delay_ms,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "WatcherConfig(path='{}', retries={}, delay={}ms)",
            self.watch_path, self.max_retries, self.retry_delay_ms
        )
    }
}

/// Evento de archivo detectado expuesto a Python
#[pyclass]
#[derive(Clone)]
pub struct PyFileEvent {
    /// Ruta del archivo
    #[pyo3(get)]
    pub path: String,
    /// Hash MD5 del archivo
    #[pyo3(get)]
    pub hash: String,
    /// Timestamp UNIX de detección
    #[pyo3(get)]
    pub timestamp_secs: u64,
}

#[pymethods]
impl PyFileEvent {
    fn __repr__(&self) -> String {
        format!(
            "FileEvent(path='{}', hash='{}', ts={})",
            self.path, self.hash, self.timestamp_secs
        )
    }
}

/// Inicia el file watcher con un callback Python.
///
/// Esta función es NO bloqueante. Inicia el watcher en un thread separado
/// y retorna inmediatamente. El callback se ejecutará en un thread de Rust
/// cuando se detecten nuevos archivos.
///
/// # Argumentos
/// * `config` - Configuración del watcher
/// * `callback` - Función Python que se llamará cuando se detecten archivos
///
/// # Callback Signature
/// ```python
/// def on_new_file(event: PyFileEvent) -> None:
///     print(f"Nuevo archivo: {event.path}")
/// ```
///
/// # Ejemplo
/// ```python
/// def on_file_detected(event):
///     print(f"Detectado: {event.path}")
///
/// config = PyWatcherConfig("/path/to/history")
/// start_file_watcher(config, on_file_detected)
/// ```
///
/// # Nota
/// Esta función NO bloquea el hilo principal de Python. El watcher corre
/// en background. Para detenerlo, el proceso debe terminarse.
#[pyfunction]
#[pyo3(signature = (config, callback))]
fn start_file_watcher(config: PyWatcherConfig, callback: PyObject) -> PyResult<()> {
    use std::thread;

    let watch_path = PathBuf::from(&config.watch_path);

    // Verificar que el directorio existe
    if !watch_path.exists() {
        return Err(PyIOError::new_err(format!(
            "[{}] El directorio no existe: {}",
            ERR_IO_ERROR, config.watch_path
        )));
    }

    // Crear configuración Rust
    let rust_config = WatcherConfig {
        watch_path: watch_path.clone(),
        max_retries: config.max_retries,
        retry_delay_ms: config.retry_delay_ms,
        use_exponential_backoff: true,
    };

    // Iniciar watcher en thread separado
    thread::spawn(move || {
        let watcher = FileWatcher::new(rust_config);

        // Callback que se ejecuta cuando se detecta un archivo
        let result = watcher.start(move |file_path| {
            // Crear evento Python
            let py_event = PyFileEvent {
                path: file_path.to_string_lossy().to_string(),
                hash: "".to_string(), // El hash está interno en el watcher
                timestamp_secs: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            // Llamar al callback Python
            Python::with_gil(|py| {
                if let Err(e) = callback.call1(py, (py_event,)) {
                    eprintln!("Error en callback Python: {:?}", e);
                }
            });
        });

        if let Err(e) = result {
            eprintln!("Error en file watcher: {:?}", e);
        }
    });

    Ok(())
}

/// Inicia el file watcher con auto-procesamiento.
///
/// Similar a `start_file_watcher` pero procesa automáticamente los archivos
/// usando el parser de Rust y llama al callback con los resúmenes de manos.
///
/// # Argumentos
/// * `config` - Configuración del watcher
/// * `callback` - Función Python que recibe lista de PyHandSummary
///
/// # Callback Signature
/// ```python
/// def on_hands_parsed(hands: List[PyHandSummary]) -> None:
///     for hand in hands:
///         print(f"Hand {hand.hand_id}: {hand.hero_played}")
/// ```
#[pyfunction]
#[pyo3(signature = (config, callback))]
fn start_file_watcher_with_parsing(config: PyWatcherConfig, callback: PyObject) -> PyResult<()> {
    use std::thread;

    let watch_path = PathBuf::from(&config.watch_path);

    if !watch_path.exists() {
        return Err(PyIOError::new_err(format!(
            "[{}] El directorio no existe: {}",
            ERR_IO_ERROR, config.watch_path
        )));
    }

    let rust_config = WatcherConfig {
        watch_path: watch_path.clone(),
        max_retries: config.max_retries,
        retry_delay_ms: config.retry_delay_ms,
        use_exponential_backoff: true,
    };

    thread::spawn(move || {
        let watcher = FileWatcher::new(rust_config);

        let result = watcher.start(move |file_path| {
            // Parsear el archivo detectado
            let result = process_files_parallel(vec![file_path]);

            // Convertir a PyHandSummary
            let summaries: Vec<PyHandSummary> = result
                .results
                .into_iter()
                .filter_map(|file_result| file_result.result.ok())
                .flat_map(|parse_result| parse_result.hands)
                .map(|hand| convert_hand_to_summary(&hand))
                .collect();

            // Llamar al callback Python con las manos
            Python::with_gil(|py| {
                if let Err(e) = callback.call1(py, (summaries,)) {
                    eprintln!("Error en callback Python: {:?}", e);
                }
            });
        });

        if let Err(e) = result {
            eprintln!("Error en file watcher: {:?}", e);
        }
    });

    Ok(())
}

// ============================================================================
// MÓDULO PYTHON
// ============================================================================

/// Módulo Python para FFI con Rust.
///
/// Expone funciones de alto rendimiento para:
/// - Parsing de historiales Winamax (16 threads)
/// - Cálculo de equity Monte Carlo (SIMD AVX2)
/// - Consultas a DuckDB
/// - File watching para detección automática de nuevos archivos
#[pymodule]
fn poker_ffi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Funciones de parsing
    m.add_function(wrap_pyfunction!(parse_winamax_files, m)?)?;
    m.add_function(wrap_pyfunction!(parse_winamax_with_details, m)?)?;

    // Funciones de equity
    m.add_function(wrap_pyfunction!(calculate_equity, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_equity_multiway, m)?)?;

    // File watcher
    m.add_function(wrap_pyfunction!(start_file_watcher, m)?)?;
    m.add_function(wrap_pyfunction!(start_file_watcher_with_parsing, m)?)?;

    // Utilidades
    m.add_function(wrap_pyfunction!(is_simd_available, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(system_info, m)?)?;

    // Clases
    m.add_class::<PyParseResult>()?;
    m.add_class::<PyEquityResult>()?;
    m.add_class::<PyHandSummary>()?;
    m.add_class::<PyDbStats>()?;
    m.add_class::<PyWatcherConfig>()?;
    m.add_class::<PyFileEvent>()?;

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards_valid() {
        let cards = parse_cards("AhKd").unwrap();
        assert_eq!(cards, vec!["Ah", "Kd"]);
    }

    #[test]
    fn test_parse_cards_empty() {
        let cards = parse_cards("").unwrap();
        assert!(cards.is_empty());
    }

    #[test]
    fn test_parse_cards_invalid_length() {
        let result = parse_cards("AhK");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_cards_invalid_card() {
        let result = parse_cards("XhKd");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_card() {
        assert!(is_valid_card("Ah"));
        assert!(is_valid_card("Kd"));
        assert!(is_valid_card("2c"));
        assert!(is_valid_card("Ts"));
        assert!(!is_valid_card("Xh"));
        assert!(!is_valid_card("Ax"));
        assert!(!is_valid_card("A"));
    }

    #[test]
    fn test_parse_board_valid() {
        let board = parse_board("Ah7s2c").unwrap();
        assert_eq!(board.len(), 3);

        let board = parse_board("Ah7s2cKd").unwrap();
        assert_eq!(board.len(), 4);

        let board = parse_board("Ah7s2cKdQh").unwrap();
        assert_eq!(board.len(), 5);
    }

    #[test]
    fn test_parse_board_invalid_count() {
        let result = parse_board("AhKd");
        assert!(result.is_err());
    }
}
