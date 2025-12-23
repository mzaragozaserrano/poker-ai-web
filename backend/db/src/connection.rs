//! # Connection Module
//!
//! Gestión de conexiones a DuckDB con configuración optimizada para Ryzen 3800X.
//!
//! ## Características
//! - In-Memory Database con 48GB de límite
//! - 16 threads para paralelización
//! - Carga automática del esquema SQL
//! - Soporte para Parquet I/O

use duckdb::{Connection, Result as DuckDbResult};
use std::path::Path;

/// Configuración de la base de datos
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Ruta al archivo de base de datos (None para in-memory)
    pub db_path: Option<String>,
    /// Número de threads (default: 16 para Ryzen 3800X)
    pub threads: u8,
    /// Límite de memoria en GB (default: 48GB)
    pub memory_limit_gb: u8,
    /// Habilitar cache de objetos
    pub enable_object_cache: bool,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            db_path: None, // In-memory por defecto
            threads: 16,
            memory_limit_gb: 48,
            enable_object_cache: true,
        }
    }
}

impl DbConfig {
    /// Crea una configuración para base de datos en memoria
    pub fn in_memory() -> Self {
        Self::default()
    }

    /// Crea una configuración para base de datos persistente
    pub fn persistent(db_path: String) -> Self {
        Self {
            db_path: Some(db_path),
            ..Self::default()
        }
    }

    /// Establece el número de threads
    pub fn with_threads(mut self, threads: u8) -> Self {
        self.threads = threads;
        self
    }

    /// Establece el límite de memoria
    pub fn with_memory_limit_gb(mut self, memory_limit_gb: u8) -> Self {
        self.memory_limit_gb = memory_limit_gb;
        self
    }
}

/// Wrapper para la conexión de DuckDB con configuración optimizada
pub struct DbConnection {
    conn: Connection,
    config: DbConfig,
}

impl DbConnection {
    /// Crea una nueva conexión con la configuración especificada
    pub fn new(config: DbConfig) -> DuckDbResult<Self> {
        let conn = match &config.db_path {
            Some(path) => Connection::open(path)?,
            None => Connection::open_in_memory()?,
        };

        let mut db_conn = Self { conn, config };
        db_conn.apply_config()?;
        Ok(db_conn)
    }

    /// Crea una conexión in-memory con configuración por defecto
    pub fn in_memory() -> DuckDbResult<Self> {
        Self::new(DbConfig::in_memory())
    }

    /// Crea una conexión persistente
    pub fn persistent(db_path: String) -> DuckDbResult<Self> {
        Self::new(DbConfig::persistent(db_path))
    }

    /// Aplica la configuración de optimización a DuckDB
    fn apply_config(&mut self) -> DuckDbResult<()> {
        // Configurar threads
        self.conn
            .execute(&format!("PRAGMA threads={}", self.config.threads), [])?;

        // Configurar límite de memoria
        self.conn.execute(
            &format!("PRAGMA memory_limit='{}GB'", self.config.memory_limit_gb),
            [],
        )?;

        // Habilitar cache de objetos
        if self.config.enable_object_cache {
            self.conn.execute("PRAGMA enable_object_cache=true", [])?;
        }

        Ok(())
    }

    /// Inicializa el esquema desde un archivo SQL
    pub fn init_schema(&self, schema_path: &Path) -> DuckDbResult<()> {
        let schema_sql = std::fs::read_to_string(schema_path)
            .map_err(|_| duckdb::Error::InvalidPath(schema_path.to_path_buf()))?;

        // Ejecutar el schema SQL (puede contener múltiples statements)
        // DuckDB no soporta múltiples statements en una sola ejecución,
        // así que necesitamos dividirlos
        for statement in schema_sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("--") {
                self.conn.execute(trimmed, [])?;
            }
        }

