//! Sistema de file watching para detectar nuevos historiales automáticamente.
//!
//! Este módulo implementa un watcher de archivos usando el crate `notify` para
//! detectar automáticamente nuevos archivos de historial de Winamax en Windows.
//!
//! ## Características
//!
//! - **File Watching**: Usa `notify::RecommendedWatcher` para compatibilidad multiplataforma
//! - **Deduplicación**: Hash MD5 para evitar procesamiento duplicado
//! - **Cola de Procesamiento**: `mpsc::channel` para manejo asíncrono
//! - **Retry Logic**: Manejo de archivos bloqueados con backoff exponencial
//! - **Integración**: Se conecta con `ParallelProcessor` para procesamiento multihilo
//!
//! ## Uso
//!
//! ```rust,no_run
//! use poker_parsers::file_watcher::{FileWatcher, WatcherConfig};
//! use std::path::PathBuf;
//!
//! let config = WatcherConfig {
//!     watch_path: PathBuf::from(r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history"),
//!     ..Default::default()
//! };
//!
//! let watcher = FileWatcher::new(config);
//! watcher.start(|file_path| {
//!     println!("Nuevo archivo detectado: {:?}", file_path);
//! });
//! ```

use notify::{
    Config, Error as NotifyError, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::parallel_processor::ParallelProcessor;

/// Configuración del file watcher.
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// Ruta del directorio a monitorear.
    pub watch_path: PathBuf,
    /// Número máximo de reintentos para archivos bloqueados.
    pub max_retries: u32,
    /// Delay inicial para retry en milisegundos.
    pub retry_delay_ms: u64,
    /// Usar backoff exponencial para retries.
    pub use_exponential_backoff: bool,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            watch_path: PathBuf::from(
                r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history",
            ),
            max_retries: 3,
            retry_delay_ms: 100,
            use_exponential_backoff: true,
        }
    }
}

/// Evento de archivo detectado por el watcher.
#[derive(Debug, Clone)]
pub struct FileEvent {
    /// Ruta del archivo.
    pub path: PathBuf,
    /// Hash MD5 del archivo (para deduplicación).
    pub hash: String,
    /// Timestamp de detección.
    pub timestamp: std::time::SystemTime,
}

/// Errores del file watcher.
#[derive(Debug)]
pub enum WatcherError {
    /// Error de notify.
    NotifyError(String),
    /// Error de I/O.
    IoError(io::Error),
    /// Archivo no es un .txt válido.
    InvalidFileType(PathBuf),
    /// Archivo bloqueado después de todos los reintentos.
    FileLocked(PathBuf),
}

impl std::fmt::Display for WatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotifyError(msg) => write!(f, "Notify Error: {}", msg),
            Self::IoError(err) => write!(f, "IO Error: {}", err),
            Self::InvalidFileType(path) => write!(f, "Invalid file type: {:?}", path),
            Self::FileLocked(path) => write!(f, "File locked after retries: {:?}", path),
        }
    }
}

impl std::error::Error for WatcherError {}

impl From<NotifyError> for WatcherError {
    fn from(err: NotifyError) -> Self {
        Self::NotifyError(err.to_string())
    }
}

impl From<io::Error> for WatcherError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

/// File watcher para detectar nuevos historiales.
pub struct FileWatcher {
    config: WatcherConfig,
    processed_hashes: Arc<Mutex<HashSet<String>>>,
    file_queue_tx: Sender<FileEvent>,
    file_queue_rx: Arc<Mutex<Receiver<FileEvent>>>,
}

