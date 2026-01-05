"""
Security middleware for localhost-only enforcement.

Ensures the API only accepts connections from localhost and blocks proxy headers.
"""

import logging
from typing import Callable

from fastapi import Request, Response
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.types import ASGIApp

logger = logging.getLogger(__name__)


class LocalhostOnlyMiddleware(BaseHTTPMiddleware):
    """
    Middleware that enforces localhost-only connections.
    
    Blocks:
    - Non-localhost client IPs
    - Proxy forwarding headers (X-Forwarded-For, X-Real-IP)
    - Any attempt to bypass localhost restriction
    """

    # Allowed hosts: localhost IPs and testclient (for pytest TestClient)
    LOCALHOST_IPS = {"127.0.0.1", "::1", "localhost", "testclient"}
    BLOCKED_HEADERS = {
        "x-forwarded-for",
        "x-real-ip",
        "x-forwarded-host",
        "x-forwarded-proto",
        "x-forwarded-port",
        "forwarded",
    }

    def __init__(self, app: ASGIApp) -> None:
        super().__init__(app)
        logger.info("LocalhostOnlyMiddleware initialized - API will only accept localhost connections")

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        """
        Process each request to ensure it's from localhost.
        
        Args:
            request: The incoming HTTP request
            call_next: The next middleware in the chain
            
        Returns:
            Response: Either the response from the next middleware or a 403 Forbidden
        """
        client_host = request.client.host if request.client else "unknown"
        
        # Check for proxy headers FIRST (more critical security issue)
        blocked_headers_found = [
            header for header in request.headers.keys() 
            if header.lower() in self.BLOCKED_HEADERS
        ]
        
        if blocked_headers_found:
            logger.warning(
                f"SECURITY: Blocked request with proxy headers: {blocked_headers_found} "
                f"from {client_host}"
            )
            return Response(
                content="Forbidden: Proxy headers are not allowed",
                status_code=403,
                media_type="text/plain",
            )
        
        # Check client IP (after proxy headers check)
        if not self._is_localhost(client_host):
            logger.warning(
                f"SECURITY: Blocked non-localhost connection attempt from {client_host} "
                f"to {request.url.path}"
            )
            return Response(
                content="Forbidden: This API only accepts connections from localhost",
                status_code=403,
                media_type="text/plain",
            )
        
        # Process the request
        response = await call_next(request)
        
        # Add security headers to response
        response.headers["X-Content-Type-Options"] = "nosniff"
        response.headers["X-Frame-Options"] = "DENY"
        response.headers["X-Localhost-Only"] = "true"
        
        return response

    def _is_localhost(self, host: str) -> bool:
        """
        Check if the host is localhost.
        
        Args:
            host: The host IP or hostname
            
        Returns:
            bool: True if localhost, False otherwise
        """
        return host in self.LOCALHOST_IPS


def validate_server_host(host: str) -> None:
    """
    Validate that the server is configured to bind only to localhost.
    
    Raises:
        ValueError: If the host is not localhost
    """
    if host not in {"127.0.0.1", "::1", "localhost"}:
        error_msg = (
            f"SECURITY ERROR: Attempted to bind to '{host}'. "
            f"This API must only bind to localhost (127.0.0.1). "
            f"Binding to 0.0.0.0 or other IPs would expose data to the network. "
            f"Change the configuration to use host='127.0.0.1'."
        )
        logger.error(error_msg)
        raise ValueError(error_msg)
    
    logger.info(f"Server host validation passed: {host}")

