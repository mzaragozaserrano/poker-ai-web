# Contratos de API y Frontera FFI (Rust - Python)

Para maximizar el rendimiento del Ryzen 3800X y sus 16 hilos, la comunicación entre la orquestación (Python) y el núcleo de cálculo (Rust) se realiza mediante **PyO3** utilizando **Apache Arrow** para el intercambio de datos masivos sin penalización de serialización.

---

## 1. Arquitectura del Puente FFI

```text
┌─────────────────────────────────────────────────────────────────┐
│                         FastAPI (Python)                        │
│                        server-api/app/                          │
├─────────────────────────────────────────────────────────────────┤
│                       Bridge Module                             │
│                   app/bridge/__init__.py                        │
│              (Wrapper Pythonic sobre FFI)                       │
├─────────────────────────────────────────────────────────────────┤
│                      PyO3 Interface                             │
│                  poker_ffi (cdylib)                             │
│                backend/ffi/src/lib.rs                           │
├─────────────────────────────────────────────────────────────────┤
│    poker-parsers    │    poker-math    │    poker-db           │
│   (FSM + Rayon)     │  (SIMD + Monte)  │  (DuckDB + Arrow)     │
└─────────────────────────────────────────────────────────────────┘
```

### 1.1 Compilación del Módulo

```bash
# Compilar módulo nativo para desarrollo
cd backend/ffi
maturin develop

# Compilar para producción (wheel)
maturin build --release
```

---

## 2. Interfaz FFI (PyO3)

### 2.1 Funciones de Parsing

#### `parse_winamax_files(files: Vec<String>) -> PyResult<PyParseResult>`

Parsea archivos de historial Winamax en paralelo usando Rayon (16 threads).

**Parámetros:**
- `files`: Lista de rutas a archivos de historial

**Retorna:**
- `PyParseResult` con:
  - `total_hands: int` - Manos parseadas
  - `successful_files: int` - Archivos exitosos
  - `failed_files: int` - Archivos con error
  - `elapsed_ms: int` - Tiempo de procesamiento

**Ejemplo Python:**
```python
from app.bridge import parse_files

result = parse_files(["history1.txt", "history2.txt"])
print(f"Parseadas {result['total_hands']} manos en {result['elapsed_ms']}ms")
```

#### `parse_winamax_with_details(files: Vec<String>) -> PyResult<Vec<PyHandSummary>>`

Parsea archivos y retorna información detallada de cada mano.

**Retorna:**
- Lista de `PyHandSummary`:
  - `hand_id: str` - ID único Winamax
  - `timestamp: str` - Fecha/hora
  - `table_name: str` - Nombre de mesa
  - `player_count: int` - Jugadores
  - `hero_played: bool` - Si thesmoy participó
  - `total_pot_cents: int` - Pot total

---

### 2.2 Funciones de Equity

#### `calculate_equity(hero, villain, board, iterations) -> PyResult<PyEquityResult>`

Calcula equity usando simulación Monte Carlo con SIMD AVX2.

**Parámetros:**
- `hero: str` - Cartas del héroe (ej: "AhKd")
- `villain: str` - Cartas del villano (ej: "QsQh")
- `board: str` - Cartas comunitarias (ej: "Qh7s2c", vacío para preflop)
- `iterations: int` - Simulaciones (default: 100,000)

**Retorna:**
- `PyEquityResult`:
  - `hero_equity: float` - Probabilidad de ganar (0.0 - 1.0)
  - `villain_equity: float` - Probabilidad de perder
  - `tie_equity: float` - Probabilidad de empate
  - `simulations_run: int` - Simulaciones ejecutadas
  - `converged_early: bool` - Si convergió temprano
  - `standard_error: float` - Error estándar

**Ejemplo Python:**
```python
from app.bridge import calculate_equity

# AA vs KK preflop
result = calculate_equity("AsAh", "KsKh", "", 50000)
print(f"AA tiene {result['hero_percent']:.1f}% de equity")  # ~82%

# Con board
result = calculate_equity("AhKh", "QsQd", "Kd7c2h", 50000)
print(f"Top pair vs overpair: {result['hero_percent']:.1f}%")
```

#### `calculate_equity_multiway(hands, board, iterations) -> PyResult<Vec<f64>>`

Calcula equity para 3+ jugadores.

**Parámetros:**
- `hands: List[str]` - Lista de manos
- `board: str` - Cartas comunitarias
- `iterations: int` - Simulaciones

**Retorna:**
- `List[float]` - Equities para cada jugador (suma = 1.0)

---

### 2.3 Funciones de Utilidad

| Función | Firma | Descripción |
|---------|-------|-------------|
| `is_simd_available()` | `-> bool` | Verifica si AVX2 está disponible |
| `version()` | `-> str` | Retorna versión del módulo |
| `system_info()` | `-> str` | Información del sistema |

---

## 3. Definición del Esquema Apache Arrow

Los datos parseados por Rust se entregan a Python/DuckDB siguiendo este esquema estricto de `HandHistoryBatch`:

