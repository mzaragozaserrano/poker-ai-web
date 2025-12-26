//! # Poker Math Engine
//!
//! Módulo de cálculos matemáticos para análisis de poker.
//!
//! Incluye:
//! - Evaluadores de manos optimizados (Cactus Kev + Lookup O(1))
//! - Simulaciones Monte Carlo paralelizadas con Rayon
//! - Cálculos de equidad con SIMD AVX2
//! - Análisis de probabilidades
//!
//! ## Performance
//! Diseñado para ejecutarse en Ryzen 3800X (16 cores) aprovechando:
//! - Lookup tables para evaluaciones O(1) (< 50ns)
//! - Rayon para paralelización de simulaciones (16 threads)
//! - SIMD AVX2 para operaciones vectorizadas
//! - Monte Carlo con convergencia temprana
//!
//! ## Módulos
//!
//! - `hand_evaluator`: Evaluador de manos de 5-7 cartas
//! - `equity_calculator`: Simulador Monte Carlo para cálculo de equity

pub mod equity_calculator;
pub mod hand_evaluator;

// Re-exports para conveniencia - Hand Evaluator
pub use hand_evaluator::{
    cards_to_index, evaluate, evaluate_5cards, evaluate_6cards, evaluate_7cards,
    evaluate_7cards_lookup, evaluate_from_strings, generate_lookup_table, index_to_cards,
    is_lookup_table_loaded, Card, Deck, HandCategory, HandRank, Rank, Suit, TOTAL_7CARD_COMBOS,
};

// Re-exports para conveniencia - Equity Calculator
pub use equity_calculator::{
    calculate_equity, calculate_equity_multiway, is_avx2_available, EquityResult, MonteCarloConfig,
    SimdEvaluator,
};
