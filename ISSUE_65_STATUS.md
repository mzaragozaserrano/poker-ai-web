# Issue #65 - Estado Actual y Continuación

**Issue:** 4.1.2 Ejecutar pruebas de carga masiva con 10M de manos  
**Branch:** `feat/issue-65-load-test-10m`  
**PR:** #72  
**Estado:** Script implementado, pendiente de ejecución en hardware objetivo

---

## 📋 Resumen del Estado

### ✅ Completado

1. **Script de Benchmark Implementado**
   - Archivo: `backend/db/examples/benchmark_10m.rs`
   - Generación de 10M manos en batches de 1M
   - Persistencia en Parquet con compresión ZSTD
   - Carga en DuckDB in-memory
   - 10 queries de benchmark (VPIP, PFR, 3Bet, filtros, joins)
   - Monitoreo de memoria durante ejecución
   - Generación automática de reporte markdown

2. **Documentación Creada**
   - Archivo: `docs/specs/performance-benchmark.md`
   - Template con estructura para resultados
   - Queries documentadas
   - Criterios de aceptación definidos

3. **Integración con Código Existente**
   - Usa `SyntheticGenerator` de `backend/parsers/src/synthetic_generator.rs`
   - Usa `ParquetWriter` de `backend/db/src/parquet_writer.rs`
   - Usa `DbConnection` con optimizaciones in-memory
   - Usa `MemoryMonitor` para tracking de RAM

### ⏳ Pendiente

1. **Ejecución del Benchmark**
   - Requiere hardware objetivo: Ryzen 7 3800X (16 threads) + 64GB RAM
   - Ejecutar: `cargo run --release --example benchmark_10m`
   - Tiempo estimado: 5-10 minutos

2. **Análisis de Resultados**
   - Verificar que cumple criterios de aceptación
   - Documentar métricas reales en `docs/specs/performance-benchmark.md`
   - Identificar cuellos de botella si los hay

3. **Optimizaciones (si es necesario)**
   - Ajustar parámetros de DuckDB
   - Optimizar queries lentas
   - Ajustar tamaño de batches si hay problemas de memoria

---

## 🖥️ Requisitos de Hardware

**OBLIGATORIO para ejecutar este benchmark:**

| Componente | Especificación Mínima |
|------------|----------------------|
| CPU | AMD Ryzen 7 3800X (8 cores, 16 threads) |
| RAM | 64GB DDR4 |
| Storage | NVMe SSD (recomendado) |
| OS | Windows 11 / Linux / macOS |

**Nota:** El benchmark está diseñado específicamente para este hardware. Ejecutarlo en hardware inferior puede causar:
- OOM (Out of Memory) errors
- Tiempos de ejecución mucho mayores
- Resultados no representativos

---

## 🚀 Instrucciones para Continuar

### Paso 1: Verificar Hardware

```bash
# Verificar CPU y threads disponibles
# Windows PowerShell:
Get-WmiObject Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors

# Linux/macOS:
lscpu | grep -E "CPU\(s\)|Thread|Core"
```

**Debe mostrar:** 8 cores, 16 threads (o equivalente)

```bash
# Verificar RAM disponible
# Windows PowerShell:
Get-WmiObject Win32_ComputerSystem | Select-Object TotalPhysicalMemory

# Linux/macOS:
free -h
```

**Debe mostrar:** Al menos 64GB de RAM disponible

### Paso 2: Preparar Entorno

```bash
# 1. Asegurarse de estar en la rama correcta
cd /ruta/al/proyecto/poker-ai-web
git checkout feat/issue-65-load-test-10m
git pull origin feat/issue-65-load-test-10m

# 2. Compilar en modo release (crítico para rendimiento)
cd backend
cargo build --release --example benchmark_10m

# 3. Verificar que compila sin errores
cargo check --example benchmark_10m
```

### Paso 3: Ejecutar Benchmark

```bash
# Ejecutar el benchmark completo
cd backend
cargo run --release --example benchmark_10m
```

**Tiempo estimado:** 5-10 minutos

