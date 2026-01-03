//! # Synthetic Hand Generator CLI
//!
//! Herramienta de linea de comandos para generar manos sinteticas de poker.
//!
//! ## Uso
//!
//! ```bash
//! cargo run --example generate_synthetic -- --count 1000000 --seed 42 --stakes NL10,NL25
//! ```
//!
//! ## Opciones
//!
//! - `--count, -c`: Numero de manos a generar (default: 10000)
//! - `--seed, -s`: Semilla para reproducibilidad (default: aleatorio)
//! - `--stakes`: Lista de stakes separada por comas (default: NL10)
//! - `--output, -o`: Archivo de salida JSON (opcional)
//! - `--benchmark`: Solo medir tiempo, no guardar

use poker_parsers::synthetic_generator::{generate_synthetic_hands, SyntheticConfig};
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parsear argumentos
    let mut count: usize = 10_000;
    let mut seed: Option<u64> = None;
    let mut stakes: Vec<String> = vec!["NL10".to_string()];
    let mut output: Option<String> = None;
    let mut benchmark = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--count" | "-c" => {
                if i + 1 < args.len() {
                    count = args[i + 1].parse().unwrap_or(10_000);
                    i += 1;
                }
            }
            "--seed" | "-s" => {
                if i + 1 < args.len() {
                    seed = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--stakes" => {
                if i + 1 < args.len() {
                    stakes = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_uppercase())
                        .collect();
                    i += 1;
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--benchmark" => {
                benchmark = true;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {}
        }
        i += 1;
    }

    println!("========================================");
    println!("  Poker Synthetic Hand Generator");
    println!("========================================");
    println!();
    println!("Configuracion:");
    println!("  - Manos a generar: {}", format_number(count));
    println!(
        "  - Semilla: {}",
        seed.map_or("aleatorio".to_string(), |s| s.to_string())
    );
    println!("  - Stakes: {:?}", stakes);
    println!("  - Threads: {}", rayon::current_num_threads());
    println!();

    // Construir configuracion
    let mut config = SyntheticConfig::new(count).with_stakes(stakes);

    if let Some(s) = seed {
        config = config.with_seed(s);
    }

    // Generar manos
    println!("Generando manos...");
    let start = Instant::now();
    let result = generate_synthetic_hands(config);
    let elapsed = start.elapsed();

    // Estadisticas
    println!();
    println!("========================================");
    println!("  Resultados");
    println!("========================================");
    println!("  - Manos generadas: {}", format_number(result.hands.len()));
    println!("  - Tiempo total: {:.2}s", elapsed.as_secs_f64());
    println!(
        "  - Velocidad: {:.0} manos/segundo",
        result.hands_per_second
    );
    println!();

    // Verificar criterio de rendimiento (1M en < 60s)
    if count >= 1_000_000 && elapsed.as_secs() < 60 {
        println!("  [OK] Criterio de rendimiento cumplido: 1M manos en < 60s");
    } else if count >= 1_000_000 {
        println!(
            "  [!!] Criterio de rendimiento NO cumplido: {} segundos",
            elapsed.as_secs()
        );
    }

    // Estadisticas de las manos generadas
    if !result.hands.is_empty() {
        let total_pot: i64 = result.hands.iter().map(|h| h.pot.total_cents).sum();
        let avg_pot = total_pot / result.hands.len() as i64;
        let total_actions: usize = result.hands.iter().map(|h| h.actions.len()).sum();
        let avg_actions = total_actions / result.hands.len();

        let hero_hands = result
            .hands
            .iter()
            .filter(|h| h.players.iter().any(|p| p.is_hero))
            .count();

        println!();
        println!("Estadisticas de manos:");
        println!("  - Bote promedio: {} centavos", avg_pot);
        println!("  - Acciones promedio por mano: {}", avg_actions);
        println!(
            "  - Manos con heroe: {} ({:.1}%)",
            hero_hands,
            (hero_hands as f64 / result.hands.len() as f64) * 100.0
        );
    }

    // Guardar a archivo si se especifico
    if !benchmark {
        if let Some(output_path) = output {
            println!();
            println!("Guardando en {}...", output_path);

            match save_to_json(&result.hands, &output_path) {
                Ok(_) => println!("  [OK] Guardado exitosamente"),
                Err(e) => println!("  [ERROR] Error al guardar: {}", e),
            }
        }
    }

    // Mostrar ejemplo de mano generada
    if let Some(first_hand) = result.hands.first() {
        println!();
        println!("========================================");
        println!("  Ejemplo de mano generada");
        println!("========================================");
        println!("  Hand ID: {}", first_hand.hand_id);
        println!("  Table: {}", first_hand.table_name);
        println!(
            "  Blinds: {}/{}",
            first_hand.small_blind_cents, first_hand.big_blind_cents
        );
        println!("  Players: {}", first_hand.players.len());
        println!("  Actions: {}", first_hand.actions.len());
        println!("  Pot: {} centavos", first_hand.pot.total_cents);
        println!(
            "  Board: {:?}",
            first_hand
                .board
                .iter()
                .map(|c| format!("{}{}", c.rank, c.suit))
                .collect::<Vec<_>>()
        );
    }
}

fn print_help() {
    println!("Poker Synthetic Hand Generator");
    println!();
    println!("USAGE:");
    println!("    generate_synthetic [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -c, --count <N>      Numero de manos a generar (default: 10000)");
    println!("    -s, --seed <N>       Semilla para reproducibilidad");
    println!("    --stakes <LIST>      Stakes separados por coma (NL2,NL5,NL10,NL25,NL50)");
    println!("    -o, --output <FILE>  Archivo de salida JSON");
    println!("    --benchmark          Solo medir tiempo, no guardar");
    println!("    -h, --help           Mostrar esta ayuda");
    println!();
    println!("EXAMPLES:");
    println!("    # Generar 1 millon de manos con semilla 42");
    println!("    cargo run --example generate_synthetic -- -c 1000000 -s 42");
    println!();
    println!("    # Generar 100k manos NL25 y NL50");
    println!("    cargo run --example generate_synthetic -- -c 100000 --stakes NL25,NL50");
    println!();
    println!("    # Benchmark de rendimiento");
    println!("    cargo run --release --example generate_synthetic -- -c 1000000 --benchmark");
}

fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}

fn save_to_json(hands: &[poker_parsers::ParsedHand], path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(hands)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
