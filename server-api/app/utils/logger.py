"""
Structured logging setup using structlog.

This module provides:
- JSON-formatted structured logs for production
- Console-formatted logs for development
- Automatic log rotation (100MB max per file)
- Context injection (request_id, timestamp, duration_ms)
- Audit trail for security-critical operations
- No PII or hand content in logs
"""

import logging
import sys
from logging.handlers import RotatingFileHandler
from pathlib import Path
from typing import Any

import structlog
from structlog.typing import EventDict, WrappedLogger

from app.config.settings import Settings


def add_timestamp(
    logger: WrappedLogger, method_name: str, event_dict: EventDict
) -> EventDict:
    """Add ISO 8601 timestamp to log entries."""
    from datetime import datetime, timezone

    event_dict["timestamp"] = datetime.now(timezone.utc).isoformat()
    return event_dict


def add_log_level(
    logger: WrappedLogger, method_name: str, event_dict: EventDict
) -> EventDict:
    """Normalize log level naming."""
    if method_name == "warn":
        method_name = "warning"
    event_dict["level"] = method_name.upper()
    return event_dict


def add_component(
    logger: WrappedLogger, method_name: str, event_dict: EventDict
) -> EventDict:
    """
    Add component identifier from logger name.
    
    Examples:
        app.routes.equity -> component: api
        app.services.file_watcher_service -> component: parser
        app.bridge.poker_ffi -> component: ffi
    """
    logger_name = event_dict.get("logger", "unknown")
    
    if "routes" in logger_name or "main" in logger_name:
        component = "api"
    elif "services" in logger_name:
        component = "parser"
    elif "bridge" in logger_name:
        component = "ffi"
    elif "db" in logger_name or "duckdb" in logger_name:
        component = "db"
    else:
        component = "api"
    
    event_dict["component"] = component
    return event_dict


def sanitize_event_dict(
    logger: WrappedLogger, method_name: str, event_dict: EventDict
) -> EventDict:
    """
    Remove sensitive data from logs.
    
    SECURITY: Never log:
    - Hand content (cards, actions in detail)
    - Full file paths (only filenames)
    - Player identifiable information beyond usernames
    """
    # Remove potentially sensitive keys
    sensitive_keys = ["password", "token", "secret", "hand_content", "cards_detail"]
    for key in sensitive_keys:
        if key in event_dict:
            event_dict[key] = "[REDACTED]"
    
    # Sanitize file paths (keep only filename)
    if "file_path" in event_dict:
        path = Path(str(event_dict["file_path"]))
        event_dict["file_path"] = path.name
    
    return event_dict


def setup_logging(settings: Settings) -> None:
    """
    Configure structured logging with rotation.
    
    Args:
        settings: Application settings with log configuration
        
    Creates two log files:
        - logs/api.log: All application logs
        - logs/audit.log: Security and audit trail only
    """
    # Create logs directory
    settings.log_dir.mkdir(parents=True, exist_ok=True)
    
    # Clear existing handlers
    logging.root.handlers.clear()
    
    # Configure standard library logging
    logging.basicConfig(
        format="%(message)s",
        level=getattr(logging, settings.log_level.upper()),
        stream=sys.stdout,
    )
    
    # Setup rotating file handler for main logs
    main_log_file = settings.log_dir / "api.log"
    main_handler = RotatingFileHandler(
        filename=main_log_file,
        maxBytes=settings.log_max_bytes,
        backupCount=settings.log_backup_count,
        encoding="utf-8",
    )
    main_handler.setLevel(getattr(logging, settings.log_level.upper()))
    
    # Setup rotating file handler for audit logs
    audit_log_file = settings.log_dir / "audit.log"
    audit_handler = RotatingFileHandler(
        filename=audit_log_file,
        maxBytes=settings.log_max_bytes,
        backupCount=settings.log_backup_count,
        encoding="utf-8",
    )
    audit_handler.setLevel(logging.INFO)
    
    # Configure structlog processors
    shared_processors = [
        structlog.stdlib.add_logger_name,
        structlog.stdlib.add_log_level,
        add_timestamp,
        add_log_level,
        add_component,
        sanitize_event_dict,
        structlog.stdlib.PositionalArgumentsFormatter(),
        structlog.processors.StackInfoRenderer(),
        structlog.processors.format_exc_info,
    ]
    
    if settings.log_format == "json":
        # JSON output for production
        processors = shared_processors + [
            structlog.processors.JSONRenderer()
        ]
    else:
        # Console output for development
        processors = shared_processors + [
            structlog.dev.ConsoleRenderer(colors=True)
        ]
    
    structlog.configure(
        processors=processors,  # type: ignore[arg-type]
        wrapper_class=structlog.stdlib.BoundLogger,
        context_class=dict,
        logger_factory=structlog.stdlib.LoggerFactory(),
        cache_logger_on_first_use=True,
    )
    
    # Add handlers to root logger
    root_logger = logging.getLogger()
    root_logger.addHandler(main_handler)
    
    # Add audit handler to audit logger
    audit_logger = logging.getLogger("audit")
    audit_logger.addHandler(audit_handler)
    audit_logger.setLevel(logging.INFO)
    
    # Log startup message
    logger = get_logger("app.utils.logger")
    logger.info(
        "logging_initialized",
        log_level=settings.log_level,
        log_dir=str(settings.log_dir),
        max_bytes=settings.log_max_bytes,
        backup_count=settings.log_backup_count,
        format=settings.log_format,
    )


def get_logger(name: str) -> structlog.stdlib.BoundLogger:
    """
    Get a structured logger instance.
    
    Args:
        name: Logger name (usually __name__)
        
    Returns:
        Configured structlog logger
        
    Example:
        >>> logger = get_logger(__name__)
        >>> logger.info("user_action", action="login", user_id=123)
    """
    return structlog.get_logger(name)


def audit_log(event: str, **context: Any) -> None:
    """
    Log an audit trail event.
    
    Used for security-critical operations:
    - API endpoint access
    - Database queries
    - File system operations
    - Authentication attempts
    
    Args:
        event: Event name (e.g., "api_request", "db_query")
        **context: Additional context fields
        
    Example:
        >>> audit_log("api_request", 
        ...           method="POST", 
        ...           path="/api/v1/equity/calculate",
        ...           duration_ms=45.2)
    """
    logger = structlog.get_logger("audit")
    logger.info(event, **context)

