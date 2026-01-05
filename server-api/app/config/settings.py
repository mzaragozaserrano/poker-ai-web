"""
Configuration settings for the Poker API.

Settings are loaded from environment variables with sensible defaults.
"""

from pathlib import Path

from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """
    Application settings loaded from environment variables.
    """

    # Application settings
    app_name: str = "Poker AI API"
    debug: bool = False
    api_port: int = 8000
    api_host: str = "127.0.0.1"  # SECURITY: Only localhost, never 0.0.0.0

    # Winamax configuration
    winamax_history_path: Path = Path(
        r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history"
    )

    # DuckDB configuration
    duckdb_memory_limit_gb: int = 48

    # Logging configuration
    log_level: str = "INFO"
    log_dir: Path = Path("logs")
    log_max_bytes: int = 100 * 1024 * 1024  # 100MB
    log_backup_count: int = 5
    log_format: str = "json"  # json or console

    class Config:
        """Pydantic config."""

        env_file = ".env"
        env_file_encoding = "utf-8"
        case_sensitive = False


