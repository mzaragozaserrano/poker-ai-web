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
from app.middleware import LocalhostOnlyMiddleware, validate_server_host, AuditMiddleware
from app.utils.logger import setup_logging, get_logger

# Initialize settings from environment
settings = Settings()

# Initialize structured logging
setup_logging(settings)
logger = get_logger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI) -> Any:
    """
    Manage application lifecycle (startup and shutdown).
    """
    # Startup
    logger.info(
        "application_startup",
        app_name=settings.app_name,
        ffi_available=is_ffi_available(),
        host=settings.api_host,
        port=settings.api_port,
    )
    
    if is_ffi_available():
        logger.debug("system_info", info=get_system_info())
    
    # Iniciar file watcher si FFI está disponible
    file_watcher = None
    if is_ffi_available() and settings.winamax_history_path.exists():
        try:
            logger.info(
                "file_watcher_starting",
                history_path=str(settings.winamax_history_path),
            )
            file_watcher = get_file_watcher_service(str(settings.winamax_history_path))
            file_watcher.start()
            logger.info("file_watcher_started")
        except Exception as e:
            logger.warning(
                "file_watcher_start_failed",
                error=str(e),
                exc_info=True,
            )
    else:
        if not is_ffi_available():
            logger.warning("file_watcher_disabled", reason="ffi_not_available")
        if not settings.winamax_history_path.exists():
            logger.warning(
                "file_watcher_disabled",
                reason="history_path_not_found",
                path=str(settings.winamax_history_path),
            )
    
    yield
    
    # Shutdown
    logger.info("application_shutdown")
    if file_watcher and file_watcher.is_running:
        file_watcher.stop()
        logger.info("file_watcher_stopped")


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

# Audit middleware for request logging
app.add_middleware(AuditMiddleware)

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


