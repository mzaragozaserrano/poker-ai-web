# System Operations & Engineering Standards

Este documento define los estándares de operación, mantenimiento y calidad para garantizar la estabilidad del sistema bajo cargas de procesamiento masivas.

---

## 1. Targets de Rendimiento (Ryzen 3800X / 64GB)
- **Ingestión:** >150,000 manos/segundo (saturando los 16 hilos del procesador).
- **Latencia de Queries:** <100ms para agregaciones sobre 10M de manos (In-Memory).
- **Evaluación de Equidad:** >1M de simulaciones/segundo mediante AVX2.

---

## 2. Gestión de Logs y Observabilidad
El sistema utiliza una política de logs estructurados para facilitar la depuración local sin comprometer el rendimiento.

- **Niveles de Log:**
    - `ERROR`: Fallos críticos (pérdida de conexión a DB, error de FFI).
    - `WARN`: Problemas de parsing recuperables (línea inválida, formato desconocido).
    - `INFO`: Hitos de sesión (inicio de ingesta, resumen de manos procesadas).
- **Destino:** `%APPDATA%/winamax-analyzer/logs/`.
- **Rotación:** Archivos de 10MB con un máximo de 5 backups (rotación diaria automática).
- **Implementación:** `tracing` crate en Rust y `structlog` en Python.

---

## 3. Manejo de Fallos en Ingestión
Dado que los archivos de Winamax pueden ser bloqueados por el cliente de juego o corromperse:

- **Archivos Bloqueados:** Si un archivo está en uso, el sistema realizará 3 reintentos con un backoff exponencial (100ms, 500ms, 1s).
- **Líneas Inválidas:** El parser FSM debe saltar líneas mal formadas y registrar un `WARN` con el `HandId` afectado, continuando con la siguiente mano sin abortar el proceso.
- **Cuarentena:** Los archivos que causen un crash persistente del parser se moverán automáticamente a `.../history/quarantine/` para análisis manual.

---

## 4. Estrategia de Migración de Esquema
Para manejar actualizaciones de DuckDB o cambios en las columnas de Parquet:

- **Versionado:** Tabla interna `schema_version` en DuckDB.
- **Procedimiento:** 1. Backup automático del archivo `.duckdb` antes de la migración.
    2. Ejecución de scripts de `ALTER TABLE` vía FastAPI al detectar una versión inferior.
    3. Si el cambio es estructuralmente incompatible, se forzará un re-parsing masivo desde los archivos Parquet originales (fuente de verdad inmutable).

---

## 5. Estrategia de Testing y Benchmarking

### 5.1 Fixtures y Golden Files
- **Conjunto de Pruebas:** Utilizar subconjuntos del archivo `example_winamax.txt` para validar casos de:
    - Mesas de 5 jugadores (validación de salto de posición UTG).
    - Manos con Showdown vs. Manos foldeadas.
    - Cálculos de Rake (ej: mano #21819158-393 con 0.03€ de rake).
- **Golden Files:** Comparación de la salida JSON del parser contra resultados esperados verificados manualmente.

### 5.2 Benchmarking por Módulo
- **Parser:** Test de estrés con un dataset de 1M de manos sintéticas.
- **Equity Engine:** Test de velocidad comparativo: Monte Carlo estándar vs. optimización AVX2.
- **Consultas DuckDB:** Medición de latencia en frío (disco) vs. caliente (64GB RAM).

---

## 6. Configuración de Rutas (MANDATORIO)
- **Path de Historial:** `C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history`.
- **Path de DuckDB:** `%APPDATA%/winamax-analyzer/database.duckdb`.
- **Huge Pages:** Configuración manual de Windows para optimizar la tabla de búsqueda de 7 cartas en RAM.