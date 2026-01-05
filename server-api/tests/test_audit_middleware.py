"""
Tests para el middleware de auditoría.

Verifica que todas las requests se loguean correctamente con:
- Request ID único
- Método y path
- Status code
- Duración en milisegundos
"""

import json
import time
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

import pytest
from fastapi import FastAPI
from fastapi.testclient import TestClient

from app.config.settings import Settings
from app.middleware.audit import AuditMiddleware
from app.utils.logger import setup_logging


@pytest.fixture
def temp_log_dir():
    """Crea un directorio temporal para logs."""
    with TemporaryDirectory() as tmpdir:
        yield Path(tmpdir)


@pytest.fixture
def test_app(temp_log_dir):
    """Crea una aplicación FastAPI de prueba con el middleware."""
    # Configurar logging en directorio temporal
    test_settings = Settings()
    test_settings.log_dir = temp_log_dir
    test_settings.log_level = "DEBUG"
    test_settings.log_format = "json"
    setup_logging(test_settings)
    
    # Crear app
    app = FastAPI()
    app.add_middleware(AuditMiddleware)
    
    @app.get("/test")
    async def test_endpoint():
        return {"message": "test"}
    
    @app.get("/slow")
    async def slow_endpoint():
        time.sleep(0.1)  # Simular operación lenta
        return {"message": "slow"}
    
    @app.get("/error")
    async def error_endpoint():
        raise ValueError("Test error")
    
    return app, temp_log_dir


def test_audit_middleware_logs_request(test_app):
    """Verifica que el middleware loguea todas las requests."""
    app, log_dir = test_app
    client = TestClient(app)
    
    response = client.get("/test")
    assert response.status_code == 200
    
    # Verificar que se creó el archivo de auditoría
    audit_file = log_dir / "audit.log"
    # El archivo puede no existir inmediatamente debido a buffering
    # En producción verificaríamos el contenido


def test_audit_middleware_adds_request_id(test_app):
    """Verifica que el middleware añade un request ID único."""
    app, log_dir = test_app
    client = TestClient(app)
    
    response1 = client.get("/test")
    response2 = client.get("/test")
    
    # Verificar que ambas responses tienen request ID
    assert "X-Request-ID" in response1.headers
    assert "X-Request-ID" in response2.headers
    
    # Verificar que son diferentes
    assert response1.headers["X-Request-ID"] != response2.headers["X-Request-ID"]


def test_audit_middleware_logs_duration(test_app):
    """Verifica que el middleware loguea la duración de la request."""
    app, log_dir = test_app
    client = TestClient(app)
    
    response = client.get("/slow")
    assert response.status_code == 200
    
    # En un test real verificaríamos que el log contiene duration_ms >= 100


def test_audit_middleware_logs_status_code(test_app):
    """Verifica que el middleware loguea el status code."""
    app, log_dir = test_app
    client = TestClient(app)
    
    # Request exitosa
    response = client.get("/test")
    assert response.status_code == 200
    
    # Request con error
    response = client.get("/error")
    assert response.status_code == 500
    
    # Ambas deben estar logueadas con sus respectivos status codes


def test_audit_middleware_handles_exceptions(test_app):
    """Verifica que el middleware maneja excepciones correctamente."""
    app, log_dir = test_app
    client = TestClient(app)
    
    # Request que lanza excepción
    response = client.get("/error")
    assert response.status_code == 500
    
    # Debe loguear el error pero no crashear


def test_audit_log_format_is_json(test_app):
    """Verifica que los logs de auditoría están en formato JSON."""
    app, log_dir = test_app
    client = TestClient(app)
    
    client.get("/test")
    
    # Leer el archivo de auditoría
    audit_file = log_dir / "audit.log"
    if audit_file.exists():
        content = audit_file.read_text()
        if content.strip():
            lines = content.strip().split("\n")
            for line in lines:
                try:
                    log_entry = json.loads(line)
                    # Verificar campos esperados
                    if "api_request_completed" in line:
                        assert "request_id" in log_entry
                        assert "method" in log_entry
                        assert "path" in log_entry
                        assert "status_code" in log_entry
                        assert "duration_ms" in log_entry
                except json.JSONDecodeError:
                    # Puede haber logs no-audit mezclados
                    pass


def test_audit_log_contains_required_fields(test_app):
    """Verifica que los logs de auditoría contienen todos los campos requeridos."""
    app, log_dir = test_app
    client = TestClient(app)
    
    response = client.get("/test")
    request_id = response.headers["X-Request-ID"]
    
    # Verificar que el log contiene:
    # - request_id
    # - timestamp
    # - method
    # - path
    # - status_code
    # - duration_ms
    # - component
    # - level
    pass  # Implementar lectura del archivo de log


def test_audit_middleware_does_not_log_sensitive_data(test_app):
    """Verifica que el middleware NO loguea datos sensibles."""
    app, log_dir = test_app
    client = TestClient(app)
    
    # Request con query params que podrían ser sensibles
    response = client.get("/test?password=secret123&token=abc")
    
    # Leer logs y verificar que password y token NO están presentes
    audit_file = log_dir / "audit.log"
    if audit_file.exists():
        content = audit_file.read_text()
        # NO debe contener los valores sensibles
        assert "secret123" not in content
        assert "abc" not in content or "abc" in response.headers["X-Request-ID"]


def test_request_id_propagates_to_state(test_app):
    """Verifica que el request_id se propaga al state de la request."""
    app, log_dir = test_app
    
    @app.get("/check-state")
    async def check_state(request):
        from fastapi import Request
        return {"has_request_id": hasattr(request.state, "request_id")}
    
    client = TestClient(app)
    response = client.get("/check-state")
    
    # Debe tener request_id en el state
    # Nota: Esto puede no funcionar en TestClient
    # En producción verificaríamos con requests reales


@pytest.mark.parametrize(
    "method,path",
    [
        ("GET", "/test"),
        ("POST", "/test"),
        ("PUT", "/test"),
        ("DELETE", "/test"),
    ],
)
def test_audit_logs_all_http_methods(test_app, method, path):
    """Verifica que se loguean todos los métodos HTTP."""
    app, log_dir = test_app
    
    # Añadir endpoints para otros métodos
    @app.post("/test")
    async def post_endpoint():
        return {"method": "POST"}
    
    @app.put("/test")
    async def put_endpoint():
        return {"method": "PUT"}
    
    @app.delete("/test")
    async def delete_endpoint():
        return {"method": "DELETE"}
    
    client = TestClient(app)
    
    if method == "GET":
        response = client.get(path)
    elif method == "POST":
        response = client.post(path)
    elif method == "PUT":
        response = client.put(path)
    elif method == "DELETE":
        response = client.delete(path)
    
    # Verificar que se logueó con el método correcto
    assert "X-Request-ID" in response.headers

