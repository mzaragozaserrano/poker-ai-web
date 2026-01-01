"""
FastAPI server for Winamax Poker Analyzer.

Entry point for the REST API orchestrator that bridges Python and Rust components.
"""

from contextlib import asynccontextmanager
from typing import Any

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.config.settings import Settings
from app.routes import stats, hands, equity
from app.bridge import is_ffi_available, get_system_info

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
    yield
    # Shutdown
    print("Shutting down Poker AI API")


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

    uvicorn.run(
        "app.main:app",
        host="127.0.0.1",
        port=settings.api_port,
        reload=settings.debug,
    )


