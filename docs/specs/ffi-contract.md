# Contratos de API y Frontera FFI (Rust ↔ Python)

Para maximizar el rendimiento del Ryzen 3800X y sus 16 hilos, la comunicación entre la orquestación (Python) y el núcleo de cálculo (Rust) se realiza mediante **PyO3** utilizando **Apache Arrow** para el intercambio de datos masivos sin penalización de serialización.

---

## 1. Interfaz FFI (PyO3)

### 1.1 Ingestión Masiva (`ingest_batch`)
- **Firma:** `fn ingest_batch(files: Vec<String>) -> PyResult<usize>`
- **Proceso:** Rust lanza un pool de hilos (`Rayon`) para procesar los archivos. Cada hilo produce un record batch de Arrow que se inserta directamente en DuckDB.

### 1.2 Motor de Equidad (`get_equity`)
- **Firma:** `fn get_equity(hero: String, villain_range: String, board: String, iterations: u32) -> PyResult<f64>`
- **Parámetros:**
    - `hero`: Mano de dos cartas (ej: "AhKd").
    - `villain_range`: Rango en formato **HandRangeDSL** (ver sección 2).
    - `board`: Cartas comunitarias (ej: "Qh7s2c").
    - `iterations`: Número de simulaciones Monte Carlo (default: 100,000).

---

## 2. Definición del Esquema Apache Arrow

Los datos parseados por Rust se entregan a Python/DuckDB siguiendo este esquema estricto de `HandHistoryBatch`:

| Campo | Tipo Arrow | Nullable | Descripción |
| :--- | :--- | :--- | :--- |
| `hand_id` | `Utf8` | No | ID único de Winamax. |
| `timestamp` | `Timestamp(Micro, UTC)` | No | Momento de la mano. |
| `player_id` | `Utf8` | No | Nickname del jugador. |
| `street` | `Int8` | No | 0:Pre, 1:Flop, 2:Turn, 3:River. |
| `action_type` | `Int8` | No | Enum: 0:Fold, 1:Call, 2:Raise, 3:Bet, 4:Check. |
| `amount_cents` | `Int64` | No | Cantidad en centavos enteros. |
| `is_hero` | `Boolean` | No | `true` si el jugador es `thesmoy`. |
| `cards` | `FixedSizeList(Utf8, 2)` | Sí | Cartas del jugador (si hay showdown). |

**Batch Size Optimizada:** 10,000 manos por bloque para maximizar el uso de la caché L3 del Ryzen.

---

## 3. Sintaxis de Rangos (HandRangeDSL)

El motor de equidad en Rust interpreta el formato textual para `villain_range` y los rangos de `thesmoy`.
- **Referencia Completa:** Ver `docs/specs/range-spec.md` para gramática, pesos (frecuencias) y validaciones de suma de frecuencias ≤ 1.0.
- **Carga de Datos:** El backend cargará las situaciones definidas en `docs/ranges/preflop-ranges.md` al iniciar la sesión.

---

## 4. Gestión de Errores y Timeouts

### 4.1 Códigos de Error (Rust -> Python)
| Código | Tipo de Error | Descripción |
| :--- | :--- | :--- |
| `101` | `IO_ERROR` | No se puede acceder a la ruta de `thesmoy\history`. |
| `102` | `PARSER_ERROR` | Formato de Winamax irreconocible o corrupto. |
| `201` | `INVALID_RANGE` | Error de sintaxis en `villain_range`. |
| `202` | `SIM_TIMEOUT` | La simulación excedió el tiempo límite (ver 4.2). |

### 4.2 Política de Timeouts y Reintentos
- **Simulaciones:** Si una consulta de equidad tarda más de **500ms**, Rust debe abortar y devolver `SIM_TIMEOUT` para no bloquear el Event Loop de FastAPI.
- **Ingestión:** No hay timeout, pero se debe reportar progreso mediante un callback o contador atómico.
- **Reintentos:** Python solo reintentará en caso de `IO_ERROR` (archivo bloqueado temporalmente por Winamax) con un máximo de 3 intentos y backoff de 100ms.