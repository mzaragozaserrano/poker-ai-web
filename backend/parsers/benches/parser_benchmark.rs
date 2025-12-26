//! Benchmarks para el parser de Winamax.
//!
//! Compara el rendimiento de diferentes estrategias de parsing:
//! - Lectura de archivos (std::fs::read vs BufReader)
//! - Parsing con bytes vs strings
//! - Detección de prefijos (bytes vs Regex)
//!
//! Ejecutar con: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use poker_parsers::{
    bytes_parser, file_reader, parallel_processor, ParallelProcessor, ProcessingConfig,
    WinamaxParser,
};
use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};

/// Genera un historial de prueba con N manos.
fn generate_test_history(num_hands: usize) -> String {
    let single_hand = r#"Winamax Poker - CashGame - HandId: #21819158-393-1765807340 - Holdem no limit (0.01€/0.02€) - 2025/12/15 14:02:20 UTC
Table: 'Nice 09' 5-max (real money) Seat #3 is the button
Seat 1: captainogue (1.76€)
Seat 2: verlan4 (2.24€)
Seat 3: CucleBen (1.82€)
Seat 4: D0LLIPRANE (2.93€)
Seat 5: thesmoy (2€)
*** ANTE/BLINDS ***
D0LLIPRANE posts small blind 0.01€
thesmoy posts big blind 0.02€
Dealt to thesmoy [8d 8s]
*** PRE-FLOP ***
captainogue folds
verlan4 calls 0.02€
CucleBen raises 0.04€ to 0.06€
D0LLIPRANE folds
thesmoy calls 0.04€
verlan4 calls 0.04€
*** FLOP *** [6d Qc 7s]
thesmoy checks
verlan4 checks
CucleBen bets 0.19€
thesmoy folds
verlan4 calls 0.19€
*** TURN *** [6d Qc 7s][2c]
verlan4 checks
CucleBen bets 0.37€
verlan4 folds
CucleBen collected 0.91€ from pot
*** SUMMARY ***
Total pot 0.91€ | Rake 0.03€
Board: [6d Qc 7s 2c]
Seat 3: CucleBen (button) won 0.91€

"#;

    let mut result = String::with_capacity(single_hand.len() * num_hands);
    for _ in 0..num_hands {
        result.push_str(single_hand);
    }
    result
}

