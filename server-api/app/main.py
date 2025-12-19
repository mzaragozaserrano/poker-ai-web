"""
FastAPI server for Winamax Poker Analyzer.

Entry point for the REST API orchestrator that bridges Python and Rust components.
"""

from contextlib import asynccontextmanager
from typing import Any

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.config.settings import Settings

# Initialize settings from environment
settings = Settings()


@asynccontextmanager
async def lifespan(app: FastAPI) -> Any:
    """
    Manage application lifecycle (startup and shutdown).
    """
    # Startup
    print(f"Starting Poker AI API with config: {settings.app_name}")
    yield
    # Shutdown
    print("Shutting down Poker AI API")


# Create FastAPI application
app = FastAPI(
    title=settings.app_name,
    description="High-performance poker analysis platform for Winamax",
    version="0.1.0",
    lifespan=lifespan,
)

# Configure CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000", "http://127.0.0.1:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health_check() -> dict[str, str]:
    """
    Health check endpoint.
    """
    return {
        "status": "healthy",
        "service": settings.app_name,
    }


@app.get("/api/v1/config")
async def get_config() -> dict[str, Any]:
    """
    Get current configuration.
    """
    return {
        "app_name": settings.app_name,
        "debug": settings.debug,
        "winamax_history_path": str(settings.winamax_history_path),
        "duckdb_memory_limit_gb": settings.duckdb_memory_limit_gb,
    }


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        "app.main:app",
        host="127.0.0.1",
        port=settings.api_port,
        reload=settings.debug,
    )
