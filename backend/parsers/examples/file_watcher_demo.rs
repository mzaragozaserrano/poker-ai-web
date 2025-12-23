//! Ejemplo de uso del file watcher para detectar nuevos historiales.
//!
//! Este ejemplo muestra cómo usar el FileWatcher para monitorear automáticamente
//! el directorio de historiales de Winamax y procesar nuevos archivos.
//!
//! ## Uso
//!
//! ```bash
//! cargo run --example file_watcher_demo
//! ```
//!
//! El watcher detectará automáticamente nuevos archivos .txt en el directorio
//! configurado y los procesará usando el ParallelProcessor.

use poker_parsers::{FileWatcherBuilder, ParallelProcessor, ProcessingConfig};
use std::path::PathBuf;

fn main() {
    println!("=== File Watcher Demo ===\n");

    // Configurar el procesador paralelo (16 hilos para Ryzen 3800X)
    let processor = ParallelProcessor::new(ProcessingConfig::default());

    // Configurar el file watcher
    let watcher = FileWatcherBuilder::new()
        .watch_path(PathBuf::from(
            r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history",
        ))
        .max_retries(3)
        .retry_delay_ms(100)
        .use_exponential_backoff(true)
        .build();

    println!("Iniciando file watcher...");
    println!("Monitoreando: C:\\Users\\Miguel\\AppData\\Roaming\\winamax\\documents\\accounts\\thesmoy\\history");
    println!("Presiona Ctrl+C para detener.\n");

    // Iniciar el watcher con integración al procesador
    if let Err(e) = watcher.start_with_processor(processor) {
        eprintln!("Error iniciando watcher: {}", e);
        std::process::exit(1);
    }
}
