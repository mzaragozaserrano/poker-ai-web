# Esquema de Base de Datos (DuckDB + Parquet)

Optimizado para operaciones vectorizadas, almacenamiento columnar in-memory (64GB RAM) y análisis avanzado de winrate/ROI.

## 1. Identidad y Jugadores

### Tabla: `players`
Consolidación de identidad única para el usuario y oponentes.
| Columna | Tipo | Descripción |
| :--- | :--- | :--- |
| `player_id` | UUID (PK) | Identificador único interno. |
| `display_name` | VARCHAR | Nombre principal para la UI. |
| `is_hero` | BOOLEAN | **Flag crítico:** `true` para `thesmoy`. Permite filtros rápidos "Hero Only". |
| `notes` | TEXT | Notas generales del jugador. |

### Tabla: `player_aliases`
Gestión de múltiples nicknames y cuentas (multi-sala).
| Columna | Tipo | Descripción |
| :--- | :--- | :--- |
| `alias_id` | UUID (PK) | Identificador del alias. |
| `player_id` | UUID (FK) | Relación con la identidad única. |
| `site_name` | ENUM | 'Winamax', 'PokerStars', etc. |
| `site_nickname`| VARCHAR | Nickname exacto en la sala (ej: 'thesmoy'). |

---

## 2. Estructura Analítica (Hands & Actions)

### Tabla: `hands_metadata` (Dimension)
| Columna | Tipo | Descripción |
| :--- | :--- | :--- |
| `hand_id` | VARCHAR (PK) | ID original de Winamax. |
| `session_id` | UUID (FK) | Relación con la sesión de juego. |
| `tournament_id`| VARCHAR (FK) | ID del torneo o NULL para Cash. |
| `timestamp` | TIMESTAMP | Fecha/hora UTC de la mano. |
| `stake` | VARCHAR | Nivel (ej: 'NL2', 'NL10'). |
| `format` | ENUM | 'CASH', 'MTT', 'SNG', 'EXPRESSO'. |
| `table_name` | VARCHAR | Nombre de la mesa. |
| `blind_level` | BIGINT | SB en centavos enteros. |
| `button_seat` | UTINYINT | Posición del botón (0-5). |

### Tabla: `hands_actions` (Fact Table)
| Columna | Tipo | Descripción |
| :--- | :--- | :--- |
| `action_id` | UUID (PK) | Generado internamente. |
| `hand_id` | VARCHAR (FK) | Relación con metadata. |
| `player_id` | UUID (FK) | Jugador que realiza la acción. |
| `street` | ENUM | Preflop, Flop, Turn, River. |
| `action_type` | ENUM | Fold, Call, Raise, Bet, Check. |
| `amount_cents` | BIGINT | Cantidad en centavos enteros. |
| `is_hero_action`| BOOLEAN | Redundancia para filtros rápidos de `thesmoy`. |
| `ev_cents` | BIGINT | Valor esperado (EV) calculado en situaciones All-in. |

---

## 3. Economía y Resultados

### Tabla: `cash_sessions`
| Columna | Tipo | Descripción |
| :--- | :--- | :--- |
| `session_id` | UUID (PK) | Identificador de la sesión. |
| `start_time` | TIMESTAMP | Inicio de la sesión. |
| `end_time` | TIMESTAMP | Fin de la sesión. |
| `net_won_cents`| BIGINT | Resultado real en centavos. |
| `ev_won_cents` | BIGINT | Resultado esperado (EV). |
| `rake_cents` | BIGINT | Rake total pagado. |
| `rakeback_cents`| BIGINT | Estimación de rakeback basado en volumen/status. |
| `bb_100` | DOUBLE | Winrate real (bb/100). |
| `ev_bb_100` | DOUBLE | Winrate esperado (EV bb/100). |

### Tabla: `tournaments` & `tournament_results`
| Columna | Tipo | Descripción |
| :--- | :--- | :--- |
| `tournament_id`| VARCHAR (PK) | ID único de Winamax. |
| `buyin_cents` | BIGINT | Costo de entrada sin rake. |
| `rake_cents` | BIGINT | Rake de inscripción. |
| `bounty_won` | BIGINT | Total acumulado de bounties. |
| `finish_pos` | INTEGER | Posición final. |
| `roi_real` | DOUBLE | ROI real considerando bounties y premios. |

---

## 4. Estrategia de Almacenamiento y Rendimiento

### Particionamiento Parquet
Para manejar 10M+ de manos sin degradación:
* **Partición por Fecha:** Los archivos Parquet se guardarán en carpetas `/data/year=YYYY/month=MM/day=DD/`.
* **Clustering:** Dentro de cada archivo Parquet, los datos se agruparán por `player_id` para acelerar las consultas de agregación de estadísticas.

### Índices Recomendados (DuckDB)
Dado que la base de datos es mayoritariamente In-Memory:
1.  **Índice B-Tree** en `hands_metadata(timestamp)` para filtros temporales rápidos.
2.  **Índice Hash** en `hands_actions(hand_id)` para joins ultra-rápidos entre hechos y dimensiones.
3.  **Índice Compuesto** en `hands_actions(player_id, street)` para el motor de cálculo de estadísticas (VPIP, PFR).

### Optimización Ryzen 3800X
* **Vectorización:** DuckDB procesará las columnas de centavos (`amount_cents`) usando instrucciones SIMD para cálculos de profit masivos.