| Campo | Tipo Arrow | Nullable | Descripción |
|:------|:-----------|:---------|:------------|
| `hand_id` | `Utf8` | No | ID único de Winamax |
| `timestamp` | `Timestamp(Micro, UTC)` | No | Momento de la mano |
| `player_id` | `Utf8` | No | Nickname del jugador |
| `street` | `Int8` | No | 0:Pre, 1:Flop, 2:Turn, 3:River |
| `action_type` | `Int8` | No | Enum: 0:Fold, 1:Call, 2:Raise, 3:Bet, 4:Check |
| `amount_cents` | `Int64` | No | Cantidad en centavos enteros |
| `is_hero` | `Boolean` | No | `true` si el jugador es `thesmoy` |
| `cards` | `FixedSizeList(Utf8, 2)` | Sí | Cartas del jugador (si hay showdown) |

**Batch Size Optimizada:** 10,000 manos por bloque para maximizar el uso de la caché L3 del Ryzen.

---

## 4. Gestión de Errores

### 4.1 Códigos de Error (Rust -> Python)

| Código | Constante | Tipo Python | Descripción |
|:-------|:----------|:------------|:------------|
| `101` | `ERR_IO_ERROR` | `IOError` | No se puede acceder al archivo |
| `102` | `ERR_PARSER_ERROR` | `RuntimeError` | Formato irreconocible o corrupto |
| `201` | `ERR_INVALID_RANGE` | `ValueError` | Error de sintaxis en cartas/rango |
| `202` | `ERR_SIM_TIMEOUT` | `RuntimeError` | Simulación excedió 500ms |

### 4.2 Manejo de Errores en Python

```python
from app.bridge import parse_files, calculate_equity

# Manejo de errores de parsing
try:
    result = parse_files(["archivo.txt"])
except IOError as e:
    print(f"Error de acceso: {e}")  # [101] No se puede acceder...
except RuntimeError as e:
    print(f"Error de parsing: {e}")  # [102] Formato irreconocible...

# Manejo de errores de equity
try:
    result = calculate_equity("XxYy", "QsQh")
except ValueError as e:
    print(f"Cartas inválidas: {e}")  # [201] Carta inválida...
except RuntimeError as e:
    print(f"Timeout: {e}")  # [202] Simulación excedió...
```

### 4.3 Política de Timeouts y Reintentos

- **Simulaciones:** Si una consulta de equidad tarda más de **500ms**, Rust aborta y devuelve `SIM_TIMEOUT`
- **Ingestión:** No hay timeout, pero se reporta progreso
- **Reintentos:** Python reintenta en caso de `IO_ERROR` (archivo bloqueado) con máximo 3 intentos y backoff de 100ms

---

## 5. Sintaxis de Rangos (HandRangeDSL)

El motor de equidad interpreta rangos en formato textual.

- **Referencia Completa:** Ver `docs/specs/range-spec.md`
- **Carga de Datos:** El backend carga situaciones de `docs/ranges/preflop-ranges.md` al iniciar

---

## 6. Clases Python Expuestas

### PyParseResult
```python
class PyParseResult:
    total_hands: int
    successful_files: int
    failed_files: int
    elapsed_ms: int
```

### PyEquityResult
```python
class PyEquityResult:
    hero_equity: float      # 0.0 - 1.0
    villain_equity: float   # 0.0 - 1.0
    tie_equity: float       # 0.0 - 1.0
    simulations_run: int
    converged_early: bool
    standard_error: float
    
    def hero_percent(self) -> float: ...    # 0 - 100
    def villain_percent(self) -> float: ... # 0 - 100
```

### PyHandSummary
```python
class PyHandSummary:
    hand_id: str
    timestamp: str
    table_name: str
    player_count: int
    hero_played: bool
    total_pot_cents: int
```

### PyDbStats
```python
class PyDbStats:
    player_count: int
    hand_count: int
    action_count: int
    session_count: int
    tournament_count: int
```

---

## 7. Integración con FastAPI

### 7.1 Uso en Endpoints

```python
from fastapi import APIRouter, HTTPException
from app.bridge import parse_files, calculate_equity, is_ffi_available

router = APIRouter()

@router.get("/ffi/status")
async def ffi_status():
    return {"available": is_ffi_available()}

@router.post("/parse")
async def parse_histories(files: list[str]):
    if not is_ffi_available():
        raise HTTPException(503, "FFI module not available")
    return parse_files(files)

@router.post("/equity")
async def compute_equity(hero: str, villain: str, board: str = "", iterations: int = 100000):
    if not is_ffi_available():
        raise HTTPException(503, "FFI module not available")
    return calculate_equity(hero, villain, board, iterations)
```

### 7.2 Type Stubs

Los type stubs para IDEs están en `server-api/app/bridge/poker_ffi.pyi`.

---

## 8. Performance Esperado

| Operación | Target | Hardware |
|-----------|--------|----------|
| Parsing 1000 manos | < 500ms | Rayon 16 threads |
| Equity (100K sims) | < 100ms | SIMD AVX2 |
| Equity multiway 3p | < 150ms | Parallel Monte Carlo |
| Overhead FFI | < 1ms | PyO3 zero-copy |

---

## 9. Compilación y Testing

### Compilar el módulo

```bash
# Desarrollo (debug, hot reload)
cd backend/ffi
maturin develop

# Producción (optimizado)
maturin build --release
pip install target/wheels/poker_ffi-*.whl
```

### Ejecutar tests

```bash
# Tests de Rust
cd backend && cargo test -p poker-ffi

# Tests de Python (con mock)
cd server-api && pytest tests/test_ffi_integration.py -v

# Tests de integración real (requiere FFI compilado)
pytest tests/test_ffi_integration.py -v -k "TestRealFFI"
```
