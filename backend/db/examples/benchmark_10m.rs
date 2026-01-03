//! # Benchmark 10M Hands
//!
//! Pruebas de carga masiva con 10 millones de manos.
//! Mide rendimiento de generacion, ingesta y consultas.
//!
//! ## Uso
//!
//! ```bash
//! cargo run --release --example benchmark_10m
//! ```
//!
//! ## Criterios de Aceptacion (Issue #65)
//! - Carga de 10M manos en < 5 minutos
//! - Consultas de stats < 500ms
//! - Uso de RAM < 32GB durante consultas
//! - Sin OOM errors

use poker_db::{
    ActionType as DbActionType, DbConfig, DbConnection, GameFormat, HandAction, HandMetadata,
    InMemoryOptimization, MemoryMonitor, ParquetWriteConfig, ParquetWriter, Street as DbStreet,
};
use poker_parsers::{
    synthetic_generator::{generate_synthetic_hands, SyntheticConfig},
    ActionType, ParsedHand, Street,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

// ============================================================================
// CONFIGURATION
// ============================================================================

const TOTAL_HANDS: usize = 10_000_000;
const BATCH_SIZE: usize = 1_000_000; // 1M por batch
const SEED: u64 = 42;
const OUTPUT_DIR: &str = "./data/benchmark_10m";

// ============================================================================
// BENCHMARK RESULTS
// ============================================================================

#[derive(Debug, Default)]
struct BenchmarkResults {
    // Generacion
    generation_time_ms: u128,
    generation_hands_per_sec: f64,

    // Persistencia Parquet
    parquet_write_time_ms: u128,
    parquet_write_hands_per_sec: f64,
    parquet_files_created: usize,
    parquet_total_size_bytes: u64,

    // Carga en DuckDB
    duckdb_load_time_ms: u128,
    duckdb_load_hands_per_sec: f64,

    // Queries
    query_results: Vec<QueryResult>,

    // Memoria
    peak_memory_mb: f64,
    final_memory_mb: f64,

    // Totales
    total_hands: usize,
    total_actions: usize,
    total_time_ms: u128,
}

#[derive(Debug, Clone)]
struct QueryResult {
    name: String,
    description: String,
    execution_time_ms: u128,
    rows_returned: usize,
    passed: bool, // < 500ms
}

impl BenchmarkResults {
    fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║            BENCHMARK RESULTS - 10M HANDS                         ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ Hardware: Ryzen 7 3800X (16 threads) + 64GB RAM                  ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");

        // Generacion
        println!("║ GENERATION                                                        ║");
        println!(
            "║   Time: {:>10.2}s  |  Speed: {:>12.0} hands/sec            ║",
            self.generation_time_ms as f64 / 1000.0,
            self.generation_hands_per_sec
        );

        // Parquet
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ PARQUET PERSISTENCE                                              ║");
        println!(
            "║   Time: {:>10.2}s  |  Speed: {:>12.0} hands/sec            ║",
            self.parquet_write_time_ms as f64 / 1000.0,
            self.parquet_write_hands_per_sec
        );
        println!(
            "║   Files: {:>9}  |  Size: {:>12.2} MB                    ║",
            self.parquet_files_created,
            self.parquet_total_size_bytes as f64 / 1_000_000.0
        );

        // DuckDB Load
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ DUCKDB LOAD                                                       ║");
        println!(
            "║   Time: {:>10.2}s  |  Speed: {:>12.0} hands/sec            ║",
            self.duckdb_load_time_ms as f64 / 1000.0,
            self.duckdb_load_hands_per_sec
        );

        // Queries
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ QUERY BENCHMARKS (Criteria: < 500ms)                             ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");

        for query in &self.query_results {
            let status = if query.passed { "PASS" } else { "FAIL" };
            println!(
                "║ {:40} {:>6}ms  [{}] ║",
                &query.name[..query.name.len().min(40)],
                query.execution_time_ms,
                status
            );
        }

        // Memoria
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ MEMORY USAGE (Criteria: < 32GB)                                   ║");
        println!(
            "║   Peak: {:>10.2} GB  |  Final: {:>10.2} GB                 ║",
            self.peak_memory_mb / 1024.0,
            self.final_memory_mb / 1024.0
        );

        let memory_passed = self.peak_memory_mb < 32768.0; // 32GB
        println!(
            "║   Status: [{}]                                                 ║",
            if memory_passed { "PASS" } else { "FAIL" }
        );

        // Resumen
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ SUMMARY                                                           ║");
        println!(
            "║   Total Hands: {:>12}  |  Total Actions: {:>12}    ║",
            format_number(self.total_hands),
            format_number(self.total_actions)
        );
        println!(
            "║   Total Time: {:>10.2}s                                        ║",
            self.total_time_ms as f64 / 1000.0
        );

        // Criterios
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ ACCEPTANCE CRITERIA                                               ║");

        let pipeline_time_min = self.total_time_ms as f64 / 60000.0;
        let pipeline_passed = pipeline_time_min < 5.0;
        println!(
            "║   [{}] Full pipeline < 5 minutes ({:.2} min)                  ║",
            if pipeline_passed { "PASS" } else { "FAIL" },
            pipeline_time_min
        );

        let queries_passed = self.query_results.iter().all(|q| q.passed);
        println!(
            "║   [{}] All queries < 500ms                                     ║",
            if queries_passed { "PASS" } else { "FAIL" }
        );

        println!(
            "║   [{}] Memory < 32GB                                           ║",
            if memory_passed { "PASS" } else { "FAIL" }
        );

        let all_passed = pipeline_passed && queries_passed && memory_passed;
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!(
            "║ FINAL RESULT: [{}]                                            ║",
            if all_passed {
                "ALL CRITERIA PASSED"
            } else {
                "SOME CRITERIA FAILED"
            }
        );
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║        BENCHMARK 10M HANDS - Poker AI Web                        ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Target: {} hands                                      ║", format_number(TOTAL_HANDS));
    println!("║ Batch Size: {} hands                                     ║", format_number(BATCH_SIZE));
    println!("║ Threads: {}                                                     ║", rayon::current_num_threads());
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let total_start = Instant::now();
    let mut results = BenchmarkResults::default();
    results.total_hands = TOTAL_HANDS;

    // Crear directorio de salida
    std::fs::create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");

    // ========================================================================
    // PHASE 1: GENERATION + PERSISTENCE (in batches)
    // ========================================================================
    println!("[PHASE 1] Generating and persisting {} hands in batches of {}...",
        format_number(TOTAL_HANDS), format_number(BATCH_SIZE));
    println!();

    let num_batches = (TOTAL_HANDS + BATCH_SIZE - 1) / BATCH_SIZE;
    let mut total_actions = 0usize;
    let mut parquet_files = Vec::new();

    let gen_start = Instant::now();
    let mut parquet_time = Duration::ZERO;

    for batch_idx in 0..num_batches {
        let batch_start = Instant::now();
        let hands_in_batch = if batch_idx == num_batches - 1 {
            TOTAL_HANDS - (batch_idx * BATCH_SIZE)
        } else {
            BATCH_SIZE
        };

        // Generar batch
        let batch_seed = SEED + (batch_idx as u64 * 1_000_000);
        let config = SyntheticConfig::new(hands_in_batch).with_seed(batch_seed);
        let gen_result = generate_synthetic_hands(config);

        let gen_elapsed = batch_start.elapsed();

        // Convertir a formato DB
        let (metadata_list, actions_list, timestamps) = convert_hands_to_db_format(&gen_result.hands);
        total_actions += actions_list.len();

        // Persistir en Parquet
        let parquet_start = Instant::now();
        let batch_output = format!("{}/batch_{:03}", OUTPUT_DIR, batch_idx);
        std::fs::create_dir_all(&batch_output).ok();

        let parquet_config = ParquetWriteConfig::new(&batch_output)
            .with_compression_level(3)
            .with_row_group_size(100_000);

        let writer = ParquetWriter::new(parquet_config);

        if let Ok(path) = writer.write_hands_metadata(metadata_list) {
            parquet_files.push(path);
        }
        if let Ok(path) = writer.write_hands_actions(actions_list, &timestamps) {
            parquet_files.push(path);
        }

        let parquet_elapsed = parquet_start.elapsed();
        parquet_time += parquet_elapsed;

        let batch_total = batch_start.elapsed();
        println!(
            "  Batch {}/{}: {} hands | Gen: {:.2}s | Parquet: {:.2}s | Total: {:.2}s",
            batch_idx + 1,
            num_batches,
            format_number(hands_in_batch),
            gen_elapsed.as_secs_f64(),
            parquet_elapsed.as_secs_f64(),
            batch_total.as_secs_f64()
        );
    }

    let gen_total = gen_start.elapsed();
    results.generation_time_ms = (gen_total - parquet_time).as_millis();
    results.generation_hands_per_sec =
        TOTAL_HANDS as f64 / (results.generation_time_ms as f64 / 1000.0);

    results.parquet_write_time_ms = parquet_time.as_millis();
    results.parquet_write_hands_per_sec =
        TOTAL_HANDS as f64 / (results.parquet_write_time_ms as f64 / 1000.0);
    results.parquet_files_created = parquet_files.len();
    results.total_actions = total_actions;

    // Calcular tamano total de Parquet
    let parquet_size: u64 = parquet_files
        .iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();
    results.parquet_total_size_bytes = parquet_size;

    println!();
    println!("  Generation complete: {:.2}s ({:.0} hands/sec)",
        results.generation_time_ms as f64 / 1000.0,
        results.generation_hands_per_sec);
    println!("  Parquet write: {:.2}s ({:.2} MB total)",
        results.parquet_write_time_ms as f64 / 1000.0,
        parquet_size as f64 / 1_000_000.0);

    // ========================================================================
    // PHASE 2: LOAD INTO DUCKDB
    // ========================================================================
    println!();
    println!("[PHASE 2] Loading data into DuckDB in-memory...");

    let load_start = Instant::now();

    // Crear conexion DuckDB con optimizaciones
    let config = DbConfig::in_memory()
        .with_threads(16)
        .with_memory_limit_gb(48);
    let mut db = DbConnection::new(config).expect("Failed to create DuckDB connection");

    // Inicializar esquema
    db.init_schema_embedded().expect("Failed to init schema");

    // Aplicar optimizaciones in-memory
    let optimization = InMemoryOptimization::new()
        .with_aggressive_cache(true)
        .with_max_workers(16);
    optimization.apply(db.conn()).expect("Failed to apply optimizations");

    // Cargar datos desde Parquet
    for batch_idx in 0..num_batches {
        let batch_dir = format!("{}/batch_{:03}", OUTPUT_DIR, batch_idx);
        
        // Cargar metadata
        let metadata_pattern = format!("{}/*.parquet", batch_dir);
        let query = format!(
            "INSERT INTO hands_metadata SELECT * FROM read_parquet('{}/*metadata*.parquet')",
            batch_dir
        );
        let _ = db.conn().execute(&query, []);

        // Cargar acciones
        let query = format!(
            "INSERT INTO hands_actions SELECT * FROM read_parquet('{}/*actions*.parquet')",
            batch_dir
        );
        let _ = db.conn().execute(&query, []);
    }

    // Ejecutar ANALYZE para optimizar indices
    db.conn().execute("ANALYZE", []).ok();

    let load_elapsed = load_start.elapsed();
    results.duckdb_load_time_ms = load_elapsed.as_millis();
    results.duckdb_load_hands_per_sec =
        TOTAL_HANDS as f64 / (results.duckdb_load_time_ms as f64 / 1000.0);

    println!("  DuckDB load complete: {:.2}s ({:.0} hands/sec)",
        load_elapsed.as_secs_f64(),
        results.duckdb_load_hands_per_sec);

    // Verificar datos cargados
    let stats = db.get_stats().unwrap_or_default();
    println!("  Loaded: {} hands, {} actions",
        format_number(stats.hand_count as usize),
        format_number(stats.action_count as usize));

    // ========================================================================
    // PHASE 3: QUERY BENCHMARKS
    // ========================================================================
    println!();
    println!("[PHASE 3] Running query benchmarks...");
    println!();

    let queries = vec![
        (
            "COUNT hands_metadata",
            "Total count of hands",
            "SELECT COUNT(*) FROM hands_metadata"
        ),
        (
            "COUNT hands_actions",
            "Total count of actions",
            "SELECT COUNT(*) FROM hands_actions"
        ),
        (
            "VPIP by player",
            "Calculate VPIP for top 10 players",
            "SELECT player_id, 
                    COUNT(CASE WHEN action_type IN ('CALL', 'RAISE', 'BET') THEN 1 END) * 100.0 / COUNT(*) as vpip
             FROM hands_actions 
             WHERE street = 'PREFLOP'
             GROUP BY player_id 
             ORDER BY COUNT(*) DESC 
             LIMIT 10"
        ),
        (
            "PFR by player",
            "Calculate PFR for top 10 players",
            "SELECT player_id, 
                    COUNT(CASE WHEN action_type = 'RAISE' THEN 1 END) * 100.0 / COUNT(*) as pfr
             FROM hands_actions 
             WHERE street = 'PREFLOP'
             GROUP BY player_id 
             ORDER BY COUNT(*) DESC 
             LIMIT 10"
        ),
        (
            "3Bet frequency",
            "Calculate 3Bet frequency",
            "SELECT player_id,
                    COUNT(CASE WHEN action_type = 'RAISE' AND action_sequence >= 2 THEN 1 END) * 100.0 / 
                    NULLIF(COUNT(CASE WHEN action_sequence >= 2 THEN 1 END), 0) as three_bet
             FROM hands_actions 
             WHERE street = 'PREFLOP'
             GROUP BY player_id 
             HAVING COUNT(*) > 100
             ORDER BY COUNT(*) DESC 
             LIMIT 10"
        ),
        (
            "Filter by stake",
            "Filter hands by stake NL10",
            "SELECT COUNT(*) FROM hands_metadata WHERE stake = 'NL10'"
        ),
        (
            "Filter by date range",
            "Filter hands in last 30 days",
            "SELECT COUNT(*) FROM hands_metadata WHERE timestamp >= CURRENT_DATE - INTERVAL 30 DAY"
        ),
        (
            "Join metadata + actions",
            "Join hands with actions for Hero",
            "SELECT h.hand_id, COUNT(a.action_id) as action_count
             FROM hands_metadata h
             JOIN hands_actions a ON h.hand_id = a.hand_id
             WHERE a.is_hero_action = TRUE
             GROUP BY h.hand_id
             LIMIT 100"
        ),
        (
            "Aggregation by action type",
            "Count actions by type and street",
            "SELECT street, action_type, COUNT(*) as cnt
             FROM hands_actions
             GROUP BY street, action_type
             ORDER BY street, cnt DESC"
        ),
        (
            "Complex stats query",
            "Full player stats with multiple metrics",
            "SELECT 
                player_id,
                COUNT(DISTINCT hand_id) as total_hands,
                COUNT(CASE WHEN street = 'PREFLOP' AND action_type IN ('CALL', 'RAISE', 'BET') THEN 1 END) * 100.0 /
                    NULLIF(COUNT(CASE WHEN street = 'PREFLOP' THEN 1 END), 0) as vpip,
                COUNT(CASE WHEN street = 'PREFLOP' AND action_type = 'RAISE' THEN 1 END) * 100.0 /
                    NULLIF(COUNT(CASE WHEN street = 'PREFLOP' THEN 1 END), 0) as pfr,
                SUM(amount_cents) as total_amount
             FROM hands_actions
             GROUP BY player_id
             HAVING COUNT(DISTINCT hand_id) > 100
             ORDER BY total_hands DESC
             LIMIT 20"
        ),
    ];

    let mut peak_memory_mb = 0.0f64;
    let memory_monitor = MemoryMonitor::new(48);

    for (name, description, sql) in queries {
        let query_start = Instant::now();

        // Ejecutar query
        let mut stmt = db.conn().prepare(sql).expect("Failed to prepare query");
        let rows: Vec<Vec<String>> = stmt
            .query_map([], |row| {
                // Contar columnas y extraer valores
                let mut values = Vec::new();
                let mut idx = 0;
                loop {
                    match row.get::<_, String>(idx) {
                        Ok(v) => values.push(v),
                        Err(_) => {
                            match row.get::<_, i64>(idx) {
                                Ok(v) => values.push(v.to_string()),
                                Err(_) => {
                                    match row.get::<_, f64>(idx) {
                                        Ok(v) => values.push(format!("{:.2}", v)),
                                        Err(_) => break,
                                    }
                                }
                            }
                        }
                    }
                    idx += 1;
                }
                Ok(values)
            })
            .expect("Query failed")
            .filter_map(|r| r.ok())
            .collect();

        let query_elapsed = query_start.elapsed();

        // Medir memoria
        if let Ok(metrics) = memory_monitor.get_metrics(db.conn()) {
            let mem_mb = metrics.used_bytes as f64 / 1_000_000.0;
            if mem_mb > peak_memory_mb {
                peak_memory_mb = mem_mb;
            }
        }

        let passed = query_elapsed.as_millis() < 500;
        let status = if passed { "PASS" } else { "FAIL" };

        println!(
            "  {:40} {:>6}ms  [{}]  ({} rows)",
            name,
            query_elapsed.as_millis(),
            status,
            rows.len()
        );

        results.query_results.push(QueryResult {
            name: name.to_string(),
            description: description.to_string(),
            execution_time_ms: query_elapsed.as_millis(),
            rows_returned: rows.len(),
            passed,
        });
    }

    results.peak_memory_mb = peak_memory_mb;

    // Memoria final
    if let Ok(metrics) = memory_monitor.get_metrics(db.conn()) {
        results.final_memory_mb = metrics.used_bytes as f64 / 1_000_000.0;
    }

    // ========================================================================
    // FINALIZE
    // ========================================================================
    let total_elapsed = total_start.elapsed();
    results.total_time_ms = total_elapsed.as_millis();

    // Imprimir reporte final
    results.print_report();

    // Generar archivo de resultados para documentacion
    generate_markdown_report(&results);
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

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
        let naive_dt = parse_timestamp(&hand.timestamp);
        timestamps.insert(hand.hand_id.clone(), naive_dt);

        let stake = format!(
            "NL{}",
            (hand.big_blind_cents * 100 / 100).max(2)
        );

        let metadata = HandMetadata {
            hand_id: hand.hand_id.clone(),
            session_id: Some(format!(
                "SESSION-{}",
                &hand.hand_id[..8.min(hand.hand_id.len())]
            )),
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

        for (seq, action) in hand.actions.iter().enumerate() {
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

            let db_street = match action.street {
                Street::Preflop => DbStreet::Preflop,
                Street::Flop => DbStreet::Flop,
                Street::Turn => DbStreet::Turn,
                Street::River => DbStreet::River,
            };

            let db_action_type = match action.action_type {
                ActionType::Fold => Some(DbActionType::Fold),
                ActionType::Call => Some(DbActionType::Call),
                ActionType::Raise => Some(DbActionType::Raise),
                ActionType::Bet => Some(DbActionType::Bet),
                ActionType::Check => Some(DbActionType::Check),
                ActionType::AllIn => Some(DbActionType::AllIn),
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

fn parse_timestamp(ts: &str) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str(ts, "%Y/%m/%d %H:%M:%S UTC")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S"))
        .unwrap_or_else(|_| chrono::Utc::now().naive_utc())
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

fn generate_markdown_report(results: &BenchmarkResults) {
    let report = format!(
        r#"# Performance Benchmark Results - 10M Hands

## Test Configuration

| Parameter | Value |
|-----------|-------|
| Total Hands | {} |
| Batch Size | {} |
| Seed | {} |
| Hardware | Ryzen 7 3800X (16 threads) + 64GB RAM |

## Results Summary

### Phase 1: Generation

| Metric | Value |
|--------|-------|
| Time | {:.2}s |
| Speed | {:.0} hands/sec |

### Phase 2: Parquet Persistence

| Metric | Value |
|--------|-------|
| Time | {:.2}s |
| Speed | {:.0} hands/sec |
| Files Created | {} |
| Total Size | {:.2} MB |

### Phase 3: DuckDB Load

| Metric | Value |
|--------|-------|
| Time | {:.2}s |
| Speed | {:.0} hands/sec |

### Phase 4: Query Benchmarks

| Query | Time (ms) | Status |
|-------|-----------|--------|
{}

### Memory Usage

| Metric | Value |
|--------|-------|
| Peak | {:.2} GB |
| Final | {:.2} GB |

## Acceptance Criteria

| Criteria | Target | Actual | Status |
|----------|--------|--------|--------|
| Full pipeline | < 5 min | {:.2} min | {} |
| All queries | < 500ms | {} | {} |
| Memory usage | < 32GB | {:.2} GB | {} |

## Total Time: {:.2}s
"#,
        format_number(results.total_hands),
        format_number(BATCH_SIZE),
        SEED,
        results.generation_time_ms as f64 / 1000.0,
        results.generation_hands_per_sec,
        results.parquet_write_time_ms as f64 / 1000.0,
        results.parquet_write_hands_per_sec,
        results.parquet_files_created,
        results.parquet_total_size_bytes as f64 / 1_000_000.0,
        results.duckdb_load_time_ms as f64 / 1000.0,
        results.duckdb_load_hands_per_sec,
        results.query_results.iter().map(|q| {
            format!("| {} | {} | {} |", q.name, q.execution_time_ms, if q.passed { "PASS" } else { "FAIL" })
        }).collect::<Vec<_>>().join("\n"),
        results.peak_memory_mb / 1024.0,
        results.final_memory_mb / 1024.0,
        results.total_time_ms as f64 / 60000.0,
        if results.total_time_ms < 300000 { "PASS" } else { "FAIL" },
        results.query_results.iter().map(|q| q.execution_time_ms.to_string() + "ms").collect::<Vec<_>>().join(", "),
        if results.query_results.iter().all(|q| q.passed) { "PASS" } else { "FAIL" },
        results.peak_memory_mb / 1024.0,
        if results.peak_memory_mb < 32768.0 { "PASS" } else { "FAIL" },
        results.total_time_ms as f64 / 1000.0,
    );

    // Guardar reporte
    let report_path = format!("{}/benchmark_report.md", OUTPUT_DIR);
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("Warning: Could not write report to {}: {}", report_path, e);
    } else {
        println!();
        println!("Report saved to: {}", report_path);
    }
}