/// Benchmark: Lectura de archivos pequeños (< 10MB).
fn bench_file_reading_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_reading_small");

    for num_hands in [10, 100, 1000] {
        let content = generate_test_history(num_hands);
        let size = content.len();

        // Crear archivo temporal
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        let path = temp_file.path();

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("optimized", num_hands),
            &path,
            |b, path| {
                b.iter(|| {
                    let content = file_reader::read_file_optimized(path).unwrap();
                    black_box(content);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("std_read_to_string", num_hands),
            &path,
            |b, path| {
                b.iter(|| {
                    let content = std::fs::read_to_string(path).unwrap();
                    black_box(content);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: Parsing completo de manos.
fn bench_full_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_parsing");

    for num_hands in [10, 100, 1000] {
        let content = generate_test_history(num_hands);
        let size = content.len();

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("parse", num_hands),
            &content,
            |b, content| {
                b.iter(|| {
                    let mut parser = WinamaxParser::new();
                    let result = parser.parse(black_box(content));
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: Detección de prefijos (bytes vs str).
fn bench_prefix_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("prefix_detection");

    let lines = vec![
        "Winamax Poker - CashGame - HandId: #123",
        "Table: 'Nice 09' 5-max (real money)",
        "Seat 1: Player1 (2.00€)",
        "*** ANTE/BLINDS ***",
        "Player1 posts small blind 0.01€",
        "*** PRE-FLOP ***",
        "Player1 folds",
        "Player2 calls 0.02€",
        "*** SUMMARY ***",
    ];

    group.bench_function("bytes_starts_with", |b| {
        b.iter(|| {
            for line in &lines {
                let line_bytes = line.as_bytes();
                let _ = black_box(bytes_parser::starts_with_bytes(
                    line_bytes,
                    bytes_parser::tokens::WINAMAX_POKER,
                ));
                let _ = black_box(bytes_parser::starts_with_bytes(
                    line_bytes,
                    bytes_parser::tokens::TABLE,
                ));
                let _ = black_box(bytes_parser::starts_with_bytes(
                    line_bytes,
                    bytes_parser::tokens::SEAT,
                ));
            }
        });
    });

    group.bench_function("str_starts_with", |b| {
        b.iter(|| {
            for line in &lines {
                let _ = black_box(line.starts_with("Winamax Poker"));
                let _ = black_box(line.starts_with("Table: "));
                let _ = black_box(line.starts_with("Seat "));
            }
        });
    });

    group.finish();
}

/// Benchmark: Parsing de cantidades (bytes vs float).
fn bench_amount_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("amount_parsing");

    let amounts = vec![
        b"0.01".as_slice(),
        b"0.02".as_slice(),
        b"1.50".as_slice(),
        b"10.00".as_slice(),
        b"100.50".as_slice(),
    ];

    group.bench_function("bytes_parse_amount", |b| {
        b.iter(|| {
            for amount in &amounts {
                let cents = bytes_parser::parse_amount_cents(black_box(amount));
                black_box(cents);
            }
        });
    });

    group.bench_function("float_parse_amount", |b| {
        b.iter(|| {
            for amount in &amounts {
                let s = std::str::from_utf8(amount).unwrap();
                let value: f64 = s.parse().unwrap();
                let cents = (value * 100.0).round() as i64;
                black_box(cents);
            }
        });
    });

    group.finish();
}

/// Benchmark: Extracción de cartas.
fn bench_card_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("card_extraction");

    let lines = vec![
        b"Dealt to thesmoy [8d 8s]".as_slice(),
        b"Player1 shows [Ah Kh] (Two pairs)".as_slice(),
        b"*** FLOP *** [6d Qc 7s]".as_slice(),
    ];

    group.bench_function("extract_cards", |b| {
        b.iter(|| {
            for line in &lines {
                let cards = bytes_parser::extract_cards(black_box(line));
                black_box(cards);
            }
        });
    });

    group.finish();
}

/// Benchmark: Objetivo principal - 1000 manos en < 10ms.
fn bench_target_performance(c: &mut Criterion) {
    let content = generate_test_history(1000);
    let size = content.len();

    c.bench_function("parse_1000_hands", |b| {
        b.iter(|| {
            let mut parser = WinamaxParser::new();
            let result = parser.parse(black_box(&content));
            assert_eq!(result.hands.len(), 1000);
            black_box(result);
        });
    });

    println!(
        "\n=== OBJETIVO: < 10ms para 1000 manos ({} bytes) ===\n",
        size
    );
}

/// Crea archivos de prueba en un directorio temporal.
fn create_test_files(temp_dir: &TempDir, num_files: usize, hands_per_file: usize) -> Vec<PathBuf> {
    let mut files = Vec::with_capacity(num_files);

    for i in 0..num_files {
        let content = generate_test_history(hands_per_file);
        let path = temp_dir.path().join(format!("history_{}.txt", i));
        std::fs::write(&path, content).unwrap();
        files.push(path);
    }

    files
}

/// Benchmark: Procesamiento paralelo vs secuencial.
fn bench_parallel_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");

    // Configuración: 10 archivos con 100 manos cada uno (1000 manos total)
    let temp_dir = TempDir::new().unwrap();
    let files = create_test_files(&temp_dir, 10, 100);

    // Benchmark secuencial (1 hilo)
    group.bench_function("sequential_1_thread", |b| {
        let processor = ParallelProcessor::new(ProcessingConfig::with_threads(1));
        b.iter(|| {
            let result = processor.process_files(
                black_box(files.clone()),
                None::<fn(parallel_processor::ProcessingProgress)>,
            );
            black_box(result);
        });
    });

    // Benchmark paralelo (4 hilos)
    group.bench_function("parallel_4_threads", |b| {
        let processor = ParallelProcessor::new(ProcessingConfig::with_threads(4));
        b.iter(|| {
            let result = processor.process_files(
                black_box(files.clone()),
                None::<fn(parallel_processor::ProcessingProgress)>,
            );
            black_box(result);
        });
    });

    // Benchmark paralelo (8 hilos)
    group.bench_function("parallel_8_threads", |b| {
        let processor = ParallelProcessor::new(ProcessingConfig::with_threads(8));
        b.iter(|| {
            let result = processor.process_files(
                black_box(files.clone()),
                None::<fn(parallel_processor::ProcessingProgress)>,
            );
            black_box(result);
        });
    });

    // Benchmark paralelo (16 hilos - Ryzen 3800X)
    group.bench_function("parallel_16_threads", |b| {
        let processor = ParallelProcessor::default_ryzen();
        b.iter(|| {
            let result = processor.process_files(
                black_box(files.clone()),
                None::<fn(parallel_processor::ProcessingProgress)>,
            );
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark: Escalabilidad con diferentes cantidades de archivos.
fn bench_parallel_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_scalability");

    let temp_dir = TempDir::new().unwrap();

    for num_files in [5, 10, 20, 50, 100] {
        let files = create_test_files(&temp_dir, num_files, 50);

        group.bench_with_input(BenchmarkId::new("files", num_files), &files, |b, files| {
            let processor = ParallelProcessor::default_ryzen();
            b.iter(|| {
                let result = processor.process_files(
                    black_box(files.clone()),
                    None::<fn(parallel_processor::ProcessingProgress)>,
                );
                black_box(result);
            });
        });
    }

    group.finish();
}

/// Benchmark: Objetivo de 100+ archivos simultáneos.
fn bench_parallel_target(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let files = create_test_files(&temp_dir, 100, 10); // 100 archivos, 10 manos cada uno

    c.bench_function("parallel_100_files", |b| {
        let processor = ParallelProcessor::default_ryzen();
        b.iter(|| {
            let result = processor.process_files(
                black_box(files.clone()),
                None::<fn(parallel_processor::ProcessingProgress)>,
            );
            assert_eq!(result.successful_files, 100);
            assert_eq!(result.total_hands, 1000);
            black_box(result);
        });
    });

    println!("\n=== OBJETIVO: Procesar 100+ archivos simultáneamente sin bloqueos ===\n");
}

criterion_group!(
    benches,
    bench_file_reading_small,
    bench_full_parsing,
    bench_prefix_detection,
    bench_amount_parsing,
    bench_card_extraction,
    bench_target_performance,
    bench_parallel_processing,
    bench_parallel_scalability,
    bench_parallel_target,
);

criterion_main!(benches);

