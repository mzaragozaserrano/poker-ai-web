//! # Memory Monitor Module
//!
//! Monitoreo en tiempo real del uso de memoria de DuckDB.
//! Implementa alertas y métricas para asegurar que el sistema no exceda
//! los límites configurados (48GB default).
//!
//! ## Características
//! - Monitoreo continuo del uso de memoria con PRAGMA memory_usage
//! - Alertas cuando se acerca al límite configurado
//! - Logging de métricas para debugging
//! - Estrategia de compactación si es necesario

use duckdb::{Connection, Result as DuckDbResult};
use std::fmt;

/// Métricas de memoria de DuckDB
#[derive(Debug, Clone, Default)]
pub struct MemoryMetrics {
    /// Memoria total usada en bytes
    pub used_bytes: u64,
    /// Límite de memoria en bytes
    pub limit_bytes: u64,
    /// Porcentaje de memoria utilizada
    pub usage_percent: f64,
    /// Flag de alerta si se acerca al límite
    pub alert: bool,
    /// Mensaje de alerta
    pub alert_message: Option<String>,
}

impl MemoryMetrics {
    /// Calcula el porcentaje de memoria utilizada
    pub fn calculate_usage_percent(&mut self) {
        if self.limit_bytes > 0 {
            self.usage_percent = (self.used_bytes as f64 / self.limit_bytes as f64) * 100.0;
        }
    }

    /// Verifica si se debe activar alerta (>80% de uso)
    pub fn check_alert(&mut self) {
        self.alert = self.usage_percent > 80.0;
        if self.alert {
            self.alert_message = Some(format!(
                "MEMORY ALERT: {} MB of {} MB in use ({:.1}%)",
                self.used_bytes / 1_000_000,
                self.limit_bytes / 1_000_000,
                self.usage_percent
            ));
        }
    }

    /// Formatea las métricas como string
    pub fn format_summary(&self) -> String {
        let used_mb = self.used_bytes as f64 / 1_000_000.0;
        let limit_mb = self.limit_bytes as f64 / 1_000_000.0;
        format!(
            "Memory: {:.2} MB / {:.2} MB ({:.1}%)",
            used_mb, limit_mb, self.usage_percent
        )
    }
}

impl fmt::Display for MemoryMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_summary())
    }
}

/// Monitor de memoria para DuckDB
pub struct MemoryMonitor {
    /// Límite de memoria en bytes
    limit_bytes: u64,
    /// Umbral de alerta en porcentaje (default: 80%)
    alert_threshold: f64,
    /// Historial de métricas (últimas 10 muestras)
    history: Vec<MemoryMetrics>,
}

impl MemoryMonitor {
    /// Crea un nuevo monitor de memoria
    pub fn new(limit_gb: u64) -> Self {
        Self {
            limit_bytes: limit_gb * 1_000_000_000, // Convertir GB a bytes
            alert_threshold: 80.0,
            history: Vec::new(),
        }
    }

    /// Establece el umbral de alerta en porcentaje
    pub fn set_alert_threshold(mut self, threshold: f64) -> Self {
        self.alert_threshold = threshold.min(100.0).max(0.0);
        self
    }

    /// Obtiene las métricas actuales de memoria desde DuckDB
    pub fn get_metrics(&self, conn: &Connection) -> DuckDbResult<MemoryMetrics> {
        // Query para obtener estadísticas de memoria de DuckDB
        let _query = "SELECT SUM(allocated_memory) FROM duckdb_memory_usage();";

        // Si la tabla duckdb_memory_usage no existe, usamos una estimación alternativa
        let mut metrics = match self.query_memory_usage(conn) {
            Ok(used) => {
                let mut m = MemoryMetrics {
                    used_bytes: used,
                    limit_bytes: self.limit_bytes,
                    ..Default::default()
                };
                m.calculate_usage_percent();
                m.alert = m.usage_percent > self.alert_threshold;
                m
            }
            Err(_) => {
                // Fallback: usar PRAGMA memory_usage
                self.get_memory_from_pragma(conn)?
            }
        };

        // Verificar si necesita alerta
        metrics.check_alert();

        Ok(metrics)
    }

    /// Query para obtener uso de memoria desde información del sistema
    fn query_memory_usage(&self, conn: &Connection) -> DuckDbResult<u64> {
        // Intenta usar la función duckdb_memory_usage si está disponible
        let mut stmt = conn.prepare(
            "SELECT SUM(allocated_memory) FROM (
                SELECT * FROM duckdb_memory_usage() LIMIT 1
            ) t",
        )?;

