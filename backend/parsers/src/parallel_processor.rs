//! Procesamiento paralelo de archivos de historial usando Rayon.
//!
//! Este módulo implementa la paralelización de la ingesta masiva de historiales
//! aprovechando los 16 hilos del Ryzen 3800X mediante Rayon.
//!
//! ## Arquitectura
//!
//! - **ThreadPool personalizado**: 16 hilos con stack size de 128KB
//! - **Granularidad de archivo**: Cada hilo procesa un archivo completo
//! - **Sincronización**: Resultados agregados mediante canales o Arc<Mutex>
//! - **Progreso**: Contador atómico para reportar avance
//!
//! ## Uso
//!
//! ```rust,no_run
//! use poker_parsers::parallel_processor::{ParallelProcessor, ProcessingConfig, ProcessingProgress};
//! use std::path::PathBuf;
//!
//! let files: Vec<PathBuf> = vec!["history1.txt".into(), "history2.txt".into()];
//! let processor = ParallelProcessor::new(ProcessingConfig::default());
//!
//! let results = processor.process_files(files, Some(|progress: ProcessingProgress| {
//!     println!("Procesados: {}/{}", progress.completed, progress.total);
//! }));
//! ```

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use crate::file_reader::{read_file_optimized, FileContent};
use crate::types::ParseResult;
use crate::WinamaxParser;

/// Número de hilos por defecto (optimizado para Ryzen 3800X).
const DEFAULT_NUM_THREADS: usize = 16;

/// Tamaño de stack por hilo (128KB para parsing profundo).
const DEFAULT_STACK_SIZE: usize = 128 * 1024;

/// Configuración del procesador paralelo.
#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    /// Número de hilos a utilizar.
    pub num_threads: usize,
    /// Tamaño de stack por hilo en bytes.
    pub stack_size: usize,
    /// Nombre del pool de hilos (para debugging).
    pub thread_name_prefix: String,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            num_threads: DEFAULT_NUM_THREADS,
            stack_size: DEFAULT_STACK_SIZE,
            thread_name_prefix: "poker-parser".to_string(),
        }
    }
}

impl ProcessingConfig {
    /// Crea una configuración con el número de hilos especificado.
    pub fn with_threads(num_threads: usize) -> Self {
        Self {
            num_threads,
            ..Default::default()
        }
    }

    /// Crea una configuración usando todos los cores disponibles.
    pub fn auto() -> Self {
        Self {
            num_threads: num_cpus(),
            ..Default::default()
        }
    }
}

/// Información de progreso del procesamiento.
#[derive(Debug, Clone)]
pub struct ProcessingProgress {
    /// Número de archivos completados.
    pub completed: usize,
    /// Total de archivos a procesar.
    pub total: usize,
    /// Número de archivos con errores.
    pub errors: usize,
}

/// Resultado del procesamiento de un archivo individual.
#[derive(Debug)]
pub struct FileProcessingResult {
    /// Ruta del archivo procesado.
    pub path: PathBuf,
    /// Resultado del parsing (Ok con ParseResult o Err con mensaje).
    pub result: Result<ParseResult, FileProcessingError>,
    /// Tamaño del archivo en bytes.
    pub file_size: u64,
}

/// Errores posibles durante el procesamiento de un archivo.
#[derive(Debug)]
pub enum FileProcessingError {
    /// Error de lectura de archivo.
    IoError(String),
    /// Error de parsing.
    ParseError(String),
}

impl std::fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO Error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

impl std::error::Error for FileProcessingError {}

/// Resultado agregado de un batch de procesamiento paralelo.
#[derive(Debug)]
pub struct BatchProcessingResult {
    /// Resultados individuales de cada archivo.
    pub results: Vec<FileProcessingResult>,
    /// Total de manos parseadas exitosamente.
    pub total_hands: usize,
    /// Total de archivos procesados exitosamente.
    pub successful_files: usize,
    /// Total de archivos con errores.
    pub failed_files: usize,
    /// Tiempo total de procesamiento en milisegundos.
    pub elapsed_ms: u128,
}

/// Token de cancelación para detener el procesamiento.
#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Crea un nuevo token de cancelación.
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Señala la cancelación del procesamiento.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Verifica si se ha solicitado cancelación.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

/// Procesador paralelo de archivos de historial.
pub struct ParallelProcessor {
    config: ProcessingConfig,
}

impl ParallelProcessor {
    /// Crea un nuevo procesador con la configuración especificada.
    pub fn new(config: ProcessingConfig) -> Self {
        Self { config }
    }

