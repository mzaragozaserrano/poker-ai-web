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

## Estado: EN PROGRESO (Parcialmente Completado)

## Tareas Completadas
- [x] Crear schema.sql con todas las tablas definidas
- [x] Crear tabla hands_metadata (Dimension Table)
- [x] Crear tabla hands_actions (Fact Table)
- [x] Crear tablas adicionales (players, player_aliases, cash_sessions, tournaments, tournament_results)
- [x] Crear índices optimizados (B-Tree, Hash, Compuestos)
- [x] Crear estructuras Rust para todas las tablas
- [x] Crear módulo connection.rs para gestión de DuckDB
- [x] Implementar tests unitarios (13 tests, 9 pasando)

## Problemas Encontrados
- **Foreign Keys en DuckDB**: Las foreign keys causan errores "Cannot alter entry" al intentar crear tablas relacionadas
- **Resource Deadlock**: Múltiples statements ejecutados en secuencia causan deadlocks en DuckDB
- **Tipos UUID**: DuckDB no soporta UUID nativamente, se usa VARCHAR en su lugar
- **Vistas y Funciones**: Sintaxis no compatible, eliminadas del schema inicial

## Tareas Pendientes
- [ ] Resolver problema de foreign keys (eliminarlas o cambiar approach)
- [ ] Resolver deadlocks en init_schema_embedded
- [ ] Hacer que todos los tests pasen (actualmente 9/13)
- [ ] Implementar vistas como queries Rust en lugar de SQL
- [ ] Validar esquema completo contra especificaciones

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

## Archivos Creados/Modificados
- `backend/db/sql/schema.sql` - Definiciones de tablas e índices (NUEVO - 220 líneas)
- `backend/db/src/schema.rs` - Estructuras Rust para tablas (NUEVO - 596 líneas)
- `backend/db/src/connection.rs` - Gestión de conexiones DuckDB (NUEVO - 400+ líneas)
- `backend/db/src/lib.rs` - Exportar módulos schema y connection
- `backend/db/Cargo.toml` - Agregar dependencias uuid y chrono
- `backend/db/migrations/` - Directorio creado para futuras migraciones

## Rama
feat/issue-8-star-schema

## PR
Pendiente de creación