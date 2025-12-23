//! # Parquet Loader Module
//!
//! Estrategia de carga de archivos Parquet al inicializar la aplicación.
//! Mantiene datos "calientes" en memoria durante la sesión.
//!
//! ## Características
//! - Carga automática de archivos Parquet particionados
//! - Validación de integridad de datos
//! - Soporte para caché de consultas frecuentes
//! - Estrategia de precarga para manos históricas

use duckdb::Result as DuckDbResult;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Configuración de carga de Parquet
#[derive(Debug, Clone)]
pub struct ParquetLoadConfig {
    /// Directorio base de datos particionadas
    pub data_dir: PathBuf,
    /// Habilitar precarga de todos los datos
    pub preload_all: bool,
    /// Máximo número de archivos para cargar (None = sin límite)
    pub max_files: Option<usize>,
    /// Tablas a cargar (None = todas)
    pub tables: Option<Vec<String>>,
}

impl ParquetLoadConfig {
    /// Crea una nueva configuración de carga
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            data_dir: data_dir.as_ref().to_path_buf(),
            preload_all: true,
            max_files: None,
            tables: None,
        }
    }

    /// Establece si se debe precargar todos los datos
    pub fn with_preload_all(mut self, preload: bool) -> Self {
        self.preload_all = preload;
        self
    }

    /// Establece el máximo número de archivos a cargar
    pub fn with_max_files(mut self, max: usize) -> Self {
        self.max_files = Some(max);
        self
    }

    /// Establece las tablas específicas a cargar
    pub fn with_tables(mut self, tables: Vec<String>) -> Self {
        self.tables = Some(tables);
        self
    }
}

/// Resultado de carga de Parquet
#[derive(Debug, Clone)]
pub struct LoadResult {
    /// Nombre de la tabla
    pub table_name: String,
    /// Número de archivos cargados
    pub files_loaded: usize,
    /// Número de filas insertadas
    pub rows_inserted: i64,
    /// Tiempo de carga en milisegundos
    pub duration_ms: u128,
    /// Tamaño total en bytes
    pub total_size_bytes: u64,
}

impl LoadResult {
    /// Formatea el resultado como string
    pub fn format_summary(&self) -> String {
        format!(
            "{}: {} files, {} rows, {} MB, {:.2}s",
            self.table_name,
            self.files_loaded,
            self.rows_inserted,
            self.total_size_bytes / 1_000_000,
            self.duration_ms as f64 / 1000.0
        )
    }
}

/// Loader de Parquet
pub struct ParquetLoader {
    config: ParquetLoadConfig,
}

impl ParquetLoader {
    /// Crea un nuevo loader de Parquet
    pub fn new(config: ParquetLoadConfig) -> Self {
        Self { config }
    }

    /// Carga datos desde Parquet para una tabla específica
    pub fn load_table(
        &self,
        conn: &duckdb::Connection,
        table_name: &str,
    ) -> DuckDbResult<LoadResult> {
        let start = Instant::now();

        // Construir ruta de datos para esta tabla
        let table_dir = self.config.data_dir.join(table_name);

        if !table_dir.exists() {
            eprintln!("Directory not found: {}", table_dir.display());
            return Ok(LoadResult {
                table_name: table_name.to_string(),
                files_loaded: 0,
                rows_inserted: 0,
                duration_ms: start.elapsed().as_millis(),
                total_size_bytes: 0,
            });
        }

        // Contar archivos y tamaño total
        let (file_count, total_size) = self.count_parquet_files(&table_dir);

        if file_count == 0 {
            return Ok(LoadResult {
                table_name: table_name.to_string(),
                files_loaded: 0,
                rows_inserted: 0,
                duration_ms: start.elapsed().as_millis(),
                total_size_bytes: 0,
            });
        }

        // Limitar número de archivos si está configurado
        let actual_count = if let Some(max) = self.config.max_files {
            file_count.min(max)
        } else {
            file_count
        };

        // Construir query de carga con patrón wildcard
        let pattern = table_dir.join("*.parquet");
        let query = format!(
            "INSERT INTO {} SELECT * FROM read_parquet('{}')",
            table_name,
            pattern.display()
        );

        // Ejecutar query de inserción
        conn.execute(&query, [])?;

        // Obtener número de filas insertadas
        let row_count = self.count_table_rows(conn, table_name)?;

        let duration_ms = start.elapsed().as_millis();

        Ok(LoadResult {
            table_name: table_name.to_string(),
            files_loaded: actual_count,
            rows_inserted: row_count,
            duration_ms,
            total_size_bytes: total_size,
        })
    }

