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

pub mod schema;
pub mod connection;
pub mod parquet_io;
pub mod query;

pub use connection::DbConnection;
pub use schema::{HandAction, PlayerStatFlat};
pub use parquet_io::{ParquetReader, ParquetWriter};

