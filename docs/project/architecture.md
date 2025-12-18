# Architecture & Tech Spec

## 1. Project Structure & Modules

La plataforma utiliza una **Arquitectura Local-First** con un núcleo de alto rendimiento:

```text
/
├── core-backend/             # Núcleo en Rust (Parser, Math Engine, FFI)
│   ├── src/parsers/          # FSM para historiales de Winamax
│   ├── src/math/             # Evaluadores SIMD y Monte Carlo
│   └── src/db/               # Integración con DuckDB
├── server-api/               # FastAPI (Orquestador Python)
│   ├── api/                  # Endpoints REST
│   └── bridge/               # PyO3 / FFI para llamar a Rust
├── frontend/                 # React + TypeScript + Vite
│   ├── src/features/         # Replayer, Stats, Dashboard
│   └── src/lib/canvas/       # Renderizado de mesa con Konva
└── data/                     # Persistencia Parquet y metadatos
```

---

## 2. Data Model (DuckDB + Parquet)

* **Fuente de Verdad**: Los datos se persisten localmente utilizando archivos **Parquet**, seleccionados por ser un formato columnar inmutable y altamente comprimido.
* **Motor de Ejecución**: Se utiliza **DuckDB** *in-process*, una base de datos analítica que elimina la latencia de red al integrarse directamente en la aplicación.
* **Esquema Star Schema (Optimizado para Lectura)**: La arquitectura utiliza un diseño híbrido para maximizar la velocidad de las consultas analíticas.

### Tabla: `hands_actions` (Fact Table)
* `hand_id`: Identificador único de la mano (PK).
* `player_id`: Identificador del jugador.
* `street`: Enum que define la etapa de la mano (Preflop, Flop, Turn, River).
* `action_type`: Enum para el tipo de acción (Bet, Call, Raise, Fold).
* `amount`: Cantidad en BigInt, almacenada en centavos para evitar errores de coma flotante.

### Tabla: `player_stats_flat` (Wide Table para Estadísticas)
* `player_name`: Nombre del jugador (PK).
* `vpip_count`: Contador de veces que el jugador puso dinero voluntariamente en el pozo.
* `pfr_count`: Contador de subidas preflop.
* `total_hands`: Número total de manos registradas para el jugador.
* `positional_data`: Estructura JSONB con estadísticas desglosadas por posición en la mesa.

---

## 3. Data Flow

### Flow de Ingesta
* **Detección**: Se utiliza la crate `notify` de Rust para detectar nuevos archivos en el directorio de historiales de Winamax mediante APIs nativas del sistema operativo.
* **Parsing**: Un worker multihilo mediante la biblioteca `Rayon` satura los 16 hilos del Ryzen 3800X, procesando líneas a través de una Máquina de Estados Finitos (FSM).
* **Carga**: Los datos se insertan en un buffer de memoria y se sincronizan con DuckDB, aprovechando los 64GB de RAM para mantener la base de datos "caliente".
* **Disponibilidad**: Los datos procesados quedan disponibles en DuckDB para consulta mediante endpoints REST cuando el usuario accede al dashboard de análisis.