    /// Crea un procesador con la configuración por defecto (16 hilos).
    pub fn default_ryzen() -> Self {
        Self::new(ProcessingConfig::default())
    }

    /// Procesa una lista de archivos en paralelo.
    ///
    /// # Argumentos
    ///
    /// * `files` - Vector de rutas de archivos a procesar.
    /// * `progress_callback` - Callback opcional para reportar progreso.
    ///
    /// # Retorno
    ///
    /// Retorna un `BatchProcessingResult` con los resultados agregados.
    pub fn process_files<F>(
        &self,
        files: Vec<PathBuf>,
        progress_callback: Option<F>,
    ) -> BatchProcessingResult
    where
        F: Fn(ProcessingProgress) + Send + Sync,
    {
        self.process_files_with_cancellation(files, progress_callback, None)
    }

    /// Procesa archivos en paralelo con soporte de cancelación.
    ///
    /// # Argumentos
    ///
    /// * `files` - Vector de rutas de archivos a procesar.
    /// * `progress_callback` - Callback opcional para reportar progreso.
    /// * `cancellation_token` - Token opcional para cancelar el procesamiento.
    pub fn process_files_with_cancellation<F>(
        &self,
        files: Vec<PathBuf>,
        progress_callback: Option<F>,
        cancellation_token: Option<CancellationToken>,
    ) -> BatchProcessingResult
    where
        F: Fn(ProcessingProgress) + Send + Sync,
    {
        let start_time = std::time::Instant::now();
        let total_files = files.len();

        if total_files == 0 {
            return BatchProcessingResult {
                results: vec![],
                total_hands: 0,
                successful_files: 0,
                failed_files: 0,
                elapsed_ms: 0,
            };
        }

        // Contadores atómicos para progreso
        let completed = Arc::new(AtomicUsize::new(0));
        let errors = Arc::new(AtomicUsize::new(0));
        let cancellation = cancellation_token.unwrap_or_default();

        // Crear pool de hilos personalizado
        let pool = ThreadPoolBuilder::new()
            .num_threads(self.config.num_threads)
            .stack_size(self.config.stack_size)
            .thread_name(|i| format!("{}-{}", "poker-parser", i))
            .build()
            .expect("Failed to create Rayon thread pool");

        // Procesar archivos en paralelo
        let results: Vec<FileProcessingResult> = pool.install(|| {
            files
                .into_par_iter()
                .map(|path| {
                    // Verificar cancelación antes de procesar
                    if cancellation.is_cancelled() {
                        let result = FileProcessingResult {
                            path: path.clone(),
                            result: Err(FileProcessingError::ParseError(
                                "Processing cancelled".to_string(),
                            )),
                            file_size: 0,
                        };
                        errors.fetch_add(1, Ordering::SeqCst);
                        return result;
                    }

                    // Procesar el archivo
                    let result = process_single_file(&path);

                    // Actualizar contadores
                    let is_error = result.result.is_err();
                    if is_error {
                        errors.fetch_add(1, Ordering::SeqCst);
                    }
                    let current_completed = completed.fetch_add(1, Ordering::SeqCst) + 1;

                    // Reportar progreso si hay callback
                    if let Some(ref callback) = progress_callback {
                        callback(ProcessingProgress {
                            completed: current_completed,
                            total: total_files,
                            errors: errors.load(Ordering::SeqCst),
                        });
                    }

                    result
                })
                .collect()
        });

        // Calcular estadísticas finales
        let (total_hands, successful_files) =
            results
                .iter()
                .fold((0, 0), |(hands, success), result| match &result.result {
                    Ok(parse_result) => (hands + parse_result.hands.len(), success + 1),
                    Err(_) => (hands, success),
                });

        BatchProcessingResult {
            results,
            total_hands,
            successful_files,
            failed_files: errors.load(Ordering::SeqCst),
            elapsed_ms: start_time.elapsed().as_millis(),
        }
    }
}

/// Procesa un archivo individual (llamado por cada hilo del pool).
fn process_single_file(path: &PathBuf) -> FileProcessingResult {
    // Leer el archivo
    let file_content: FileContent = match read_file_optimized(path) {
        Ok(content) => content,
        Err(e) => {
            return FileProcessingResult {
                path: path.clone(),
                result: Err(FileProcessingError::IoError(e.to_string())),
                file_size: 0,
            };
        }
    };

    let file_size = file_content.size;

    // Convertir bytes a string (lossy para evitar errores UTF-8)
    let text = String::from_utf8_lossy(&file_content.bytes);

    // Parsear con FSM
    let mut parser = WinamaxParser::new();
    let parse_result = parser.parse(&text);

    FileProcessingResult {
        path: path.clone(),
        result: Ok(parse_result),
        file_size,
    }
}

