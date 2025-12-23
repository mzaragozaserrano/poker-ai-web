# TAREA ACTIVA: ISSUE #8

## Título
1.3.1 Diseño del Esquema Star en DuckDB

## Descripción y Requisitos
Diseñar e implementar el esquema Star Schema optimizado para consultas analíticas. El sistema debe:
- Crear tabla hands_metadata (Dimension Table) con campos definidos en db-schema.md
- Crear tabla hands_actions (Fact Table) con campos definidos en db-schema.md
- Crear tablas adicionales: players, player_aliases, cash_sessions
- Crear índices optimizados para consultas analíticas
- Usar tipos de datos apropiados (VARCHAR, TIMESTAMP, ENUM, BIGINT)

## Estado: EN PROGRESO

## Tareas Pendientes
- [ ] Crear tabla hands_metadata (Dimension Table)
- [ ] Crear tabla hands_actions (Fact Table)
- [ ] Crear tablas adicionales (players, player_aliases, cash_sessions)
- [ ] Crear índices optimizados
- [ ] Validar esquema contra especificaciones

## Criterios de Aceptación
- [ ] El esquema Star está correctamente implementado en DuckDB
- [ ] Los tipos de datos son apropiados para análisis analítico
- [ ] Los índices mejoran significativamente el rendimiento de consultas
- [ ] El esquema sigue las especificaciones de db-schema.md

## Arquitectura Planificada
- **Base de Datos**: DuckDB (In-Memory con 64GB RAM)
- **Persistencia**: Apache Parquet (Particionado por fecha)
- **Índices**: B-Tree (timestamp), Hash (hand_id), Compuesto (player_id, street)
- **Optimización**: Vectorización SIMD para operaciones columnar

## Archivos a Crear/Modificar
- `backend/database/` - Nuevo directorio para esquema DuckDB
- `backend/database/schema.sql` - Definiciones de tablas e índices (NUEVO)
- `backend/database/migrations/` - Sistema de migraciones (NUEVO)

## Rama
feat/issue-8-star-schema

## PR
Pendiente de creación