**Durante la ejecución verás:**
- Progreso de generación por batches (10 batches de 1M manos)
- Tiempo de persistencia Parquet
- Tiempo de carga en DuckDB
- Resultados de cada query con tiempo de ejecución
- Uso de memoria pico y final

### Paso 4: Revisar Resultados

El script genera automáticamente:

1. **Reporte en consola** - Tabla formateada con todos los resultados
2. **Archivo markdown** - `data/benchmark_10m/benchmark_report.md`

```bash
# Ver el reporte generado
cat data/benchmark_10m/benchmark_report.md
```

### Paso 5: Verificar Criterios de Aceptación

El script verifica automáticamente y muestra en el reporte:

| Criterio | Objetivo | Verificación |
|----------|----------|--------------|
| Pipeline completo | < 5 minutos | ✅/❌ |
| Todas las queries | < 500ms | ✅/❌ |
| Uso de memoria | < 32GB | ✅/❌ |
| Sin OOM errors | 0 errores | ✅/❌ |

**Si todos los criterios pasan:**
- ✅ Issue #65 completada
- Actualizar `docs/specs/performance-benchmark.md` con resultados reales
- Hacer commit y merge del PR

**Si algún criterio falla:**
- Analizar qué falló (tiempo, memoria, queries)
- Revisar logs del benchmark
- Considerar optimizaciones (ver sección "Troubleshooting")

---

## 📁 Archivos Relevantes

### Código Implementado

- **`backend/db/examples/benchmark_10m.rs`** - Script principal del benchmark
  - Líneas 33-36: Configuración (TOTAL_HANDS, BATCH_SIZE, SEED)
  - Líneas 42-60: Estructura de resultados
  - Líneas 272-311: Generación y persistencia
  - Líneas 313-380: Carga en DuckDB
  - Líneas 382-450: Queries de benchmark
  - Líneas 452-520: Generación de reporte

- **`backend/parsers/src/synthetic_generator.rs`** - Generador de manos sintéticas
  - Usado por el benchmark para generar datos

- **`backend/db/src/parquet_writer.rs`** - Writer de Parquet
  - Usado para persistir datos

- **`backend/db/src/connection.rs`** - Conexión DuckDB
  - Configuración in-memory optimizada

- **`backend/db/src/memory_monitor.rs`** - Monitor de memoria
  - Tracking de uso de RAM durante ejecución

### Documentación

- **`docs/specs/performance-benchmark.md`** - Template de resultados
  - Actualizar con métricas reales después de ejecutar

- **`docs/project/active-context.md`** - Contexto del proyecto
  - Línea 7-12: Estado actual de Issue #65

---

## 🔍 Queries de Benchmark Incluidas

El script ejecuta 10 queries para medir rendimiento:

1. **COUNT hands_metadata** - Conteo total de manos
2. **COUNT hands_actions** - Conteo total de acciones
3. **VPIP by player** - Cálculo de VPIP para top 10 jugadores
4. **PFR by player** - Cálculo de PFR para top 10 jugadores
5. **3Bet frequency** - Frecuencia de 3Bet
6. **Filter by stake** - Filtro por stake NL10
7. **Filter by date range** - Filtro últimos 30 días
8. **Join metadata + actions** - Join para acciones del Hero
9. **Aggregation by action type** - Conteo por tipo y street
10. **Complex stats query** - Stats completas con múltiples métricas

Todas deben ejecutarse en < 500ms según criterios.

---

## 🐛 Troubleshooting

### Error: OOM (Out of Memory)

**Síntoma:** El proceso se mata o falla durante la carga en DuckDB

**Soluciones:**
1. Verificar que tienes 64GB RAM disponibles (no solo instalados)
2. Cerrar otras aplicaciones que consuman RAM
3. Reducir `BATCH_SIZE` en el código (línea 34) de 1M a 500K
4. Reducir `memory_limit_gb` en `DbConfig` (línea ~320)

### Error: Queries muy lentas (> 500ms)

**Síntoma:** Algunas queries tardan más de 500ms