impl FileWatcher {
    /// Crea un nuevo file watcher con la configuración especificada.
    pub fn new(config: WatcherConfig) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            config,
            processed_hashes: Arc::new(Mutex::new(HashSet::new())),
            file_queue_tx: tx,
            file_queue_rx: Arc::new(Mutex::new(rx)),
        }
    }

    /// Inicia el file watcher con un callback para procesar archivos.
    ///
    /// El callback recibe la ruta del archivo detectado.
    /// Esta función bloquea el hilo actual.
    pub fn start<F>(self, callback: F) -> Result<(), WatcherError>
    where
        F: Fn(PathBuf) + Send + 'static,
    {
        // Verificar que el directorio existe
        if !self.config.watch_path.exists() {
            return Err(WatcherError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Watch path does not exist: {:?}", self.config.watch_path),
            )));
        }

        // Crear el watcher
        let (notify_tx, notify_rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(notify_tx, Config::default())?;

        // Iniciar el watching
        watcher.watch(&self.config.watch_path, RecursiveMode::NonRecursive)?;

        println!(
            "File watcher iniciado en: {:?}",
            self.config.watch_path.display()
        );

        // Clonar referencias para los threads
        let processed_hashes = self.processed_hashes.clone();
        let file_queue_tx = self.file_queue_tx.clone();
        let config = self.config.clone();

        // Thread 1: Escuchar eventos de notify
        thread::spawn(move || {
            for event_result in notify_rx {
                match event_result {
                    Ok(event) => {
                        if let Err(e) = Self::handle_notify_event(
                            event,
                            &config,
                            &processed_hashes,
                            &file_queue_tx,
                        ) {
                            eprintln!("Error manejando evento: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error de notify: {}", e),
                }
            }
        });

        // Thread 2: Procesar archivos de la cola
        let file_queue_rx = self.file_queue_rx.clone();
        thread::spawn(move || loop {
            let rx = file_queue_rx.lock().unwrap();
            if let Ok(file_event) = rx.recv() {
                drop(rx); // Liberar el lock antes de procesar
                callback(file_event.path);
            }
        });

        // Mantener el watcher vivo (bloquea el hilo principal)
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Inicia el watcher con integración directa al ParallelProcessor.
    ///
    /// Los archivos detectados se procesan automáticamente usando Rayon.
    pub fn start_with_processor(self, processor: ParallelProcessor) -> Result<(), WatcherError> {
        self.start(move |path| {
            println!("Procesando archivo detectado: {:?}", path);
            let result = processor.process_files(vec![path.clone()], None::<fn(_)>);
            println!(
                "Procesadas {} manos en {}ms",
                result.total_hands, result.elapsed_ms
            );
        })
    }

    /// Maneja un evento de notify.
    fn handle_notify_event(
        event: Event,
        config: &WatcherConfig,
        processed_hashes: &Arc<Mutex<HashSet<String>>>,
        file_queue_tx: &Sender<FileEvent>,
    ) -> Result<(), WatcherError> {
        // Solo procesar eventos Create y Modify
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {}
            _ => return Ok(()),
        }

        for path in event.paths {
            // Filtrar solo archivos .txt
            if !Self::is_txt_file(&path) {
                continue;
            }

            // Intentar procesar el archivo con retry logic
            match Self::process_file_with_retry(&path, config) {
                Ok(file_event) => {
                    // Verificar deduplicación
                    let mut hashes = processed_hashes.lock().unwrap();
                    if hashes.contains(&file_event.hash) {
                        continue; // Ya procesado
                    }

                    // Marcar como procesado y enviar a la cola
                    hashes.insert(file_event.hash.clone());
                    drop(hashes); // Liberar lock

                    file_queue_tx
                        .send(file_event)
                        .expect("Failed to send file event");
                }
                Err(e) => {
                    eprintln!("Error procesando archivo {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    /// Verifica si un archivo es .txt.
    fn is_txt_file(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("txt"))
            .unwrap_or(false)
    }

    /// Procesa un archivo con retry logic para manejar archivos bloqueados.
    fn process_file_with_retry(
        path: &Path,
        config: &WatcherConfig,
    ) -> Result<FileEvent, WatcherError> {
        let mut attempt = 0;
        let mut delay = config.retry_delay_ms;

        loop {
            match Self::read_and_hash_file(path) {
                Ok(hash) => {
                    return Ok(FileEvent {
                        path: path.to_path_buf(),
                        hash,
                        timestamp: std::time::SystemTime::now(),
                    });
                }
                Err(e) if attempt < config.max_retries => {
                    // Archivo bloqueado, reintentar
                    eprintln!(
                        "Archivo bloqueado (intento {}/{}): {:?}",
                        attempt + 1,
                        config.max_retries,
                        path
                    );
                    thread::sleep(Duration::from_millis(delay));

                    // Backoff exponencial
                    if config.use_exponential_backoff {
                        delay *= 2;
                    }

                    attempt += 1;
                }
                Err(_) => {
                    // Máximo de reintentos alcanzado
                    return Err(WatcherError::FileLocked(path.to_path_buf()));
                }
            }
        }
    }

    /// Lee un archivo y calcula su hash MD5.
    fn read_and_hash_file(path: &Path) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Calcular MD5
        let digest = md5::compute(&buffer);
        Ok(format!("{:x}", digest))
    }
}

/// Builder para configurar el file watcher.
pub struct FileWatcherBuilder {
    config: WatcherConfig,
}

impl FileWatcherBuilder {
    /// Crea un nuevo builder con configuración por defecto.
    pub fn new() -> Self {
        Self {
            config: WatcherConfig::default(),
        }
    }

    /// Establece la ruta a monitorear.
    pub fn watch_path(mut self, path: PathBuf) -> Self {
        self.config.watch_path = path;
        self
    }

    /// Establece el número máximo de reintentos.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// Establece el delay de retry.
    pub fn retry_delay_ms(mut self, delay: u64) -> Self {
        self.config.retry_delay_ms = delay;
        self
    }

    /// Habilita/deshabilita backoff exponencial.
    pub fn use_exponential_backoff(mut self, enable: bool) -> Self {
        self.config.use_exponential_backoff = enable;
        self
    }

    /// Construye el file watcher.
    pub fn build(self) -> FileWatcher {
        FileWatcher::new(self.config)
    }
}

impl Default for FileWatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_is_txt_file() {
        assert!(FileWatcher::is_txt_file(Path::new("test.txt")));
        assert!(FileWatcher::is_txt_file(Path::new("test.TXT")));
        assert!(!FileWatcher::is_txt_file(Path::new("test.log")));
        assert!(!FileWatcher::is_txt_file(Path::new("test")));
    }

    #[test]
    fn test_read_and_hash_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Crear archivo de prueba
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();
        drop(file);

        // Leer y hashear
        let hash = FileWatcher::read_and_hash_file(&file_path).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 32); // MD5 produce 32 caracteres hex
    }

    #[test]
    fn test_read_and_hash_same_content_produces_same_hash() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("test1.txt");
        let file2 = temp_dir.path().join("test2.txt");

        // Crear dos archivos con el mismo contenido
        let content = b"Identical content";
        fs::write(&file1, content).unwrap();
        fs::write(&file2, content).unwrap();

        let hash1 = FileWatcher::read_and_hash_file(&file1).unwrap();
        let hash2 = FileWatcher::read_and_hash_file(&file2).unwrap();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_watcher_config_default() {
        let config = WatcherConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_ms, 100);
        assert!(config.use_exponential_backoff);
    }

    #[test]
    fn test_builder_pattern() {
        let watcher = FileWatcherBuilder::new()
            .watch_path(PathBuf::from("/custom/path"))
            .max_retries(5)
            .retry_delay_ms(200)
            .use_exponential_backoff(false)
            .build();

        assert_eq!(watcher.config.watch_path, PathBuf::from("/custom/path"));
        assert_eq!(watcher.config.max_retries, 5);
        assert_eq!(watcher.config.retry_delay_ms, 200);
        assert!(!watcher.config.use_exponential_backoff);
    }
}
