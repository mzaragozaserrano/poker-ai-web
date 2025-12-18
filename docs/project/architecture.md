# Architecture & Tech Spec

## 1. Project Structure & Modules

La plataforma utiliza una **Arquitectura Local-First** con un núcleo de alto rendimiento:

```text
/
├── core-backend/             # Núcleo en Rust (Parser, Math Engine, FFI)
│   ├── src/parsers/          # FSM para historiales de Winamax
│   ├── src/math/             # Evaluadores SIMD y Monte Carlo
│   ├── src/ranges/           # Parser de rangos HandRangeDSL
│   └── src/db/               # Integración con DuckDB
├── server-api/               # FastAPI (Orquestador Python)
│   ├── api/                  # Endpoints REST
│   └── bridge/               # PyO3 / FFI para llamar a Rust
├── frontend/                 # React + TypeScript + Vite
│   ├── src/features/         # Replayer, Stats, Dashboard
│   └── src/lib/canvas/       # Renderizado de mesa con Konva
├── docs/ranges/              # Rangos estratégicos GTO (preflop-ranges.md)
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

---

## 4. Sistema de Rangos Estratégicos

El sistema integra un motor de análisis de rangos que compara las acciones reales de **thesmoy** con rangos estratégicos GTO definidos en formato **HandRangeDSL**.

### 4.1 Formato HandRangeDSL
- **Sintaxis**: Rangos definidos en archivos Markdown con frontmatter YAML y notación compacta (`AA:1,KK:0.9,AKs:0.8`)
- **Ubicación**: Los rangos se almacenan en `docs/ranges/preflop-ranges.md` y se cargan al iniciar la sesión
- **Parser**: El backend Rust parsea los rangos y los expone mediante FFI para comparación con acciones reales

### 4.2 Integración FFI (Apache Arrow)
- **Intercambio de Datos**: Los datos parseados se transfieren entre Rust y Python mediante **Apache Arrow** para evitar penalización de serialización
- **Batch Size**: 10,000 manos por bloque para maximizar el uso de la caché L3 del Ryzen
- **Esquema Arrow**: Estructura estricta `HandHistoryBatch` con campos tipados (hand_id, timestamp, player_id, street, action_type, amount_cents, is_hero, cards)

### 4.3 Análisis de Desviación
- **Identificación de Situación**: Cada mano parseada se vincula con un `situationId` (ej: `SB_Open_Raise_01`) del rango estratégico correspondiente
- **Detección de Leaks**: Si una mano jugada tiene frecuencia 0.0 en el rango estratégico para esa acción, se marca como "Leak" en la UI
- **Estrategias Mixtas**: El sistema maneja acciones con múltiples frecuencias (ej: 3bet 28% + call marginal 2% + fold 70%)