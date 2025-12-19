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

    # Winamax configuration
    winamax_history_path: Path = Path(
        r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history"
    )

    # DuckDB configuration
    duckdb_memory_limit_gb: int = 48

    class Config:
        """Pydantic config."""

        env_file = ".env"
        env_file_encoding = "utf-8"
        case_sensitive = False
