# TAREA ACTIVA: ISSUE #9

## Título
1.3.3 Configuración de DuckDB para operación in-memory

## Descripción y Requisitos
Configurar DuckDB para operar íntegramente en memoria aprovechando los 64GB de RAM. El sistema debe:
- Configurar DuckDB en modo in-memory con límite de 48GB
- Optimizar configuración para Ryzen 3800X (16 threads)
- Implementar estrategia de carga de archivos Parquet
- Implementar gestión y monitoreo de memoria

## Estado: EN PROGRESO

## Tareas Completadas
- [x] Schema Star implementado en DuckDB (Issue #8)

## Tareas Pendientes
- [ ] Configurar DuckDB en modo in-memory con límite de 48GB
- [ ] Optimizar configuración para Ryzen 3800X (16 threads)
- [ ] Implementar estrategia de carga de Parquet
- [ ] Implementar gestión y monitoreo de memoria
- [ ] Validar rendimiento con 10M+ de manos

## Criterios de Aceptación
- [ ] DuckDB opera completamente en memoria sin swapping
- [ ] La configuración aprovecha los 16 hilos del Ryzen 3800X
- [ ] Los datos se cargan eficientemente desde Parquet
- [ ] El sistema puede manejar 10M+ de manos en memoria
- [ ] El uso de memoria se mantiene dentro de límites razonables

## Arquitectura Planificada
- **Base de Datos**: DuckDB (In-Memory con 64GB RAM)
- **Persistencia**: Apache Parquet (Particionado por fecha)
- **Índices**: B-Tree (timestamp), Hash (hand_id), Compuesto (player_id, street)
- **Optimización**: Vectorización SIMD para operaciones columnar

## Archivos Creados/Modificados
- `backend/db/sql/schema.sql` - Definiciones de tablas e índices (NUEVO - 220 líneas)
- `backend/db/src/schema.rs` - Estructuras Rust para tablas (NUEVO - 596 líneas)
- `backend/db/src/connection.rs` - Gestión de conexiones DuckDB (NUEVO - 400+ líneas)
- `backend/db/src/lib.rs` - Exportar módulos schema y connection
- `backend/db/Cargo.toml` - Agregar dependencias uuid y chrono
- `backend/db/migrations/` - Directorio creado para futuras migraciones

## Rama
feat/issue-9-duckdb-inmemory

## PR
Pendiente de creación

## Archivos a Crear/Modificar
- `backend/db/src/inmemory.rs` - Configuración y optimización in-memory
- `backend/db/src/memory_monitor.rs` - Monitoreo de memoria
- `backend/db/src/parquet_loader.rs` - Estrategia de carga de Parquet
- `backend/db/src/lib.rs` - Exportar nuevos módulos