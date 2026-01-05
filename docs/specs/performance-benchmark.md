# Performance Benchmark - 10M Hands

## Objetivo

Documentar los resultados de las pruebas de carga masiva con 10 millones de manos sinteticas para validar que el sistema cumple con los criterios de rendimiento establecidos.

## Configuracion de Hardware

| Componente | Especificacion |
|------------|----------------|
| CPU | AMD Ryzen 7 3800X (8 cores, 16 threads) |
| RAM | 64GB DDR4 |
| Storage | NVMe SSD |
| OS | Windows 11 / macOS |

## Configuracion de Software

| Componente | Version |
|------------|---------|
| Rust | 1.75+ |
| DuckDB | 0.9+ |
| Rayon | 1.10+ |

## Criterios de Aceptacion (Issue #65)

| Criterio | Objetivo | Estado |
|----------|----------|--------|
| Pipeline completo (generacion + persistencia + carga) | < 5 minutos | PENDIENTE |
| Todas las consultas de estadisticas | < 500ms | PENDIENTE |
| Uso de memoria durante consultas | < 32GB | PENDIENTE |
| Sin errores OOM | 0 errores | PENDIENTE |

## Ejecucion del Benchmark

### Comando

```bash
cd backend
cargo run --release --example benchmark_10m
```

### Parametros

| Parametro | Valor |
|-----------|-------|
| Total de manos | 10,000,000 |
| Tamano de batch | 1,000,000 |
| Semilla | 42 |
| Threads | 16 |

## Resultados

> Los resultados se actualizaran despues de ejecutar el benchmark

### Fase 1: Generacion de Manos

| Metrica | Valor |
|---------|-------|
| Tiempo total | - |
| Velocidad | - manos/seg |

### Fase 2: Persistencia en Parquet

| Metrica | Valor |
|---------|-------|
| Tiempo total | - |
| Velocidad | - manos/seg |
| Archivos creados | - |
| Tamano total | - MB |

### Fase 3: Carga en DuckDB

| Metrica | Valor |
|---------|-------|
| Tiempo total | - |
| Velocidad | - manos/seg |

### Fase 4: Benchmark de Queries

| Query | Descripcion | Tiempo (ms) | Estado |
|-------|-------------|-------------|--------|
| COUNT hands_metadata | Total de manos | - | - |
| COUNT hands_actions | Total de acciones | - | - |
| VPIP by player | Calculo de VPIP top 10 | - | - |
| PFR by player | Calculo de PFR top 10 | - | - |
| 3Bet frequency | Frecuencia de 3Bet | - | - |
| Filter by stake | Filtro por stake NL10 | - | - |
| Filter by date range | Filtro ultimos 30 dias | - | - |
| Join metadata + actions | Join para Hero | - | - |
| Aggregation by action type | Conteo por tipo | - | - |
| Complex stats query | Stats completas | - | - |

### Uso de Memoria

| Metrica | Valor |
|---------|-------|
| Memoria pico | - GB |
| Memoria final | - GB |

## Queries de Benchmark

### 1. VPIP (Voluntarily Put money In Pot)

```sql
SELECT player_id, 
       COUNT(CASE WHEN action_type IN ('CALL', 'RAISE', 'BET') THEN 1 END) * 100.0 / COUNT(*) as vpip
FROM hands_actions 
WHERE street = 'PREFLOP'
GROUP BY player_id 
ORDER BY COUNT(*) DESC 
LIMIT 10
```

### 2. PFR (Pre-Flop Raise)

```sql
SELECT player_id, 
       COUNT(CASE WHEN action_type = 'RAISE' THEN 1 END) * 100.0 / COUNT(*) as pfr
FROM hands_actions 
WHERE street = 'PREFLOP'
GROUP BY player_id 
ORDER BY COUNT(*) DESC 
LIMIT 10
```

### 3. 3Bet Frequency

```sql
SELECT player_id,
       COUNT(CASE WHEN action_type = 'RAISE' AND action_sequence >= 2 THEN 1 END) * 100.0 / 
       NULLIF(COUNT(CASE WHEN action_sequence >= 2 THEN 1 END), 0) as three_bet
FROM hands_actions 
WHERE street = 'PREFLOP'
GROUP BY player_id 
HAVING COUNT(*) > 100
ORDER BY COUNT(*) DESC 
LIMIT 10
```

### 4. Complex Stats Query

```sql
SELECT 
    player_id,
    COUNT(DISTINCT hand_id) as total_hands,
    COUNT(CASE WHEN street = 'PREFLOP' AND action_type IN ('CALL', 'RAISE', 'BET') THEN 1 END) * 100.0 /
        NULLIF(COUNT(CASE WHEN street = 'PREFLOP' THEN 1 END), 0) as vpip,
    COUNT(CASE WHEN street = 'PREFLOP' AND action_type = 'RAISE' THEN 1 END) * 100.0 /
        NULLIF(COUNT(CASE WHEN street = 'PREFLOP' THEN 1 END), 0) as pfr,
    SUM(amount_cents) as total_amount
FROM hands_actions
GROUP BY player_id
HAVING COUNT(DISTINCT hand_id) > 100
ORDER BY total_hands DESC
LIMIT 20
```

## Optimizaciones Aplicadas

### DuckDB

- `PRAGMA threads=16` - Aprovecha los 16 threads del Ryzen 3800X
- `PRAGMA memory_limit='48GB'` - Limita uso de memoria a 48GB
- `PRAGMA enable_object_cache=true` - Cache de objetos agresivo

### Parquet

- Compresion ZSTD nivel 3
- Row groups de 100,000 filas
- Particionado por batch

### Generador Sintetico

- Paralelizacion con Rayon (16 threads)
- Semilla determinista para reproducibilidad
- Batches de 1M para control de memoria

## Conclusiones

> Se completara despues de ejecutar el benchmark

## Recomendaciones de Optimizacion

> Se completara despues de analizar los resultados

---

*Documento generado para Issue #65 - Fase 4.1 Rendimiento y Escalabilidad*

