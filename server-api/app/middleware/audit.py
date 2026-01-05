"""
Audit middleware for logging API requests.

Logs all incoming requests with timing, status codes, and context.
"""

import time
import uuid
from typing import Callable

from fastapi import Request, Response
from starlette.middleware.base import BaseHTTPMiddleware

from app.utils.logger import audit_log, get_logger

logger = get_logger(__name__)


class AuditMiddleware(BaseHTTPMiddleware):
    """
    Middleware for auditing API requests.
    
    Logs:
    - Request method and path
    - Response status code
    - Request duration in milliseconds
    - Unique request ID for tracing
    
    Does NOT log:
    - Request/response bodies (may contain sensitive data)
    - Query parameters (may contain user info)
    """

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        """Process request and log audit trail."""
        # Generate unique request ID
        request_id = str(uuid.uuid4())
        
        # Add request ID to request state for propagation
        request.state.request_id = request_id
        
        # Record start time
        start_time = time.perf_counter()
        
        # Log incoming request
        logger.debug(
            "api_request_started",
            request_id=request_id,
            method=request.method,
            path=request.url.path,
            client_host=request.client.host if request.client else "unknown",
        )
        
        # Process request
        try:
            response = await call_next(request)
            
            # Calculate duration
            duration_ms = (time.perf_counter() - start_time) * 1000
            
            # Log completed request (audit trail)
            audit_log(
                "api_request_completed",
                request_id=request_id,
                method=request.method,
                path=request.url.path,
                status_code=response.status_code,
                duration_ms=round(duration_ms, 2),
            )
            
            # Add request ID to response headers for client tracing
            response.headers["X-Request-ID"] = request_id
            
            return response
            
        except Exception as e:
            # Calculate duration even on error
            duration_ms = (time.perf_counter() - start_time) * 1000
            
            # Log failed request
            logger.error(
                "api_request_failed",
                request_id=request_id,
                method=request.method,
                path=request.url.path,
                duration_ms=round(duration_ms, 2),
                error=str(e),
                exc_info=True,
            )
            
            raise

