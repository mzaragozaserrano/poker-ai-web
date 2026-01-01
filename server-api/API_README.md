# API REST - Poker AI Web

API REST de alto rendimiento para análisis de póker, construida con FastAPI y puente FFI a Rust.

## Endpoints Disponibles

### Health & Config

- `GET /health` - Health check del servicio
- `GET /api/v1/config` - Configuración actual

### Estadísticas de Jugadores

- `GET /api/v1/stats/player/{player_name}` - Estadísticas globales y por posición
  - Query params: `start_date`, `end_date`, `stake`, `game_type`, `min_hands`

### Consulta de Manos

- `GET /api/v1/hands/recent` - Últimas manos jugadas
  - Query params: `limit`, `offset`, `start_date`, `end_date`, `stake`, `hero_only`
- `GET /api/v1/hands/{hand_id}` - Detalle completo de una mano

### Cálculos de Equity

- `POST /api/v1/equity/calculate` - Equity entre 2 jugadores
  - Body: `{"hero_cards": "AhKd", "villain_cards": "QsQh", "board": "", "iterations": 100000}`
- `POST /api/v1/equity/calculate/multiway` - Equity multiway (3+ jugadores)
  - Body: `{"hands": ["AhKd", "QsQh", "8c8d"], "board": "", "iterations": 50000}`

## Documentación Interactiva

Una vez iniciado el servidor, la documentación está disponible en:

- **Swagger UI**: http://127.0.0.1:8000/docs
- **ReDoc**: http://127.0.0.1:8000/redoc
- **OpenAPI JSON**: http://127.0.0.1:8000/api/v1/openapi.json

## Instalación

### Requisitos

- Python 3.11+
- Rust toolchain (para compilar el módulo FFI)
- Poetry (opcional, para gestión de dependencias)

### Pasos

1. **Compilar el módulo FFI** (primera vez):

```bash
cd backend/ffi
cargo build --release
maturin develop --release
```

2. **Instalar dependencias Python**:

```bash
cd server-api
pip install -e .
```

O con Poetry:

```bash
cd server-api
poetry install
```

## Ejecución

### Modo Desarrollo

```bash
cd server-api
python -m app.main
```

O con el script de PowerShell:

```powershell
.\run.ps1
```

El servidor iniciará en: http://127.0.0.1:8000

### Modo Producción

```bash
cd server-api
uvicorn app.main:app --host 127.0.0.1 --port 8000 --workers 4
```

## Tests

```bash
cd server-api
pytest tests/test_api_endpoints.py -v
```

### Coverage

```bash
pytest tests/test_api_endpoints.py --cov=app --cov-report=html
```

## Ejemplos de Uso

### cURL

```bash
# Health check
curl http://127.0.0.1:8000/health

# Estadísticas de thesmoy
curl "http://127.0.0.1:8000/api/v1/stats/player/thesmoy?stake=NL10&min_hands=50"

# Manos recientes
curl "http://127.0.0.1:8000/api/v1/hands/recent?limit=10&hero_only=true"

# Calcular equity AA vs KK preflop
curl -X POST http://127.0.0.1:8000/api/v1/equity/calculate \
  -H "Content-Type: application/json" \
  -d '{"hero_cards":"AsAh","villain_cards":"KsKh","board":"","iterations":50000}'
```

### Python

```python
import requests

# Calcular equity
response = requests.post(
    "http://127.0.0.1:8000/api/v1/equity/calculate",
    json={
        "hero_cards": "AhKd",
        "villain_cards": "QsQh",
        "board": "Qh7s2c",
        "iterations": 100000
    }
)

result = response.json()
print(f"Hero equity: {result['hero_percent']:.1f}%")
print(f"Villain equity: {result['villain_percent']:.1f}%")
print(f"Elapsed: {result['elapsed_ms']}ms")
```

### JavaScript (Fetch)

```javascript
// Obtener estadísticas
const response = await fetch(
  'http://127.0.0.1:8000/api/v1/stats/player/thesmoy'
);
const stats = await response.json();
console.log(`VPIP: ${stats.summary.vpip}%`);
console.log(`PFR: ${stats.summary.pfr}%`);
```

## Seguridad

- **CORS configurado SOLO para localhost**: El servidor rechaza requests de orígenes externos
- **Sin autenticación**: La API es para uso local, no exponer a internet
- **Binding a 127.0.0.1**: El servidor escucha exclusivamente en localhost

## Performance Esperado

| Operación | Target | Hardware |
|-----------|--------|----------|
| GET /stats/player | < 50ms | DuckDB in-memory |
| GET /hands/recent | < 100ms | DuckDB consulta columnar |
| POST /equity/calculate (100K sims) | < 100ms | Rust SIMD AVX2 + Rayon |
| POST /equity/multiway (3 players) | < 150ms | Monte Carlo paralelo |

## Estructura del Código

```
server-api/
├── app/
│   ├── main.py              # Entry point, configuración FastAPI
│   ├── models/              # Modelos Pydantic
│   │   ├── stats.py         # Modelos de estadísticas
│   │   ├── hands.py         # Modelos de manos
│   │   └── equity.py        # Modelos de equity
│   ├── routes/              # Routers
│   │   ├── stats.py         # Endpoints de estadísticas
│   │   ├── hands.py         # Endpoints de manos
│   │   └── equity.py        # Endpoints de equity
│   ├── bridge/              # FFI Rust-Python
│   │   ├── __init__.py      # Wrapper Python
│   │   └── poker_ffi.pyi    # Type stubs
│   └── config/
│       └── settings.py      # Configuración del servidor
└── tests/
    ├── test_api_endpoints.py  # Tests de integración
    ├── test_ffi_integration.py
    └── test_health.py
```

## Solución de Problemas

### Error: "FFI module not available"

El módulo Rust no está compilado. Ejecutar:

```bash
cd backend/ffi
maturin develop --release
```

### Error: "Module 'poker_ffi' not found"

El módulo no está en el PYTHONPATH. Asegurarse de:

1. Compilar con `maturin develop` (no solo `cargo build`)
2. Ejecutar desde el entorno virtual correcto

### Tests fallan con "command not found: pytest"

Instalar pytest:

```bash
pip install pytest pytest-cov
```

## Próximos Pasos

- [ ] Integrar DuckDB para consultas reales (actualmente usa mock data)
- [ ] Implementar WebSockets para notificaciones en tiempo real
- [ ] Agregar caché Redis para consultas frecuentes
- [ ] Implementar autenticación JWT para deployment remoto (futuro)

## Contribución

Ver el archivo principal README.md en la raíz del proyecto.

## Licencia

Ver LICENSE en la raíz del proyecto.

