"""
Health check endpoint tests.
"""

from fastapi.testclient import TestClient


def test_health_check(client: TestClient) -> None:
    """
    Test that health check endpoint returns healthy status.
    """
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "healthy"
    assert "service" in data


def test_config_endpoint(client: TestClient) -> None:
    """
    Test that config endpoint returns application configuration.
    """
    response = client.get("/api/v1/config")
    assert response.status_code == 200
    data = response.json()
    assert "app_name" in data
    assert "debug" in data
    assert "winamax_history_path" in data
    assert "duckdb_memory_limit_gb" in data