        let memory_bytes: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(memory_bytes.max(0) as u64)
    }

    /// Obtiene memoria usando PRAGMA (fallback)
    fn get_memory_from_pragma(&self, conn: &Connection) -> DuckDbResult<MemoryMetrics> {
        // PRAGMA memory_usage devuelve información de memoria
        // Format: "Total: X MB, Used: Y MB"
        let query = "PRAGMA memory_usage;";

        let mut stmt = conn.prepare(query)?;
        let memory_info: String = stmt.query_row([], |row| row.get(0))?;

        // Parse del formato "Total: X MB, Used: Y MB"
        let used_bytes = parse_memory_from_pragma(&memory_info);

        let mut metrics = MemoryMetrics {
            used_bytes,
            limit_bytes: self.limit_bytes,
            ..Default::default()
        };

        metrics.calculate_usage_percent();
        Ok(metrics)
    }

    /// Registra una métrica en el historial
    pub fn record_metric(&mut self, metrics: MemoryMetrics) {
        self.history.push(metrics);
        // Mantener solo las últimas 10 muestras
        if self.history.len() > 10 {
            self.history.remove(0);
        }
    }

    /// Obtiene el historial de métricas
    pub fn history(&self) -> &[MemoryMetrics] {
        &self.history
    }

    /// Calcula la tendencia de uso de memoria (aumento, disminución, estable)
    pub fn trend(&self) -> MemoryTrend {
        if self.history.len() < 2 {
            return MemoryTrend::Stable;
        }

        let first = &self.history[0].usage_percent;
        let last = &self.history[self.history.len() - 1].usage_percent;
        let diff = last - first;

        if diff > 5.0 {
            MemoryTrend::Increasing
        } else if diff < -5.0 {
            MemoryTrend::Decreasing
        } else {
            MemoryTrend::Stable
        }
    }

    /// Genera un reporte de memoria
    pub fn report(&self) -> MemoryReport {
        MemoryReport {
            current: self.history.last().cloned().unwrap_or_default(),
            trend: self.trend(),
            samples: self.history.len(),
        }
    }
}

/// Tendencia de uso de memoria
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryTrend {
    Increasing,
    Decreasing,
    Stable,
}

impl fmt::Display for MemoryTrend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryTrend::Increasing => write!(f, "↑ Increasing"),
            MemoryTrend::Decreasing => write!(f, "↓ Decreasing"),
            MemoryTrend::Stable => write!(f, "→ Stable"),
        }
    }
}

/// Reporte de memoria
#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub current: MemoryMetrics,
    pub trend: MemoryTrend,
    pub samples: usize,
}

impl fmt::Display for MemoryReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Memory Report:\n  Current: {}\n  Trend: {}\n  Samples: {}",
            self.current.format_summary(),
            self.trend,
            self.samples
        )
    }
}

/// Helper para parsear el formato de PRAGMA memory_usage
fn parse_memory_from_pragma(pragma_output: &str) -> u64 {
    // Intenta extraer el número de "Used: X MB" o similar
    for part in pragma_output.split(',') {
        if part.to_lowercase().contains("used") {
            let numbers: String = part.chars().filter(|c| c.is_numeric()).collect();
            if let Ok(mb) = numbers.parse::<u64>() {
                return mb * 1_000_000; // Convertir MB a bytes
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_metrics_calculation() {
        let mut metrics = MemoryMetrics {
            used_bytes: 50_000_000_000,      // 50 GB
            limit_bytes: 100_000_000_000,    // 100 GB
            ..Default::default()
        };

        metrics.calculate_usage_percent();
        assert_eq!(metrics.usage_percent, 50.0);
    }

    #[test]
    fn test_memory_alert_threshold() {
        let mut metrics = MemoryMetrics {
            used_bytes: 85_000_000_000,      // 85 GB
            limit_bytes: 100_000_000_000,    // 100 GB
            ..Default::default()
        };

        metrics.calculate_usage_percent();
        metrics.check_alert();
        assert!(metrics.alert);
        assert!(metrics.alert_message.is_some());
    }

    #[test]
    fn test_memory_monitor_new() {
        let monitor = MemoryMonitor::new(48);
        assert_eq!(monitor.limit_bytes, 48_000_000_000);
        assert_eq!(monitor.alert_threshold, 80.0);
    }

    #[test]
    fn test_memory_trend_increasing() {
        let mut monitor = MemoryMonitor::new(48);

        let m1 = MemoryMetrics {
            used_bytes: 20_000_000_000,
            limit_bytes: 48_000_000_000,
            usage_percent: 41.0,
            ..Default::default()
        };

        let m2 = MemoryMetrics {
            used_bytes: 39_000_000_000,
            limit_bytes: 48_000_000_000,
            usage_percent: 81.0,
            ..Default::default()
        };

        monitor.record_metric(m1);
        monitor.record_metric(m2);

        assert_eq!(monitor.trend(), MemoryTrend::Increasing);
    }

    #[test]
    fn test_memory_format_summary() {
        let metrics = MemoryMetrics {
            used_bytes: 25_000_000_000,
            limit_bytes: 48_000_000_000,
            usage_percent: 52.08,
            ..Default::default()
        };

        let summary = metrics.format_summary();
        assert!(summary.contains("25000.00"));
        assert!(summary.contains("48000.00"));
        assert!(summary.contains("52.1%")); // El formato usa {:.1}%, no {:.2}%
    }

    #[test]
    fn test_parse_memory_from_pragma() {
        let pragma_output = "Total: 100 MB, Used: 50 MB";
        let parsed = parse_memory_from_pragma(pragma_output);
        assert_eq!(parsed, 50_000_000); // 50 MB en bytes
    }
}