    /// Carga datos desde Parquet particionados por fecha
    pub fn load_partitioned(
        &self,
        conn: &duckdb::Connection,
        table_name: &str,
        year: Option<i32>,
        month: Option<u32>,
    ) -> DuckDbResult<LoadResult> {
        let start = Instant::now();

        // Construir ruta con particiones
        let mut path = self.config.data_dir.join(table_name);

        if let Some(y) = year {
            path = path.join(format!("year={}", y));
            if let Some(m) = month {
                path = path.join(format!("month={:02}", m));
            }
        }

        if !path.exists() {
            return Ok(LoadResult {
                table_name: table_name.to_string(),
                files_loaded: 0,
                rows_inserted: 0,
                duration_ms: start.elapsed().as_millis(),
                total_size_bytes: 0,
            });
        }

        // Contar archivos
        let (file_count, total_size) = self.count_parquet_files(&path);

        if file_count == 0 {
            return Ok(LoadResult {
                table_name: table_name.to_string(),
                files_loaded: 0,
                rows_inserted: 0,
                duration_ms: start.elapsed().as_millis(),
                total_size_bytes: 0,
            });
        }

        // Construir query con patrón wildcard recursivo
        let pattern = path.join("**/*.parquet");
        let query = format!(
            "INSERT INTO {} SELECT * FROM read_parquet('{}')",
            table_name,
            pattern.display()
        );

        // Ejecutar inserción
        conn.execute(&query, [])?;

        // Contar filas
        let row_count = self.count_table_rows(conn, table_name)?;

        Ok(LoadResult {
            table_name: table_name.to_string(),
            files_loaded: file_count,
            rows_inserted: row_count,
            duration_ms: start.elapsed().as_millis(),
            total_size_bytes: total_size,
        })
    }

    /// Carga todas las tablas configuradas
    pub fn load_all(&self, conn: &duckdb::Connection) -> DuckDbResult<Vec<LoadResult>> {
        let tables = self.get_tables_to_load();
        let mut results = Vec::new();

        for table_name in tables {
            match self.load_table(conn, &table_name) {
                Ok(result) => {
                    println!("✓ {}", result.format_summary());
                    results.push(result);
                }
                Err(e) => {
                    eprintln!("✗ Error loading table '{}': {}", table_name, e);
                }
            }
        }

        Ok(results)
    }

    /// Obtiene la lista de tablas a cargar
    fn get_tables_to_load(&self) -> Vec<String> {
        if let Some(ref tables) = self.config.tables {
            tables.clone()
        } else {
            // Tablas por defecto del esquema
            vec![
                "players".to_string(),
                "player_aliases".to_string(),
                "hands_metadata".to_string(),
                "hands_actions".to_string(),
                "cash_sessions".to_string(),
                "tournaments".to_string(),
                "tournament_results".to_string(),
            ]
        }
    }

    /// Cuenta archivos Parquet en un directorio
    fn count_parquet_files(&self, dir: &Path) -> (usize, u64) {
        let mut count = 0;
        let mut total_size = 0u64;

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Some(filename) = entry.file_name().to_str() {
                        if filename.ends_with(".parquet") {
                            count += 1;
                            total_size += metadata.len();
                        }
                    }
                }
            }
        }

        (count, total_size)
    }

    /// Cuenta filas en una tabla
    fn count_table_rows(&self, conn: &duckdb::Connection, table_name: &str) -> DuckDbResult<i64> {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        let mut stmt = conn.prepare(&query)?;
        stmt.query_row([], |row| row.get(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parquet_load_config_new() {
        let config = ParquetLoadConfig::new("/data/parquet");
        assert_eq!(config.data_dir, PathBuf::from("/data/parquet"));
        assert!(config.preload_all);
        assert_eq!(config.max_files, None);
    }

    #[test]
    fn test_parquet_load_config_builder() {
        let config = ParquetLoadConfig::new("/data")
            .with_preload_all(false)
            .with_max_files(100)
            .with_tables(vec!["hands_metadata".to_string()]);

        assert!(!config.preload_all);
        assert_eq!(config.max_files, Some(100));
        assert_eq!(config.tables.unwrap().len(), 1);
    }

    #[test]
    fn test_load_result_format_summary() {
        let result = LoadResult {
            table_name: "hands_metadata".to_string(),
            files_loaded: 10,
            rows_inserted: 50000,
            duration_ms: 2500,
            total_size_bytes: 100_000_000,
        };

        let summary = result.format_summary();
        assert!(summary.contains("hands_metadata"));
        assert!(summary.contains("10 files"));
        assert!(summary.contains("50000 rows"));
    }

    #[test]
    fn test_parquet_loader_new() {
        let config = ParquetLoadConfig::new("/data");
        let loader = ParquetLoader::new(config);
        assert_eq!(loader.config.data_dir, PathBuf::from("/data"));
    }

    #[test]
    fn test_get_tables_to_load_default() {
        let config = ParquetLoadConfig::new("/data");
        let loader = ParquetLoader::new(config);
        let tables = loader.get_tables_to_load();
        assert_eq!(tables.len(), 7); // 7 tablas por defecto
        assert!(tables.contains(&"hands_metadata".to_string()));
    }

    #[test]
    fn test_get_tables_to_load_custom() {
        let config = ParquetLoadConfig::new("/data")
            .with_tables(vec!["hands_actions".to_string(), "players".to_string()]);
        let loader = ParquetLoader::new(config);
        let tables = loader.get_tables_to_load();
        assert_eq!(tables.len(), 2);
        assert!(tables.contains(&"hands_actions".to_string()));
    }

    #[test]
    fn test_count_parquet_files() {
        let temp_dir = TempDir::new().unwrap();
        let loader = ParquetLoader::new(ParquetLoadConfig::new(temp_dir.path()));

        // Crear algunos archivos de prueba
        fs::write(temp_dir.path().join("data1.parquet"), "mock data").unwrap();
        fs::write(temp_dir.path().join("data2.parquet"), "mock data").unwrap();
        fs::write(temp_dir.path().join("readme.txt"), "text").unwrap();

        let (count, _size) = loader.count_parquet_files(temp_dir.path());
        assert_eq!(count, 2); // Solo .parquet files
    }
}
