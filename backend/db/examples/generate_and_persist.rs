//! # Generate and Persist Synthetic Hands
//!
//! Ejemplo que demuestra la integración del generador sintético con el Parquet writer.
//!
//! ## Uso
//!
//! ```bash
//! cargo run --example generate_and_persist -- --count 100000 --output ./data
//! ```

use poker_db::{
    ActionType as DbActionType, GameFormat, HandAction, HandMetadata, ParquetWriteConfig,
    ParquetWriter, Street as DbStreet,
};
use poker_parsers::{
    synthetic_generator::{generate_synthetic_hands, SyntheticConfig},
    ActionType, ParsedHand, Street,
};
use std::collections::HashMap;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parsear argumentos
    let mut count: usize = 10_000;
    let mut seed: Option<u64> = None;
    let mut output_dir = String::from("./data/synthetic");

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
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_dir = args[i + 1].clone();
                    i += 1;
                }
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
    println!("  Generate and Persist Synthetic Hands");
    println!("========================================");
    println!();
    println!("Configuracion:");
    println!("  - Manos a generar: {}", format_number(count));
    println!(
        "  - Semilla: {}",
        seed.map_or("aleatorio".to_string(), |s| s.to_string())
    );
    println!("  - Directorio de salida: {}", output_dir);
    println!("  - Threads: {}", rayon::current_num_threads());
    println!();

    // Paso 1: Generar manos sinteticas
    println!("[1/3] Generando manos sinteticas...");
    let gen_start = Instant::now();

    let mut config = SyntheticConfig::new(count);
    if let Some(s) = seed {
        config = config.with_seed(s);
    }

    let result = generate_synthetic_hands(config);
    let gen_elapsed = gen_start.elapsed();

    println!(
        "      Generadas {} manos en {:.2}s ({:.0} manos/s)",
        format_number(result.hands.len()),
        gen_elapsed.as_secs_f64(),
        result.hands_per_second
    );

    // Paso 2: Convertir a formato DB
    println!();
    println!("[2/3] Convirtiendo a formato DB...");
    let conv_start = Instant::now();

    let (metadata_list, actions_list, timestamps) = convert_hands_to_db_format(&result.hands);
    let conv_elapsed = conv_start.elapsed();

    println!(
        "      Convertidas {} manos y {} acciones en {:.2}s",
        format_number(metadata_list.len()),
        format_number(actions_list.len()),
        conv_elapsed.as_secs_f64()
    );

    // Paso 3: Persistir en Parquet
    println!();
    println!("[3/3] Persistiendo en Parquet...");
    let persist_start = Instant::now();

    let parquet_config = ParquetWriteConfig::new(&output_dir)
        .with_compression_level(3)
        .with_row_group_size(100_000);

    let writer = ParquetWriter::new(parquet_config);

    // Escribir metadata
    match writer.write_hands_metadata(metadata_list.clone()) {
        Ok(path) => println!("      Metadata escrita: {:?}", path),
        Err(e) => println!("      Error escribiendo metadata: {}", e),
    }

    // Escribir acciones
    match writer.write_hands_actions(actions_list.clone(), &timestamps) {
        Ok(path) => println!("      Acciones escritas: {:?}", path),
        Err(e) => println!("      Error escribiendo acciones: {}", e),
    }

    let persist_elapsed = persist_start.elapsed();
    println!(
        "      Persistencia completada en {:.2}s",
        persist_elapsed.as_secs_f64()
    );

    // Resumen final
    let total_elapsed = gen_elapsed + conv_elapsed + persist_elapsed;
    println!();
    println!("========================================");
    println!("  Resumen");
    println!("========================================");
    println!("  - Manos procesadas: {}", format_number(count));
    println!("  - Acciones totales: {}", format_number(actions_list.len()));
    println!("  - Tiempo total: {:.2}s", total_elapsed.as_secs_f64());
    println!(
        "  - Velocidad total: {:.0} manos/s",
        count as f64 / total_elapsed.as_secs_f64()
    );
    println!();

    // Verificar criterio de rendimiento
    if count >= 1_000_000 && total_elapsed.as_secs() < 60 {
        println!("  [OK] Criterio cumplido: Pipeline completo < 60s para 1M manos");
    }
}

