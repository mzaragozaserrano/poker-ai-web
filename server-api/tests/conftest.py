"""
Pytest configuration and fixtures for Poker AI API tests.
"""

import pytest
from fastapi.testclient import TestClient

from app.main import app


@pytest.fixture
def client() -> TestClient:
    """
    Create a test client for FastAPI application.
    """
    return TestClient(app)


@pytest.fixture
def api_base_url() -> str:
    """
    Base URL for API endpoints.
    """
    return "/api/v1"

