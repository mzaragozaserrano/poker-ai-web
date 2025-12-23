//! # Parquet Writer Module
//!
//! Módulo para escritura de datos en formato Parquet con características avanzadas:
//! - Particionamiento por fecha (year=YYYY/month=MM/day=DD/)
//! - Clustering por player_id para optimizar consultas
//! - Compresión ZSTD para balance entre velocidad y ratio
//! - Schema Arrow compatible con DuckDB
//!
//! ## Estrategia de Particionamiento
//! Los datos se particionan por fecha para:
//! - Permitir carga incremental eficiente
//! - Facilitar borrado de datos antiguos
//! - Optimizar queries con predicados temporales
//!
//! ## Clustering
//! Dentro de cada partición, los datos se ordenan por:
//! 1. player_id (para queries por jugador)
//! 2. timestamp (para mantener orden temporal)

use anyhow::{Context, Result};
use arrow::array::{
    ArrayRef, BooleanArray, Int64Array, RecordBatch, StringArray, TimestampMicrosecondArray,
    UInt8Array,
};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use chrono::{Datelike, NaiveDateTime, Utc};
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::WriterProperties;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::schema::{HandAction, HandMetadata};

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuración para escritura de archivos Parquet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParquetWriteConfig {
    /// Directorio base donde se almacenarán los archivos
    pub base_path: PathBuf,

    /// Nivel de compresión ZSTD (1-22, mayor = más compresión pero más lento)
    pub compression_level: i32,

    /// Tamaño del grupo de filas (row group) en Parquet
    /// Valores típicos: 100_000 - 1_000_000
    pub row_group_size: usize,

    /// Habilitar estadísticas de columnas (para pruning)
    pub enable_statistics: bool,

    /// Habilitar diccionario de encoding (reduce tamaño para strings repetidos)
    pub enable_dictionary: bool,
}

impl Default for ParquetWriteConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("data"),
            compression_level: 3, // Balance entre velocidad y compresión
            row_group_size: 500_000,
            enable_statistics: true,
            enable_dictionary: true,
        }
    }
}

impl ParquetWriteConfig {
    /// Crea una nueva configuración con un path base personalizado
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            ..Default::default()
        }
    }

    /// Ajusta el nivel de compresión
    pub fn with_compression_level(mut self, level: i32) -> Self {
        self.compression_level = level.clamp(1, 22);
        self
    }

    /// Ajusta el tamaño del row group
    pub fn with_row_group_size(mut self, size: usize) -> Self {
        self.row_group_size = size;
        self
    }
}

// ============================================================================
// PARTITIONING
// ============================================================================

/// Información de particionamiento por fecha
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatePartition {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DatePartition {
    /// Crea una partición a partir de un timestamp
    pub fn from_timestamp(timestamp: &NaiveDateTime) -> Self {
        Self {
            year: timestamp.year(),
            month: timestamp.month(),
            day: timestamp.day(),
        }
    }

    /// Genera el path relativo para esta partición
    /// Formato: year=YYYY/month=MM/day=DD/
    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(format!(
            "year={}/month={:02}/day={:02}",
            self.year, self.month, self.day
        ))
    }

    /// Genera el nombre del archivo Parquet para esta partición
    /// Formato: hands_YYYY_MM_DD_HHMMSS.parquet
    pub fn to_filename(&self) -> String {
        let now = Utc::now();
        format!(
            "hands_{}_{:02}_{:02}_{}.parquet",
            self.year,
            self.month,
            self.day,
            now.format("%H%M%S")
        )
    }
}

// ============================================================================
// SCHEMA DEFINITIONS
// ============================================================================

/// Schema Arrow para la tabla hands_metadata
pub fn hands_metadata_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("hand_id", DataType::Utf8, false),
        Field::new("session_id", DataType::Utf8, true),
        Field::new("tournament_id", DataType::Utf8, true),
        Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Microsecond, None),
            false,
        ),
        Field::new("stake", DataType::Utf8, false),
        Field::new("format", DataType::Utf8, false),
        Field::new("table_name", DataType::Utf8, false),
        Field::new("blind_level", DataType::Int64, false),
        Field::new("button_seat", DataType::UInt8, false),
    ]))
}

/// Schema Arrow para la tabla hands_actions
pub fn hands_actions_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("action_id", DataType::Utf8, false),
        Field::new("hand_id", DataType::Utf8, false),
        Field::new("player_id", DataType::Utf8, false),
        Field::new("street", DataType::Utf8, false),
        Field::new("action_type", DataType::Utf8, false),
        Field::new("amount_cents", DataType::Int64, false),
        Field::new("is_hero_action", DataType::Boolean, false),
        Field::new("ev_cents", DataType::Int64, true),
        Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Microsecond, None),
            false,
        ),
    ]))
}