/// Convierte manos parseadas al formato de la base de datos
fn convert_hands_to_db_format(
    hands: &[ParsedHand],
) -> (
    Vec<HandMetadata>,
    Vec<HandAction>,
    HashMap<String, chrono::NaiveDateTime>,
) {
    let mut metadata_list = Vec::with_capacity(hands.len());
    let mut actions_list = Vec::with_capacity(hands.len() * 10);
    let mut timestamps = HashMap::new();

    for hand in hands {
        // Parsear timestamp
        let naive_dt = parse_timestamp(&hand.timestamp);
        timestamps.insert(hand.hand_id.clone(), naive_dt);

        // Determinar stake string
        let stake = format!(
            "NL{}",
            (hand.big_blind_cents * 100 / 100).max(2) // Simplificado
        );

        // Crear metadata
        let metadata = HandMetadata {
            hand_id: hand.hand_id.clone(),
            session_id: Some(format!("SESSION-{}", &hand.hand_id[..8.min(hand.hand_id.len())])),
            tournament_id: None,
            timestamp: naive_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            stake,
            format: GameFormat::Cash,
            table_name: hand.table_name.clone(),
            blind_level: hand.small_blind_cents,
            button_seat: hand.button_seat,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        metadata_list.push(metadata);

        // Crear acciones
        for (seq, action) in hand.actions.iter().enumerate() {
            // Encontrar player_id
            let player_id = hand
                .players
                .iter()
                .find(|p| p.name == action.player_name)
                .map(|p| format!("PLAYER-{}", &p.name))
                .unwrap_or_else(|| format!("PLAYER-{}", &action.player_name));

            let is_hero = hand
                .players
                .iter()
                .any(|p| p.name == action.player_name && p.is_hero);

            // Convertir street
            let db_street = match action.street {
                Street::Preflop => DbStreet::Preflop,
                Street::Flop => DbStreet::Flop,
                Street::Turn => DbStreet::Turn,
                Street::River => DbStreet::River,
            };

            // Convertir action type (filtrar los que no son validos para hands_actions)
            let db_action_type = match action.action_type {
                ActionType::Fold => Some(DbActionType::Fold),
                ActionType::Call => Some(DbActionType::Call),
                ActionType::Raise => Some(DbActionType::Raise),
                ActionType::Bet => Some(DbActionType::Bet),
                ActionType::Check => Some(DbActionType::Check),
                ActionType::AllIn => Some(DbActionType::AllIn),
                // Ignorar acciones de ciegas y showdown para la tabla de acciones
                ActionType::PostSmallBlind
                | ActionType::PostBigBlind
                | ActionType::PostAnte
                | ActionType::Show
                | ActionType::Collect => None,
            };

            if let Some(action_type) = db_action_type {
                let hand_action = HandAction {
                    action_id: uuid::Uuid::new_v4().to_string(),
                    hand_id: hand.hand_id.clone(),
                    player_id,
                    street: db_street,
                    action_type,
                    amount_cents: action.amount_cents.unwrap_or(0),
                    is_hero_action: is_hero,
                    ev_cents: None,
                    action_sequence: seq as i32,
                    created_at: chrono::Utc::now().to_rfc3339(),
                };
                actions_list.push(hand_action);
            }
        }
    }

    (metadata_list, actions_list, timestamps)
}

/// Parsea timestamp del formato del generador
fn parse_timestamp(ts: &str) -> chrono::NaiveDateTime {
    // Formato: "2024/01/15 10:30:00 UTC"
    chrono::NaiveDateTime::parse_from_str(ts, "%Y/%m/%d %H:%M:%S UTC")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S"))
        .unwrap_or_else(|_| chrono::Utc::now().naive_utc())
}

fn print_help() {
    println!("Generate and Persist Synthetic Hands");
    println!();
    println!("USAGE:");
    println!("    generate_and_persist [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -c, --count <N>      Numero de manos a generar (default: 10000)");
    println!("    -s, --seed <N>       Semilla para reproducibilidad");
    println!("    -o, --output <DIR>   Directorio de salida para Parquet (default: ./data/synthetic)");
    println!("    -h, --help           Mostrar esta ayuda");
    println!();
    println!("EXAMPLES:");
    println!("    # Generar y persistir 100k manos");
    println!("    cargo run --example generate_and_persist -- -c 100000");
    println!();
    println!("    # Generar 1M manos con semilla");
    println!("    cargo run --release --example generate_and_persist -- -c 1000000 -s 42");
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

