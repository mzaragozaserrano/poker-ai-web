//! # Poker Math Engine
//!
//! Módulo de cálculos matemáticos para análisis de poker.
//!
//! Incluye:
//! - Evaluadores de manos optimizados
//! - Simulaciones Monte Carlo paralelizadas (próximamente)
//! - Cálculos de equidad (próximamente)
//! - Análisis de probabilidades (próximamente)
//!
//! ## Performance
//! Diseñado para ejecutarse en Ryzen 3800X (16 cores) aprovechando:
//! - Lookup tables para evaluaciones O(1)
//! - Rayon para paralelización de simulaciones
//! - SIMD AVX2 para operaciones vectorizadas (próximamente)
//!
//! ## Módulos
//!
//! - `hand_evaluator`: Evaluador de manos de 5-7 cartas

pub mod hand_evaluator;

// Re-exports para conveniencia
pub use hand_evaluator::{
    evaluate, evaluate_5cards, evaluate_6cards, evaluate_7cards, evaluate_from_strings, Card, Deck,
    HandCategory, HandRank, Rank, Suit,
};

// TODO: Implementar módulos adicionales (Fase 2.1 según roadmap)
// pub mod equity;
// pub mod monte_carlo;