// ============================================================================
// PARQUET WRITER
// ============================================================================

/// Writer para archivos Parquet con particionamiento y clustering
pub struct ParquetWriter {
    config: ParquetWriteConfig,
}

impl ParquetWriter {
    /// Crea un nuevo writer con la configuración especificada
    pub fn new(config: ParquetWriteConfig) -> Self {
        Self { config }
    }

    /// Crea un writer con configuración por defecto
    pub fn default() -> Self {
        Self::new(ParquetWriteConfig::default())
    }

    /// Escribe un lote de metadata de manos a Parquet
    ///
    /// # Arguments
    /// * `metadata` - Vector de metadata de manos a escribir
    ///
    /// # Returns
    /// Path del archivo Parquet creado
    pub fn write_hands_metadata(&self, metadata: Vec<HandMetadata>) -> Result<PathBuf> {
        if metadata.is_empty() {
            return Err(anyhow::anyhow!("No metadata to write"));
        }

        // Agrupar por fecha y ordenar
        let mut grouped = self.group_and_sort_metadata(metadata);

        // Procesar cada grupo (una partición por fecha)
        let mut written_files = Vec::new();
        for (partition, mut batch) in grouped.drain(..) {
            let path = self.write_metadata_partition(&partition, &mut batch)?;
            written_files.push(path);
        }

        // Retornar el primer archivo (o podríamos retornar todos)
        Ok(written_files.into_iter().next().unwrap())
    }

    /// Escribe un lote de acciones de manos a Parquet
    ///
    /// # Arguments
    /// * `actions` - Vector de acciones a escribir
    /// * `timestamps` - Mapa de hand_id -> timestamp para particionamiento
    ///
    /// # Returns
    /// Path del archivo Parquet creado
    pub fn write_hands_actions(
        &self,
        actions: Vec<HandAction>,
        timestamps: &std::collections::HashMap<String, NaiveDateTime>,
    ) -> Result<PathBuf> {
        if actions.is_empty() {
            return Err(anyhow::anyhow!("No actions to write"));
        }

        // Agrupar por fecha, ordenar por player_id y timestamp
        let mut grouped = self.group_and_sort_actions(actions, timestamps);

        // Procesar cada grupo
        let mut written_files = Vec::new();
        for (partition, mut batch) in grouped.drain(..) {
            let path = self.write_actions_partition(&partition, &mut batch)?;
            written_files.push(path);
        }

        Ok(written_files.into_iter().next().unwrap())
    }

    // ========================================================================
    // PRIVATE METHODS - Grouping & Sorting
    // ========================================================================

    /// Agrupa metadata por fecha y ordena por timestamp
    fn group_and_sort_metadata(
        &self,
        mut metadata: Vec<HandMetadata>,
    ) -> Vec<(DatePartition, Vec<HandMetadata>)> {
        // Ordenar por timestamp primero
        metadata.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Agrupar por fecha
        let mut groups: std::collections::HashMap<DatePartition, Vec<HandMetadata>> =
            std::collections::HashMap::new();

        for meta in metadata {
            // Parsear timestamp ISO 8601 a NaiveDateTime
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&meta.timestamp) {
                let naive_dt = dt.naive_utc();
                let partition = DatePartition::from_timestamp(&naive_dt);
                groups.entry(partition).or_insert_with(Vec::new).push(meta);
            }
        }

