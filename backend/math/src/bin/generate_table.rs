//! Generador de Perfect Hash Table de 7 cartas
//!
//! Este binario genera el archivo `lookup_7cards.bin` que contiene
//! los rankings pre-calculados para todas las C(52,7) = 133,784,560
//! combinaciones posibles de 7 cartas.
//!
//! ## Uso
//!
//! ```bash
//! cd backend
//! cargo run --release --bin generate_table
//! ```
//!
//! ## Tiempo estimado
//!
//! Con Rayon en un Ryzen 7 3800X (16 threads): ~2-5 minutos
//!
//! ## Salida
//!
//! Genera `backend/math/data/lookup_7cards.bin` (~267MB)

use poker_math::generate_lookup_table;
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("=== Generador de Perfect Hash Table de 7 Cartas ===\n");

    // Determinar ruta de salida
    let output_path = if Path::new("math/data").exists() {
        "math/data/lookup_7cards.bin"
    } else if Path::new("backend/math/data").exists() {
        "backend/math/data/lookup_7cards.bin"
    } else if Path::new("data").exists() {
        "data/lookup_7cards.bin"
    } else {
        // Crear directorio si no existe
        std::fs::create_dir_all("data").expect("No se pudo crear directorio 'data'");
        "data/lookup_7cards.bin"
    };

    println!("Ruta de salida: {}", output_path);
    println!(
        "Combinaciones a procesar: {} (C(52,7))",
        poker_math::TOTAL_7CARD_COMBOS
    );
    println!(
        "Tamaño estimado: {} MB\n",
        (poker_math::TOTAL_7CARD_COMBOS * 2) / (1024 * 1024)
    );

    let start = Instant::now();

    match generate_lookup_table(output_path) {
        Ok(()) => {
            let elapsed = start.elapsed();
            println!("\nTabla generada exitosamente!");
            println!("Tiempo total: {:.2?}", elapsed);
            println!(
                "Velocidad: {:.0} combinaciones/segundo",
                poker_math::TOTAL_7CARD_COMBOS as f64 / elapsed.as_secs_f64()
            );

            // Verificar tamaño del archivo
            if let Ok(metadata) = std::fs::metadata(output_path) {
                println!("Tamaño del archivo: {} bytes", metadata.len());
            }
        }
        Err(e) => {
            eprintln!("\nError al generar tabla: {}", e);
            std::process::exit(1);
        }
    }
}
