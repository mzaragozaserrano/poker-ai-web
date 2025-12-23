# TAREA ACTIVA: ISSUE #11

## Título
1.3.2 Implementación de persistencia en formato Parquet

## Descripción y Requisitos
Implementar la capa de persistencia usando Apache Parquet para almacenamiento inmutable y comprimido. El sistema debe:
- Configurar escritura a Parquet con Arrow y compresión ZSTD/SNAPPY
- Implementar particionamiento por fecha (year=YYYY/month=MM/day=DD/)
- Implementar clustering por player_id y ordenamiento temporal
- Implementar lectura desde Parquet con carga incremental
- Validar integridad de datos al cargar

## Estado: EN PROGRESO

## Tareas Pendientes
- [ ] Configurar escritura a Parquet con Arrow y compresión ZSTD
- [ ] Implementar particionamiento por fecha en estructura de directorios
- [ ] Implementar clustering por player_id y ordenamiento temporal
- [ ] Implementar lectura desde Parquet con carga incremental
- [ ] Crear tests unitarios e integración
- [ ] Validar integridad de datos al cargar

## Criterios de Aceptación
- [ ] Los datos se escriben correctamente en formato Parquet
- [ ] El particionamiento por fecha funciona correctamente
- [ ] Los archivos Parquet se pueden leer y consultar en DuckDB
- [ ] La compresión reduce significativamente el tamaño de almacenamiento
- [ ] El clustering mejora el rendimiento de consultas por jugador

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
feat/issue-11-parquet-persistence

## Archivos a Crear/Modificar
- `backend/db/src/parquet_writer.rs` - Escritura de archivos Parquet con particionamiento (NUEVO)
- `backend/db/src/parquet_reader.rs` - Lectura incremental de archivos Parquet (NUEVO)
- `backend/db/src/lib.rs` - Exportar nuevos módulos reader/writer
- `backend/db/Cargo.toml` - Actualizar dependencias de Parquet/Arrow
- `backend/db/tests/parquet_tests.rs` - Tests unitarios e integración (NUEVO)