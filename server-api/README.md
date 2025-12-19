# Poker AI API - FastAPI Server

Backend REST API server that orchestrates the poker analysis platform. Provides the interface between the frontend and Rust core components via PyO3/FFI.

## Requirements

- Python 3.11+
- Poetry 1.8+
- Rust 1.70+ (for PyO3 compilation)

## Setup

### 1. Install dependencies

```powershell
poetry install
```

### 2. Configure environment

Copy `.env.example` to `.env` and customize if needed:

```powershell
Copy-Item .env.example .env
# Edit .env with your settings
```

Default environment variables:
- `APP_NAME`: Application name (default: "Poker AI API")
- `DEBUG`: Debug mode (default: false)
- `API_PORT`: Server port (default: 8000)
- `WINAMAX_HISTORY_PATH`: Path to Winamax hand histories
- `DUCKDB_MEMORY_LIMIT_GB`: DuckDB memory limit (default: 48GB)

### 3. Build Rust extensions (optional for development)

```powershell
.\build.ps1 -Dev
```

Or for release build:

```powershell
.\build.ps1 -Release
```

## Running the Server

### Development mode with hot reload

```powershell
.\run.ps1
```

Or manually:

```powershell
poetry run uvicorn app.main:app --host 127.0.0.1 --port 8000 --reload
```

### Production mode

```powershell
poetry run uvicorn app.main:app --host 127.0.0.1 --port 8000 --workers 4
```

## API Documentation

Once the server is running:
- Swagger UI: http://127.0.0.1:8000/docs
- ReDoc: http://127.0.0.1:8000/redoc
- OpenAPI schema: http://127.0.0.1:8000/openapi.json

## Project Structure

```
server-api/
├── app/
│   ├── __init__.py
│   ├── main.py              # FastAPI application entry point
│   ├── config/
│   │   ├── __init__.py
│   │   └── settings.py      # Configuration management
│   ├── bridge/              # PyO3 FFI bridge (Rust integration)
│   │   └── __init__.py
│   └── routes/              # API endpoint routes
│       └── __init__.py
├── pyproject.toml           # Poetry dependencies
├── build.ps1                # Maturin build script
├── run.ps1                  # Development server starter
└── README.md                # This file
```

## Development

### Code formatting

```powershell
poetry run black app/
```

### Type checking

```powershell
poetry run mypy app/
```

### Linting

```powershell
poetry run ruff check app/
```

### Testing

```powershell
poetry run pytest
```

## Integration with Rust Backend

The `app/bridge/` module will contain compiled Rust extensions via PyO3. This allows calling high-performance Rust functions directly from Python.

### FFI Contract

See `docs/specs/ffi-contract.md` for the interface specification between Python and Rust.

## Performance Notes

- DuckDB runs in-process for minimal latency
- Parquet files are used for persistent storage
- The API is designed for a single local user (127.0.0.1 only)
- Data never leaves the local machine
