"""
FastAPI server for Winamax Poker Analyzer.

Entry point for the REST API orchestrator that bridges Python and Rust components.
"""

from contextlib import asynccontextmanager
from typing import Any

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.config.settings import Settings
from app.routes import stats, hands, equity, websocket
from app.bridge import is_ffi_available, get_system_info
from app.services.file_watcher_service import get_file_watcher_service
from app.middleware import LocalhostOnlyMiddleware, validate_server_host

# Initialize settings from environment
settings = Settings()


@asynccontextmanager
async def lifespan(app: FastAPI) -> Any:
    """
    Manage application lifecycle (startup and shutdown).
    """
    # Startup
    print(f"Starting Poker AI API with config: {settings.app_name}")
    print(f"FFI Module available: {is_ffi_available()}")
    if is_ffi_available():
        print(f"System info:\n{get_system_info()}")
    
    # Iniciar file watcher si FFI está disponible
    file_watcher = None
    if is_ffi_available() and settings.winamax_history_path.exists():
        try:
            print(f"Starting file watcher on: {settings.winamax_history_path}")
            file_watcher = get_file_watcher_service(str(settings.winamax_history_path))
            file_watcher.start()
            print("File watcher started successfully")
        except Exception as e:
            print(f"Warning: Could not start file watcher: {e}")
    else:
        if not is_ffi_available():
            print("Warning: FFI not available, file watcher disabled")
        if not settings.winamax_history_path.exists():
            print(f"Warning: History path does not exist: {settings.winamax_history_path}")
    
    yield
    
    # Shutdown
    print("Shutting down Poker AI API")
    if file_watcher and file_watcher.is_running:
        file_watcher.stop()
        print("File watcher stopped")


# Create FastAPI application
app = FastAPI(
    title=settings.app_name,
    description="High-performance poker analysis platform for Winamax",
    version="0.1.0",
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/api/v1/openapi.json",
)

# SECURITY: Localhost-only enforcement middleware (MUST be first)
app.add_middleware(LocalhostOnlyMiddleware)

# Configure CORS middleware - LOCALHOST ONLY para seguridad
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://localhost:5173",  # Vite dev server
        "http://127.0.0.1:5173",
    ],
    allow_credentials=True,
    allow_methods=["GET", "POST", "PUT", "DELETE"],
    allow_headers=["*"],
)

# Include routers
app.include_router(stats.router)
app.include_router(hands.router)
app.include_router(equity.router)
app.include_router(websocket.router)


@app.get("/health")
async def health_check() -> dict[str, str]:
    """
    Health check endpoint.
    
    Verifica el estado del servicio y la disponibilidad del módulo FFI.
    """
    return {
        "status": "healthy",
        "service": settings.app_name,
        "ffi_available": str(is_ffi_available()),
    }


@app.get("/api/v1/config")
async def get_config() -> dict[str, Any]:
    """
    Get current configuration.
    
    Retorna la configuración actual del servidor (solo para desarrollo/debug).
    """
    return {
        "app_name": settings.app_name,
        "debug": settings.debug,
        "winamax_history_path": str(settings.winamax_history_path),
        "duckdb_memory_limit_gb": settings.duckdb_memory_limit_gb,
        "ffi_available": is_ffi_available(),
    }


if __name__ == "__main__":
    import uvicorn

    # SECURITY: Validate host is localhost before starting
    validate_server_host(settings.api_host)
    
    uvicorn.run(
        "app.main:app",
        host=settings.api_host,
        port=settings.api_port,
        reload=settings.debug,
    )


