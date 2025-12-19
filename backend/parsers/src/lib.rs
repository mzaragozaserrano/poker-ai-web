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
//!
//! ## Uso
//!
//! ```rust,no_run
//! use poker_parsers::WinamaxParser;
//!
//! let content = std::fs::read_to_string("history.txt").unwrap();
//! let mut parser = WinamaxParser::new();
//! let result = parser.parse(&content);
//!
//! for hand in result.hands {
//!     println!("Hand ID: {}", hand.hand_id);
//! }
//! ```

pub mod fsm;
pub mod types;

pub use fsm::WinamaxParser;
pub use types::{
    Action, ActionType, Card, GameType, ParseResult, ParsedHand, ParserState, Player, Position,
    PotInfo, Street,
};