**Soluciones:**
1. Verificar que DuckDB está usando 16 threads: `PRAGMA threads=16`
2. Ejecutar `ANALYZE` después de cargar datos (ya está en el código)
3. Verificar que los índices se crearon correctamente
4. Revisar el plan de ejecución: `EXPLAIN <query>`

### Error: Compilación falla

**Síntoma:** `cargo build` falla con errores

**Soluciones:**
1. Verificar versión de Rust: `rustc --version` (debe ser 1.75+)
2. Actualizar dependencias: `cargo update`
3. Limpiar build: `cargo clean && cargo build --release`

### Benchmark tarda mucho (> 10 minutos)

**Posibles causas:**
1. Hardware no es el objetivo (menos threads/RAM)
2. Storage lento (no NVMe SSD)
3. Sistema operativo con swap activo

**Solución:** Verificar que estás en el hardware objetivo. Si no, este benchmark no es representativo.

---

## 📊 Estructura de Resultados Esperada

El reporte generado incluirá:

```markdown
# Performance Benchmark Results - 10M Hands

## Test Configuration
- Total Hands: 10,000,000
- Batch Size: 1,000,000
- Seed: 42
- Hardware: Ryzen 7 3800X (16 threads) + 64GB RAM

## Results Summary

### Phase 1: Generation
- Time: ~XX.XXs
- Speed: ~XXXXX hands/sec

### Phase 2: Parquet Persistence
- Time: ~XX.XXs
- Speed: ~XXXXX hands/sec
- Files Created: 20
- Total Size: ~XXX.XX MB

### Phase 3: DuckDB Load
- Time: ~XX.XXs
- Speed: ~XXXXX hands/sec

### Phase 4: Query Benchmarks
- [Tabla con tiempos de cada query]

### Memory Usage
- Peak: ~XX.XX GB
- Final: ~XX.XX GB

## Acceptance Criteria
- [Tabla con PASS/FAIL de cada criterio]
```

---

## ✅ Checklist para Completar Issue #65

- [ ] Verificar hardware objetivo (16 threads, 64GB RAM)
- [ ] Compilar en modo release: `cargo build --release --example benchmark_10m`
- [ ] Ejecutar benchmark: `cargo run --release --example benchmark_10m`
- [ ] Revisar reporte generado en `data/benchmark_10m/benchmark_report.md`
- [ ] Verificar que todos los criterios pasan (Pipeline < 5min, Queries < 500ms, RAM < 32GB)
- [ ] Actualizar `docs/specs/performance-benchmark.md` con resultados reales
- [ ] Hacer commit con resultados: `git add docs/specs/performance-benchmark.md && git commit -m "docs: add benchmark results for 10M hands"`
- [ ] Push y merge del PR #72

---

## 🔗 Referencias

- **Issue GitHub:** #65
- **Pull Request:** #72
- **Branch:** `feat/issue-65-load-test-10m`
- **Documentación relacionada:**
  - `docs/project/roadmap.md` - Fase 4.1 Rendimiento y Escalabilidad
  - `docs/specs/performance-benchmark.md` - Template de resultados
  - `docs/project/active-context.md` - Contexto actual del proyecto

---

## 💡 Notas Adicionales

1. **Semilla determinista:** El benchmark usa `SEED = 42` para reproducibilidad. Los resultados deben ser consistentes entre ejecuciones.

2. **Batches:** Se generan 10 batches de 1M manos cada uno para controlar memoria. Si tienes problemas, puedes reducir el tamaño.

3. **Parquet files:** Se generan en `./data/benchmark_10m/batch_XXX/`. Puedes eliminarlos después si no los necesitas (ocupan ~XXX MB).

4. **DuckDB in-memory:** Los datos se cargan en memoria, no se persisten en disco. Al terminar el script, los datos se pierden (esto es intencional para el benchmark).

5. **Monitoreo de memoria:** El script mide memoria pico durante queries. Si supera 32GB, el criterio falla.

---

**Última actualización:** 2024-12-XX  
**Estado:** Script implementado, pendiente ejecución en hardware objetivo

