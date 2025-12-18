//! # Poker Parsers
//!
//! Módulo de parsing ultra-rápido para historiales de Winamax.
//!
//! Utiliza una Máquina de Estados Finitos (FSM) para procesar líneas de historiales
//! de forma eficiente con paralelización multihilo mediante Rayon.
//!
//! ## Features
//! - FSM para análisis de historiales Winamax
//! - Paralelización con Rayon (16 threads en Ryzen 3800X)
//! - Soporte para Cash Games NLHE 6-max
//! - File watching con notificación en tiempo real

// TODO: Implementar módulos (Fase 1.2 según roadmap)
// pub mod fsm;
// pub mod winamax;

// pub use fsm::HandParser;
// pub use winamax::WinamaxHistoryParser;
