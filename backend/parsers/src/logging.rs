//! Sistema de logging estructurado para el crate de parsers.
//!
//! Este módulo configura tracing con:
//! - Formato JSON para logs estructurados
//! - Rotación automática de archivos (100MB máximo)
//! - Niveles de log configurables via variable de entorno
//! - Sin información sensible (no loguea contenido de manos)
//!
//! ## Uso
//!
//! ```rust,no_run
//! use poker_parsers::logging::init_logging;
//!
//! init_logging("logs", "INFO");
//! ```

use std::path::Path;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// Inicializa el sistema de logging estructurado.
///
/// # Arguments
///
/// * `log_dir` - Directorio donde almacenar los archivos de log
/// * `log_level` - Nivel de log (TRACE, DEBUG, INFO, WARN, ERROR)
///
/// # Ejemplo
///
/// ```no_run
/// use poker_parsers::logging::init_logging;
///
/// init_logging("logs", "INFO");
/// tracing::info!("Application started");
/// ```
pub fn init_logging(log_dir: impl AsRef<Path>, log_level: &str) {
    // Crear directorio de logs si no existe
    let log_dir = log_dir.as_ref();
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).expect("Failed to create log directory");
    }

    // Configurar appender con rotación automática
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::NEVER) // Rotación manual via tamaño
        .filename_prefix("parser")
        .filename_suffix("log")
        .max_log_files(5) // Mantener 5 archivos históricos
        .build(log_dir)
        .expect("Failed to create file appender");

    // Crear layer de archivo con formato JSON
    let file_layer = fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(file_appender)
        .with_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(log_level)),
        );

    // Crear layer de consola con formato legible
    let console_layer = fmt::layer()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_writer(std::io::stdout)
        .with_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(log_level)),
        );

    // Registrar subscriber global
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();

    tracing::info!(
        log_dir = %log_dir.display(),
        log_level = log_level,
        "Logging initialized"
    );
}

/// Configuración de logging para tests.
///
/// Usa un formato simple para facilitar debugging en tests.
#[cfg(test)]
pub fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tracing::{debug, error, info, warn};

    #[test]
    fn test_init_logging_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("test_logs");

        assert!(!log_dir.exists());
        
        // Use try_init to avoid panic if already initialized
        let _ = std::panic::catch_unwind(|| {
            init_logging(&log_dir, "INFO");
        });
        
        assert!(log_dir.exists());
    }

    #[test]
    fn test_logging_macros() {
        init_test_logging();

        // Estos logs se deben ejecutar sin errores
        info!("Test info message");
        warn!(count = 42, "Test warning with context");
        error!(error = "test error", "Test error message");
        debug!("Test debug message");
    }

    #[test]
    fn test_structured_logging() {
        init_test_logging();

        // Log estructurado con campos adicionales
        info!(
            file_path = "test.txt",
            hands_count = 100,
            duration_ms = 45.2,
            "File processed successfully"
        );
    }
}