        groups.into_iter().collect()
    }

    /// Agrupa acciones por fecha y ordena por player_id + timestamp
    fn group_and_sort_actions(
        &self,
        mut actions: Vec<HandAction>,
        timestamps: &std::collections::HashMap<String, NaiveDateTime>,
    ) -> Vec<(DatePartition, Vec<HandAction>)> {
        // Ordenar por player_id primero, luego por hand_id (proxy para timestamp)
        actions.sort_by(|a, b| {
            a.player_id
                .cmp(&b.player_id)
                .then_with(|| a.hand_id.cmp(&b.hand_id))
        });

        // Agrupar por fecha
        let mut groups: std::collections::HashMap<DatePartition, Vec<HandAction>> =
            std::collections::HashMap::new();

        for action in actions {
            if let Some(ts) = timestamps.get(&action.hand_id) {
                let partition = DatePartition::from_timestamp(ts);
                groups
                    .entry(partition)
                    .or_insert_with(Vec::new)
                    .push(action);
            }
        }

        groups.into_iter().collect()
    }

    // ========================================================================
    // PRIVATE METHODS - Writing
    // ========================================================================

    /// Escribe una partición de metadata
    fn write_metadata_partition(
        &self,
        partition: &DatePartition,
        metadata: &[HandMetadata],
    ) -> Result<PathBuf> {
        // Construir path completo
        let partition_dir = self.config.base_path.join(partition.to_path());
        fs::create_dir_all(&partition_dir)
            .with_context(|| format!("Failed to create partition directory: {:?}", partition_dir))?;

        let file_path = partition_dir.join(partition.to_filename());

        // Convertir a RecordBatch Arrow
        let batch = self.metadata_to_record_batch(metadata)?;

        // Escribir archivo Parquet
        self.write_record_batch(&file_path, batch, hands_metadata_schema())?;

        Ok(file_path)
    }

    /// Escribe una partición de acciones
    fn write_actions_partition(
        &self,
        partition: &DatePartition,
        actions: &[HandAction],
    ) -> Result<PathBuf> {
        // Construir path completo
        let partition_dir = self.config.base_path.join(partition.to_path());
        fs::create_dir_all(&partition_dir)
            .with_context(|| format!("Failed to create partition directory: {:?}", partition_dir))?;

        let file_path = partition_dir.join(partition.to_filename());

        // Convertir a RecordBatch Arrow
        let batch = self.actions_to_record_batch(actions)?;

        // Escribir archivo Parquet
        self.write_record_batch(&file_path, batch, hands_actions_schema())?;

        Ok(file_path)
    }

    /// Escribe un RecordBatch a archivo Parquet
    fn write_record_batch(
        &self,
        path: &Path,
        batch: RecordBatch,
        schema: Arc<Schema>,
    ) -> Result<()> {
        let file = File::create(path)
            .with_context(|| format!("Failed to create file: {:?}", path))?;

        // Configurar propiedades de escritura
        let props = WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::try_new(
                self.config.compression_level,
            )?))
            .set_max_row_group_size(self.config.row_group_size)
            .set_statistics_enabled(
                parquet::file::properties::EnabledStatistics::Chunk,
            )
            .set_dictionary_enabled(self.config.enable_dictionary)
            .set_writer_version(parquet::file::properties::WriterVersion::PARQUET_2_0)
            .build();

        // Crear writer y escribir batch
        let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props))
            .context("Failed to create ArrowWriter")?;

        writer
            .write(&batch)
            .context("Failed to write record batch")?;

        writer.close().context("Failed to close writer")?;

        Ok(())
    }

    // ========================================================================
    // PRIVATE METHODS - Arrow Conversion
    // ========================================================================

    /// Convierte metadata a RecordBatch Arrow
    fn metadata_to_record_batch(&self, metadata: &[HandMetadata]) -> Result<RecordBatch> {

        // Construir arrays
        let hand_ids: Vec<Option<String>> = metadata.iter().map(|m| Some(m.hand_id.clone())).collect();
        let session_ids: Vec<Option<String>> = metadata.iter().map(|m| m.session_id.clone()).collect();
        let tournament_ids: Vec<Option<String>> = metadata.iter().map(|m| m.tournament_id.clone()).collect();
        
        let timestamps: Vec<i64> = metadata
            .iter()
            .map(|m| {
                chrono::DateTime::parse_from_rfc3339(&m.timestamp)
                    .map(|dt| dt.timestamp_micros())
                    .unwrap_or(0)
            })
            .collect();
        
        let stakes: Vec<Option<String>> = metadata.iter().map(|m| Some(m.stake.clone())).collect();
        let formats: Vec<Option<String>> = metadata.iter().map(|m| Some(m.format.to_string())).collect();
        let table_names: Vec<Option<String>> = metadata.iter().map(|m| Some(m.table_name.clone())).collect();
        let blind_levels: Vec<i64> = metadata.iter().map(|m| m.blind_level).collect();
        let button_seats: Vec<u8> = metadata.iter().map(|m| m.button_seat).collect();

        // Crear arrays Arrow
        let hand_id_array: ArrayRef = Arc::new(StringArray::from(hand_ids));
        let session_id_array: ArrayRef = Arc::new(StringArray::from(session_ids));
        let tournament_id_array: ArrayRef = Arc::new(StringArray::from(tournament_ids));
        let timestamp_array: ArrayRef = Arc::new(TimestampMicrosecondArray::from(timestamps));
        let stake_array: ArrayRef = Arc::new(StringArray::from(stakes));
        let format_array: ArrayRef = Arc::new(StringArray::from(formats));
        let table_name_array: ArrayRef = Arc::new(StringArray::from(table_names));
        let blind_level_array: ArrayRef = Arc::new(Int64Array::from(blind_levels));
        let button_seat_array: ArrayRef = Arc::new(UInt8Array::from(button_seats));

        // Crear RecordBatch
        let batch = RecordBatch::try_new(
            hands_metadata_schema(),
            vec![
                hand_id_array,
                session_id_array,
                tournament_id_array,
                timestamp_array,
                stake_array,
                format_array,
                table_name_array,
                blind_level_array,
                button_seat_array,
            ],
        )
        .context("Failed to create RecordBatch for metadata")?;

        Ok(batch)
    }

    /// Convierte acciones a RecordBatch Arrow
    fn actions_to_record_batch(&self, actions: &[HandAction]) -> Result<RecordBatch> {
        let n = actions.len();

        // Construir arrays
        let action_ids: Vec<Option<String>> = actions.iter().map(|a| Some(a.action_id.clone())).collect();
        let hand_ids: Vec<Option<String>> = actions.iter().map(|a| Some(a.hand_id.clone())).collect();
        let player_ids: Vec<Option<String>> = actions.iter().map(|a| Some(a.player_id.clone())).collect();
        let streets: Vec<Option<String>> = actions.iter().map(|a| Some(a.street.to_string())).collect();
        let action_types: Vec<Option<String>> = actions.iter().map(|a| Some(a.action_type.to_string())).collect();
        let amount_cents: Vec<i64> = actions.iter().map(|a| a.amount_cents).collect();
        let is_hero_actions: Vec<bool> = actions.iter().map(|a| a.is_hero_action).collect();
        let ev_cents: Vec<Option<i64>> = actions.iter().map(|a| a.ev_cents).collect();
        
        // Para timestamp, usamos el actual como placeholder (en producción vendría del metadata)
        let now = chrono::Utc::now().timestamp_micros();
        let timestamps: Vec<i64> = vec![now; n];

        // Crear arrays Arrow
        let action_id_array: ArrayRef = Arc::new(StringArray::from(action_ids));
        let hand_id_array: ArrayRef = Arc::new(StringArray::from(hand_ids));
        let player_id_array: ArrayRef = Arc::new(StringArray::from(player_ids));
        let street_array: ArrayRef = Arc::new(StringArray::from(streets));
        let action_type_array: ArrayRef = Arc::new(StringArray::from(action_types));
        let amount_cents_array: ArrayRef = Arc::new(Int64Array::from(amount_cents));
        let is_hero_action_array: ArrayRef = Arc::new(BooleanArray::from(is_hero_actions));
        let ev_cents_array: ArrayRef = Arc::new(Int64Array::from(ev_cents));
        let timestamp_array: ArrayRef = Arc::new(TimestampMicrosecondArray::from(timestamps));

        // Crear RecordBatch
        let batch = RecordBatch::try_new(
            hands_actions_schema(),
            vec![
                action_id_array,
                hand_id_array,
                player_id_array,
                street_array,
                action_type_array,
                amount_cents_array,
                is_hero_action_array,
                ev_cents_array,
                timestamp_array,
            ],
        )
        .context("Failed to create RecordBatch for actions")?;

        Ok(batch)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_date_partition_creation() {
        let dt = NaiveDateTime::parse_from_str("2024-03-15 14:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let partition = DatePartition::from_timestamp(&dt);

        assert_eq!(partition.year, 2024);
        assert_eq!(partition.month, 3);
        assert_eq!(partition.day, 15);
    }

    #[test]
    fn test_date_partition_path() {
        let partition = DatePartition {
            year: 2024,
            month: 3,
            day: 15,
        };

        let path = partition.to_path();
        assert_eq!(path, PathBuf::from("year=2024/month=03/day=15"));
    }

    #[test]
    fn test_date_partition_filename() {
        let partition = DatePartition {
            year: 2024,
            month: 3,
            day: 15,
        };

        let filename = partition.to_filename();
        assert!(filename.starts_with("hands_2024_03_15_"));
        assert!(filename.ends_with(".parquet"));
    }

    #[test]
    fn test_parquet_write_config_defaults() {
        let config = ParquetWriteConfig::default();

        assert_eq!(config.base_path, PathBuf::from("data"));
        assert_eq!(config.compression_level, 3);
        assert_eq!(config.row_group_size, 500_000);
        assert!(config.enable_statistics);
        assert!(config.enable_dictionary);
    }

    #[test]
    fn test_parquet_write_config_builder() {
        let config = ParquetWriteConfig::new("/tmp/test")
            .with_compression_level(5)
            .with_row_group_size(100_000);

        assert_eq!(config.base_path, PathBuf::from("/tmp/test"));
        assert_eq!(config.compression_level, 5);
        assert_eq!(config.row_group_size, 100_000);
    }

    #[test]
    fn test_write_empty_metadata_fails() {
        let temp_dir = TempDir::new().unwrap();
        let config = ParquetWriteConfig::new(temp_dir.path());
        let writer = ParquetWriter::new(config);

        let result = writer.write_hands_metadata(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_empty_actions_fails() {
        let temp_dir = TempDir::new().unwrap();
        let config = ParquetWriteConfig::new(temp_dir.path());
        let writer = ParquetWriter::new(config);

        let timestamps = std::collections::HashMap::new();
        let result = writer.write_hands_actions(vec![], &timestamps);
        assert!(result.is_err());
    }
}

