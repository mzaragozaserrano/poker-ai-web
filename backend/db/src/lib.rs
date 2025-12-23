//! # Poker Database Module
//!
//! Módulo para gestión de datos con DuckDB e integración Parquet.
//!
//! Características:
//! - In-memory DuckDB para análisis columnar ultra-rápido
//! - Persistencia con Parquet (formato columnar comprimido)
//! - Arrow para intercambio de datos sin serialización
//! - Esquema Star Schema optimizado para lectura
//!
//! ## Data Model
//! - Tabla Fact: `hands_actions`
//! - Tabla Wide: `player_stats_flat`
//! - Índices optimizados para Ryzen 3800X
//!
//! ## In-Memory Strategy
//! Los 64GB de RAM disponibles permiten mantener toda la base de datos
//! "caliente" en memoria para latencia cero.

// Módulos implementados
pub mod schema;
pub mod connection;

// TODO: Implementar módulos restantes (Fase 1.3 según roadmap)
// pub mod parquet_io;
// pub mod query;

// Re-exports principales
pub use schema::{
    ActionType, CashSession, GameFormat, HandAction, HandMetadata, HandMetadataBuilder, Player,
    PlayerAlias, SiteName, Street, Tournament, TournamentResult,
};

pub use connection::{DbConfig, DbConnection, DbStats};

// pub use parquet_io::{ParquetReader, ParquetWriter};