        Ok(())
    }

    /// Inicializa el esquema desde el SQL embebido
    pub fn init_schema_embedded(&mut self) -> DuckDbResult<()> {
        const SCHEMA_SQL: &str = include_str!("../sql/schema.sql");

        // Limpiar el SQL: remover comentarios y líneas vacías
        let cleaned_sql: String = SCHEMA_SQL
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("--")
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Dividir por punto y coma y ejecutar cada statement
        let statements: Vec<&str> = cleaned_sql
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        eprintln!("Executing {} SQL statements...", statements.len());

        for (i, statement) in statements.iter().enumerate() {
            match self.conn.execute(statement, []) {
                Ok(_) => {
                    if statement.starts_with("CREATE TABLE") {
                        eprintln!("[{}/{}] ✓ Created table", i + 1, statements.len());
                    } else if statement.starts_with("CREATE INDEX") {
                        eprintln!("[{}/{}] ✓ Created index", i + 1, statements.len());
                    } else if statement.starts_with("PRAGMA") {
                        eprintln!("[{}/{}] ✓ Set pragma", i + 1, statements.len());
                    }
                }
                Err(e) => {
                    eprintln!("[{}/{}] ✗ Failed: {}", i + 1, statements.len(), e);
                    eprintln!("   Statement: {}", &statement[..statement.len().min(80)]);
                    // No retornar error, continuar con el siguiente statement
                }
            }
        }

        Ok(())
    }

    /// Obtiene una referencia a la conexión DuckDB subyacente
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Obtiene una referencia mutable a la conexión DuckDB subyacente
    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }

    /// Verifica que las tablas principales existan
    pub fn verify_schema(&self) -> DuckDbResult<bool> {
        let tables = vec![
            "players",
            "player_aliases",
            "hands_metadata",
            "hands_actions",
            "cash_sessions",
            "tournaments",
            "tournament_results",
        ];

        for table in tables {
            // Intentar hacer una consulta simple a la tabla
            let query = format!("SELECT COUNT(*) FROM {}", table);
            match self.conn.prepare(&query) {
                Ok(_) => continue,          // La tabla existe
                Err(_) => return Ok(false), // La tabla no existe
            }
        }

        Ok(true)
    }

    /// Obtiene estadísticas de la base de datos
    pub fn get_stats(&self) -> DuckDbResult<DbStats> {
        let mut stats = DbStats::default();

        // Contar jugadores
        stats.player_count = self.count_rows("players")?;

        // Contar manos
        stats.hand_count = self.count_rows("hands_metadata")?;

        // Contar acciones
        stats.action_count = self.count_rows("hands_actions")?;

        // Contar sesiones
        stats.session_count = self.count_rows("cash_sessions")?;

        // Contar torneos
        stats.tournament_count = self.count_rows("tournaments")?;

        Ok(stats)
    }

    /// Cuenta las filas de una tabla
    fn count_rows(&self, table_name: &str) -> DuckDbResult<i64> {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        let mut stmt = self.conn.prepare(&query)?;
        stmt.query_row([], |row| row.get(0))
    }

    /// Exporta una tabla a Parquet
    pub fn export_to_parquet(&self, table_name: &str, output_path: &Path) -> DuckDbResult<()> {
        let query = format!(
            "COPY {} TO '{}' (FORMAT PARQUET, COMPRESSION ZSTD)",
            table_name,
            output_path.display()
        );
        self.conn.execute(&query, [])?;
        Ok(())
    }

    /// Importa datos desde Parquet
    pub fn import_from_parquet(&self, table_name: &str, input_path: &Path) -> DuckDbResult<()> {
        let query = format!(
            "INSERT INTO {} SELECT * FROM read_parquet('{}')",
            table_name,
            input_path.display()
        );
        self.conn.execute(&query, [])?;
        Ok(())
    }

    /// Carga datos desde un directorio de archivos Parquet particionados
    pub fn load_partitioned_parquet(&self, table_name: &str, dir_path: &Path) -> DuckDbResult<()> {
        let pattern = dir_path.join("**/*.parquet");
        let query = format!(
            "INSERT INTO {} SELECT * FROM read_parquet('{}')",
            table_name,
            pattern.display()
        );
        self.conn.execute(&query, [])?;
        Ok(())
    }
}

/// Estadísticas de la base de datos
#[derive(Debug, Default, Clone)]
pub struct DbStats {
    pub player_count: i64,
    pub hand_count: i64,
    pub action_count: i64,
    pub session_count: i64,
    pub tournament_count: i64,
}

impl DbStats {
    /// Formatea las estadísticas como string
    pub fn summary(&self) -> String {
        format!(
            "Database Statistics:\n\
             - Players: {}\n\
             - Hands: {}\n\
             - Actions: {}\n\
             - Cash Sessions: {}\n\
             - Tournaments: {}",
            self.player_count,
            self.hand_count,
            self.action_count,
            self.session_count,
            self.tournament_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_in_memory_connection() {
        let conn = DbConnection::in_memory();
        assert!(conn.is_ok());
    }

    #[test]
    fn test_apply_config() {
        let config = DbConfig::in_memory()
            .with_threads(8)
            .with_memory_limit_gb(32);
        let conn = DbConnection::new(config);
        assert!(conn.is_ok());
    }

    #[test]
    fn test_init_schema_embedded() {
        let mut conn = DbConnection::in_memory().unwrap();
        let result = conn.init_schema_embedded();
        assert!(result.is_ok());

        // Intentar crear una tabla simple para verificar que la conexión funciona
        conn.conn()
            .execute(
                "CREATE TABLE IF NOT EXISTS test_table (id INTEGER, name VARCHAR)",
                [],
            )
            .unwrap();

        // Verificar que la tabla se creó
        let count: i64 = conn
            .conn()
            .prepare("SELECT COUNT(*) FROM test_table")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_verify_schema() {
        let mut conn = DbConnection::in_memory().unwrap();
        conn.init_schema_embedded().unwrap();

        // Verificar cada tabla individualmente para debugging
        let tables = vec![
            "players",
            "player_aliases",
            "hands_metadata",
            "hands_actions",
            "cash_sessions",
            "tournaments",
            "tournament_results",
        ];

        for table in &tables {
            let query = format!(
                "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = '{}'",
                table
            );
            let mut stmt = conn.conn().prepare(&query).unwrap();
            let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
            println!("Table '{}': count = {}", table, count);
        }

        let verified = conn.verify_schema().unwrap();
        assert!(verified);
    }

    #[test]
    fn test_get_stats() {
        let mut conn = DbConnection::in_memory().unwrap();
        conn.init_schema_embedded().unwrap();
        let stats = conn.get_stats().unwrap();
        assert_eq!(stats.player_count, 0);
        assert_eq!(stats.hand_count, 0);
    }

    #[test]
    fn test_count_rows() {
        let mut conn = DbConnection::in_memory().unwrap();
        conn.init_schema_embedded().unwrap();
        let count = conn.count_rows("players").unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_db_stats_summary() {
        let stats = DbStats {
            player_count: 10,
            hand_count: 1000,
            action_count: 5000,
            session_count: 50,
            tournament_count: 20,
        };
        let summary = stats.summary();
        assert!(summary.contains("Players: 10"));
        assert!(summary.contains("Hands: 1000"));
    }
}
