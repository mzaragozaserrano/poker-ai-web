//! Ejemplo simple de file watcher con callback personalizado.
//!
//! Este ejemplo muestra cómo usar el FileWatcher con un callback personalizado
//! para manejar los archivos detectados de forma manual.
//!
//! ## Uso
//!
//! ```bash
//! cargo run --example file_watcher_simple
//! ```

use poker_parsers::{FileWatcherBuilder, WinamaxParser};
use std::path::PathBuf;

fn main() {
    println!("=== File Watcher Simple Demo ===\n");

    // Configurar el file watcher
    let watcher = FileWatcherBuilder::new()
        .watch_path(PathBuf::from(
            r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history",
        ))
        .max_retries(3)
        .retry_delay_ms(100)
        .build();

    println!("Iniciando file watcher...");
    println!("Monitoreando: C:\\Users\\Miguel\\AppData\\Roaming\\winamax\\documents\\accounts\\thesmoy\\history");
    println!("Presiona Ctrl+C para detener.\n");

    // Iniciar el watcher con callback personalizado
    if let Err(e) = watcher.start(|file_path| {
        println!("\n[NUEVO ARCHIVO DETECTADO]");
        println!("Ruta: {:?}", file_path);

        // Leer y parsear el archivo manualmente
        match std::fs::read_to_string(&file_path) {
            Ok(content) => {
                let mut parser = WinamaxParser::new();
                let result = parser.parse(&content);

                println!("Manos parseadas: {}", result.hands.len());
                println!("Errores: {}", result.errors.len());

                // Mostrar información de la primera mano si existe
                if let Some(hand) = result.hands.first() {
                    println!("\nPrimera mano:");
                    println!("  Hand ID: {}", hand.hand_id);
                    println!("  Jugadores: {}", hand.players.len());
                    println!("  Acciones: {}", hand.actions.len());
                }
            }
            Err(e) => {
                eprintln!("Error leyendo archivo: {}", e);
            }
        }

        println!("---");
    }) {
        eprintln!("Error iniciando watcher: {}", e);
        std::process::exit(1);
    }
}

