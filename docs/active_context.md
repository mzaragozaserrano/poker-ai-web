# Context Activo - Sesión en Curso

## Current Focus
**Issue #2:** Configuración de entorno Python (FastAPI + PyO3)

## Task Description
Configurar el entorno Python para la API FastAPI y el puente FFI con Rust. Crear la estructura de servidor-api/ con Poetry, FastAPI, uvicorn, PyO3 y DuckDB.

## Key Requirements
- Crear estructura de proyecto server-api/ con Poetry
- Crear server-api/app/main.py como punto de entrada FastAPI
- Crear server-api/app/bridge/ para módulos PyO3
- Configurar pyproject.toml: FastAPI, uvicorn, pyo3, duckdb
- Script de build con maturin para compilar extensiones PyO3
- Variables de entorno: WINAMAX_HISTORY_PATH, DUCKDB_MEMORY_LIMIT

## Active Problems
- Issue #2: Configuración de entorno Python (FastAPI + PyO3)
- ✅ COMPLETADO: Workspace de Rust creado con estructura multimembre

## Recent Decisions
- Usar Poetry para gestión de dependencias Python
- Usar Maturin para compilar extensiones PyO3
- DuckDB con límite de memoria de 48GB (dejar margen de 16GB)

## Reference Docs
- `docs/project/architecture.md` - Arquitectura del proyecto
- `docs/specs/` - Especificaciones técnicas
- `.cursor/workflows/feature-workflow.md` - Flujo de desarrollo

## Implementation Status
- ✅ Paso 1: Rama creada (feat/issue-2-config-python-fastapi)
- ⏳ Paso 2: Crear estructura server-api/ con Poetry
- ⏳ Paso 3: Configurar pyproject.toml con dependencias
- ⏳ Paso 4: Crear scripts de build con maturin
- ⏳ Paso 5: Configurar variables de entorno

---
*Última actualización: 2025-12-19*

