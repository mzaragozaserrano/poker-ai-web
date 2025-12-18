//! # Poker Ranges Parser
//! 
//! Módulo para parsear y gestionar rangos estratégicos en formato HandRangeDSL.
//! 
//! Soporta:
//! - Archivos Markdown con frontmatter YAML
//! - Notación compacta de rangos (AA:1, KK:0.9, AKs:0.8)
//! - Análisis de desviaciones entre acciones reales y rangos GTO
//! - Integración con sistema de detección de leaks
//! 
//! ## Formato HandRangeDSL
//! Los rangos se definen en `docs/ranges/preflop-ranges.md` con sintaxis:
//! ```markdown
//! ---
//! position: BTN
//! action: Open
//! ---
//! AA:1,KK:0.9,AKs:0.8,...
//! ```

// TODO: Implementar módulos (Fase 2.1 según roadmap)
// pub mod dsl_parser;
// pub mod range_analyzer;
// pub mod leak_detector;

// pub use dsl_parser::RangeParser;
// pub use range_analyzer::RangeAnalyzer;
// pub use leak_detector::LeakDetector;

