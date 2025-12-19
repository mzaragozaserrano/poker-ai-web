//! # Poker Parsers
//!
//! Módulo de parsing ultra-rápido para historiales de Winamax.
//!
//! Utiliza una Máquina de Estados Finitos (FSM) para procesar líneas de historiales
//! de forma eficiente con paralelización multihilo mediante Rayon.
//!
//! ## Features
//! - FSM para análisis de historiales Winamax
//! - Lectura optimizada de archivos (std::fs::read + BufReader)
//! - Parser basado en bytes sin Regex para máximo rendimiento
//! - Paralelización con Rayon (16 threads en Ryzen 3800X)
//! - Soporte para Cash Games NLHE 6-max
//! - File watching con notificación en tiempo real
//!
//! ## Uso básico
//!
//! ```rust,no_run
//! use poker_parsers::{WinamaxParser, file_reader};
//!
//! // Lectura optimizada del archivo
//! let content = file_reader::read_file_optimized("history.txt").unwrap();
//! let text = String::from_utf8_lossy(&content.bytes);
//!
//! // Parsing con FSM
//! let mut parser = WinamaxParser::new();
//! let result = parser.parse(&text);
//!
//! for hand in result.hands {
//!     println!("Hand ID: {}", hand.hand_id);
//! }
//! ```
//!
//! ## Uso avanzado con bytes
//!
//! ```rust,no_run
//! use poker_parsers::{bytes_parser, file_reader};
//!
//! // Lectura línea por línea como bytes (sin validación UTF-8)
//! for line_result in file_reader::read_lines_bytes("history.txt").unwrap() {
//!     let line = line_result.unwrap();
//!     
//!     // Detección rápida de prefijos
//!     if bytes_parser::starts_with_bytes(&line, bytes_parser::tokens::WINAMAX_POKER) {
//!         println!("Nueva mano detectada");
//!     }
//! }
//! ```

pub mod bytes_parser;
pub mod file_reader;
pub mod fsm;
pub mod parallel_processor;
pub mod types;

pub use fsm::WinamaxParser;
pub use parallel_processor::{
    process_files_parallel, process_files_parallel_with_progress, BatchProcessingResult,
    CancellationToken, FileProcessingError, FileProcessingResult, ParallelProcessor,
    ProcessingConfig, ProcessingProgress,
};
pub use types::{
    Action, ActionType, Card, GameType, ParseResult, ParsedHand, ParserState, Player, Position,
    PotInfo, Street,
};
