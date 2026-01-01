//! # Calculador de Equity
//!
//! Módulo para cálculo de equity (probabilidad de ganar) usando simulación Monte Carlo
//! optimizado con SIMD AVX2 y paralelización Rayon.
//!
//! ## Características
//!
//! - Simulación Monte Carlo con muestreo aleatorio del deck
//! - Optimización SIMD AVX2 para evaluación paralela de manos
//! - Paralelización con Rayon para aprovechar 16 threads del Ryzen 3800X
//! - Early stopping cuando la convergencia < 0.1%
//!
//! ## Uso
//!
//! ```rust,ignore
//! use poker_math::equity_calculator::{calculate_equity, EquityResult};
//!
//! // Calcular equity de AA vs KK preflop
//! let hero = ["As", "Ah"];
//! let villain = ["Ks", "Kh"];
//! let result = calculate_equity(&hero, &villain, &[], 10000);
//! println!("Hero equity: {:.2}%", result.hero_equity * 100.0);
//! ```
//!
//! ## Performance
//!
//! Objetivo: > 100K simulaciones/segundo en Ryzen 7 3800X

mod monte_carlo;
mod simd;

pub use monte_carlo::{
    calculate_equity, calculate_equity_multiway, simulate_single, EquityResult, MonteCarloConfig,
};
pub use simd::{is_avx2_available, SimdEvaluator};