/// Obtiene el número de CPUs disponibles.
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(DEFAULT_NUM_THREADS)
}

/// Función de conveniencia para procesar archivos con configuración por defecto.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use poker_parsers::parallel_processor::process_files_parallel;
/// use std::path::PathBuf;
///
/// let files = vec![PathBuf::from("history.txt")];
/// let result = process_files_parallel(files);
/// println!("Procesadas {} manos en {}ms", result.total_hands, result.elapsed_ms);
/// ```
pub fn process_files_parallel(files: Vec<PathBuf>) -> BatchProcessingResult {
    let processor = ParallelProcessor::default_ryzen();
    processor.process_files(files, None::<fn(ProcessingProgress)>)
}

/// Función de conveniencia con callback de progreso.
pub fn process_files_parallel_with_progress<F>(
    files: Vec<PathBuf>,
    progress_callback: F,
) -> BatchProcessingResult
where
    F: Fn(ProcessingProgress) + Send + Sync,
{
    let processor = ParallelProcessor::default_ryzen();
    processor.process_files(files, Some(progress_callback))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_processing_config_default() {
        let config = ProcessingConfig::default();
        assert_eq!(config.num_threads, DEFAULT_NUM_THREADS);
        assert_eq!(config.stack_size, DEFAULT_STACK_SIZE);
    }

    #[test]
    fn test_processing_config_with_threads() {
        let config = ProcessingConfig::with_threads(8);
        assert_eq!(config.num_threads, 8);
    }

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());

        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn test_process_empty_files() {
        let processor = ParallelProcessor::default_ryzen();
        let result = processor.process_files(vec![], None::<fn(ProcessingProgress)>);

        assert_eq!(result.total_hands, 0);
        assert_eq!(result.successful_files, 0);
        assert_eq!(result.failed_files, 0);
    }

    #[test]
    fn test_process_single_file_io_error() {
        let path = PathBuf::from("nonexistent_file_12345.txt");
        let result = process_single_file(&path);

        assert!(result.result.is_err());
        matches!(result.result, Err(FileProcessingError::IoError(_)));
    }

    #[test]
    fn test_process_single_valid_file() {
        // Crear archivo con contenido válido de Winamax
        let content = r#"Winamax Poker - CashGame - HandId: #123-456-789 - Holdem no limit (0.01€/0.02€) - 2024/01/15 20:30:00 UTC
Table: 'Test Table' 6-max Seat #1 is the button
Seat 1: Player1 (2.00€)
Seat 2: thesmoy (2.00€)
*** ANTE/BLINDS ***
Player1 posts small blind 0.01€
thesmoy posts big blind 0.02€
*** PRE-FLOP ***
Player1 folds
thesmoy collected 0.03€ from pot
*** SUMMARY ***
Total pot 0.03€ | No rake
"#;
        let file = create_test_file(content);
        let result = process_single_file(&file.path().to_path_buf());

        assert!(result.result.is_ok());
        assert!(result.file_size > 0);
    }

    #[test]
    fn test_parallel_processing_with_progress() {
        let content = "Winamax Poker - Test file";
        let file1 = create_test_file(content);
        let file2 = create_test_file(content);

        let files = vec![file1.path().to_path_buf(), file2.path().to_path_buf()];

        let progress_updates = Arc::new(AtomicUsize::new(0));
        let progress_counter = progress_updates.clone();

        let processor = ParallelProcessor::new(ProcessingConfig::with_threads(2));
        let _result = processor.process_files(
            files,
            Some(move |_progress| {
                progress_counter.fetch_add(1, Ordering::SeqCst);
            }),
        );

        // Debería haber 2 actualizaciones de progreso (una por archivo)
        assert_eq!(progress_updates.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_cancellation_stops_processing() {
        let content = "Winamax Poker - Test file";
        let file1 = create_test_file(content);
        let file2 = create_test_file(content);

        let files = vec![file1.path().to_path_buf(), file2.path().to_path_buf()];

        let token = CancellationToken::new();
        token.cancel(); // Cancelar antes de procesar

        let processor = ParallelProcessor::new(ProcessingConfig::with_threads(1));
        let result = processor.process_files_with_cancellation(
            files,
            None::<fn(ProcessingProgress)>,
            Some(token),
        );

        // Todos los archivos deberían tener error de cancelación
        assert_eq!(result.failed_files, 2);
    }
}
