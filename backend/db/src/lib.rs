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
pub mod connection;
pub mod inmemory;
pub mod memory_monitor;
pub mod parquet_loader;
pub mod parquet_reader;
pub mod parquet_writer;
pub mod schema;

// Re-exports principales
pub use schema::{
    ActionType, CashSession, GameFormat, HandAction, HandMetadata, HandMetadataBuilder, Player,
    PlayerAlias, SiteName, Street, Tournament, TournamentResult,
};

pub use connection::{DbConfig, DbConnection, DbStats};
pub use inmemory::{CacheStats, InMemoryOptimization, MemoryMaintenance, QueryOptimizer};
pub use memory_monitor::{MemoryMetrics, MemoryMonitor, MemoryReport, MemoryTrend};
pub use parquet_loader::{LoadResult, ParquetLoadConfig, ParquetLoader};
pub use parquet_reader::{
    CacheStats as ReaderCacheStats, ParquetReadConfig, ParquetReader, ReadResult,
};
pub use parquet_writer::{DatePartition, ParquetWriteConfig, ParquetWriter};
