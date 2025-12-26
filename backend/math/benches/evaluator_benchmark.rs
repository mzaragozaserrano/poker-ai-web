//! # Benchmarks del Evaluador de Manos
//!
//! Mide el rendimiento del evaluador de manos para verificar
//! el objetivo de < 100ns por evaluación.
//!
//! ## Ejecutar
//! ```bash
//! cargo bench --package poker-math
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use poker_math::hand_evaluator::{evaluate_5cards, evaluate_7cards, Card, Deck};
use std::str::FromStr;

/// Genera manos aleatorias para benchmarking
fn generate_random_5card_hands(count: usize) -> Vec<[Card; 5]> {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::seed_from_u64(42); // Seed fijo para reproducibilidad
    let mut hands = Vec::with_capacity(count);

    for _ in 0..count {
        let mut deck = Deck::new();
        deck.shuffle_with_rng(&mut rng);
        let cards = deck.deal_n(5);
        hands.push([cards[0], cards[1], cards[2], cards[3], cards[4]]);
    }

    hands
}

/// Genera manos aleatorias de 7 cartas
fn generate_random_7card_hands(count: usize) -> Vec<[Card; 7]> {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut hands = Vec::with_capacity(count);

    for _ in 0..count {
        let mut deck = Deck::new();
        deck.shuffle_with_rng(&mut rng);
        let cards = deck.deal_n(7);
        hands.push([
            cards[0], cards[1], cards[2], cards[3], cards[4], cards[5], cards[6],
        ]);
    }

    hands
}

/// Benchmark de evaluación de 5 cartas
fn bench_evaluate_5cards(c: &mut Criterion) {
    let hands = generate_random_5card_hands(1000);

    c.bench_function("evaluate_5cards", |b| {
        b.iter(|| {
            for hand in &hands {
                black_box(evaluate_5cards(black_box(hand)));
            }
        })
    });
}

/// Benchmark de evaluación de 7 cartas (Texas Hold'em)
fn bench_evaluate_7cards(c: &mut Criterion) {
    let hands = generate_random_7card_hands(1000);

    c.bench_function("evaluate_7cards", |b| {
        b.iter(|| {
            for hand in &hands {
                black_box(evaluate_7cards(black_box(hand)));
            }
        })
    });
}

/// Benchmark comparativo por tamaño de batch
fn bench_batch_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_evaluation");

    for size in [100, 1000, 10000].iter() {
        let hands = generate_random_7card_hands(*size);

        group.bench_with_input(BenchmarkId::new("7cards", size), &hands, |b, hands| {
            b.iter(|| {
                for hand in hands {
                    black_box(evaluate_7cards(black_box(hand)));
                }
            })
        });
    }

    group.finish();
}

/// Benchmark de manos específicas (casos conocidos)
fn bench_specific_hands(c: &mut Criterion) {
    // Royal Flush
    let royal_flush: [Card; 5] = [
        "As".parse().unwrap(),
        "Ks".parse().unwrap(),
        "Qs".parse().unwrap(),
        "Js".parse().unwrap(),
        "Ts".parse().unwrap(),
    ];

    // Pair
    let pair: [Card; 5] = [
        "Ah".parse().unwrap(),
        "As".parse().unwrap(),
        "Kd".parse().unwrap(),
        "Qc".parse().unwrap(),
        "Jh".parse().unwrap(),
    ];

    // High Card
    let high_card: [Card; 5] = [
        "Ah".parse().unwrap(),
        "Ks".parse().unwrap(),
        "9d".parse().unwrap(),
        "5c".parse().unwrap(),
        "2h".parse().unwrap(),
    ];

    let mut group = c.benchmark_group("specific_hands");

    group.bench_function("royal_flush", |b| {
        b.iter(|| black_box(evaluate_5cards(black_box(&royal_flush))))
    });

    group.bench_function("pair", |b| {
        b.iter(|| black_box(evaluate_5cards(black_box(&pair))))
    });

    group.bench_function("high_card", |b| {
        b.iter(|| black_box(evaluate_5cards(black_box(&high_card))))
    });

    group.finish();
}

/// Benchmark de throughput (manos por segundo)
fn bench_throughput(c: &mut Criterion) {
    let hands = generate_random_7card_hands(10000);

    c.bench_function("throughput_10k_hands", |b| {
        b.iter(|| {
            let mut count = 0u64;
            for hand in &hands {
                let rank = evaluate_7cards(hand);
                count += rank.value() as u64;
            }
            black_box(count)
        })
    });
}

criterion_group!(
    benches,
    bench_evaluate_5cards,
    bench_evaluate_7cards,
    bench_batch_evaluation,
    bench_specific_hands,
    bench_throughput,
);

criterion_main!(benches);
