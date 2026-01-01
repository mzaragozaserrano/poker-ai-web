"""
Tests de integración para los endpoints REST de la API.

Estos tests verifican que todos los endpoints respondan correctamente
y que la validación de datos funcione según lo esperado.
"""

import pytest
from fastapi.testclient import TestClient
from app.main import app

client = TestClient(app)


class TestHealthEndpoint:
    """Tests para el endpoint de health check."""
    
    def test_health_check(self):
        """Verifica que el endpoint /health responda correctamente."""
        response = client.get("/health")
        assert response.status_code == 200
        
        data = response.json()
        assert data["status"] == "healthy"
        assert "service" in data
        assert "ffi_available" in data


class TestConfigEndpoint:
    """Tests para el endpoint de configuración."""
    
    def test_get_config(self):
        """Verifica que el endpoint /api/v1/config responda correctamente."""
        response = client.get("/api/v1/config")
        assert response.status_code == 200
        
        data = response.json()
        assert "app_name" in data
        assert "debug" in data
        assert "ffi_available" in data


class TestStatsEndpoints:
    """Tests para endpoints de estadísticas."""
    
    def test_get_player_stats_valid(self):
        """Test de consulta de stats con nombre válido."""
        response = client.get("/api/v1/stats/player/thesmoy")
        assert response.status_code == 200
        
        data = response.json()
        assert data["player"] == "thesmoy"
        assert data["is_hero"] is True
        assert "summary" in data
        assert "positional" in data
        
        # Verificar estructura del summary
        summary = data["summary"]
        assert "vpip" in summary
        assert "pfr" in summary
        assert "hands" in summary
        assert summary["vpip"] >= 0 and summary["vpip"] <= 100
        assert summary["pfr"] >= 0 and summary["pfr"] <= 100
        
        # Verificar posiciones (6-max)
        positional = data["positional"]
        expected_positions = ["BTN", "SB", "BB", "MP", "CO"]
        for pos in expected_positions:
            assert pos in positional
            assert "vpip" in positional[pos]
            assert "pfr" in positional[pos]
            assert "hands" in positional[pos]
    
    def test_get_player_stats_with_filters(self):
        """Test de consulta con filtros de fecha y stake."""
        response = client.get(
            "/api/v1/stats/player/thesmoy",
            params={
                "stake": "NL10",
                "game_type": "NLHE",
                "min_hands": 50
            }
        )
        assert response.status_code == 200
    
    def test_get_player_stats_empty_name(self):
        """Test de error con nombre vacío."""
        response = client.get("/api/v1/stats/player/   ")
        assert response.status_code == 400
    
    def test_get_player_stats_invalid_game_type(self):
        """Test de error con game_type inválido."""
        response = client.get(
            "/api/v1/stats/player/thesmoy",
            params={"game_type": "INVALID"}
        )
        assert response.status_code == 400
    
    def test_get_player_stats_invalid_dates(self):
        """Test de error con fechas inválidas (end antes de start)."""
        response = client.get(
            "/api/v1/stats/player/thesmoy",
            params={
                "start_date": "2023-12-31T00:00:00Z",
                "end_date": "2023-12-01T00:00:00Z"
            }
        )
        assert response.status_code == 400


class TestHandsEndpoints:
    """Tests para endpoints de consulta de manos."""
    
    def test_get_recent_hands_default(self):
        """Test de consulta de manos recientes sin filtros."""
        response = client.get("/api/v1/hands/recent")
        assert response.status_code == 200
        
        data = response.json()
        assert isinstance(data, list)
        # Como es mock, debería retornar algunas manos
        assert len(data) > 0
        
        # Verificar estructura de una mano
        if len(data) > 0:
            hand = data[0]
            assert "hand_id" in hand
            assert "timestamp" in hand
            assert "table_name" in hand
            assert "player_count" in hand
            assert "hero_played" in hand
            assert "total_pot_cents" in hand
    
    def test_get_recent_hands_with_limit(self):
        """Test de consulta con límite."""
        response = client.get("/api/v1/hands/recent?limit=5")
        assert response.status_code == 200
        
        data = response.json()
        assert len(data) <= 5
    
    def test_get_recent_hands_hero_only(self):
        """Test de filtro hero_only."""
        response = client.get("/api/v1/hands/recent?hero_only=true")
        assert response.status_code == 200
        
        data = response.json()
        # Todas las manos deben tener hero_played = true
        for hand in data:
            assert hand["hero_played"] is True
    
    def test_get_recent_hands_invalid_limit(self):
        """Test de error con límite inválido (> 500)."""
        response = client.get("/api/v1/hands/recent?limit=1000")
        assert response.status_code == 422  # Validation error
    
    def test_get_hand_detail_valid(self):
        """Test de detalle de mano con ID válido."""
        hand_id = "20231215-12345678"
        response = client.get(f"/api/v1/hands/{hand_id}")
        assert response.status_code == 200
        
        data = response.json()
        assert data["hand_id"] == hand_id
        assert "players" in data
        assert "actions" in data
        assert "board" in data
        assert isinstance(data["players"], list)
        assert isinstance(data["actions"], list)
        
        # Verificar estructura de jugadores
        if len(data["players"]) > 0:
            player = data["players"][0]
            assert "name" in player
            assert "position" in player
            assert "stack_cents" in player
        
        # Verificar estructura de acciones
        if len(data["actions"]) > 0:
            action = data["actions"][0]
            assert "player" in action
            assert "street" in action
            assert "action_type" in action
            assert "amount_cents" in action
    
    def test_get_hand_detail_not_found(self):
        """Test de error con ID no encontrado."""
        response = client.get("/api/v1/hands/99999999-99999999")
        assert response.status_code == 404
    
    def test_get_hand_detail_invalid_id(self):
        """Test de error con ID inválido."""
        response = client.get("/api/v1/hands/invalid")
        assert response.status_code == 400


class TestEquityEndpoints:
    """Tests para endpoints de cálculo de equity."""
    
    def test_calculate_equity_valid(self):
        """Test de cálculo de equity con datos válidos."""
        payload = {
            "hero_cards": "AhKd",
            "villain_cards": "QsQh",
            "board": "",
            "iterations": 10000
        }
        
        response = client.post("/api/v1/equity/calculate", json=payload)
        
        # Si el FFI no está disponible, esperamos 503
        if response.status_code == 503:
            pytest.skip("FFI module not available")
        
        assert response.status_code == 200
        
        data = response.json()
        assert "hero_equity" in data
        assert "villain_equity" in data
        assert "tie_equity" in data
        assert "simulations_run" in data
        assert "elapsed_ms" in data
        
        # Verificar rangos
        assert 0 <= data["hero_equity"] <= 1
        assert 0 <= data["villain_equity"] <= 1
        assert 0 <= data["tie_equity"] <= 1
        
        # La suma debe ser aproximadamente 1.0
        total = data["hero_equity"] + data["villain_equity"] + data["tie_equity"]
        assert abs(total - 1.0) < 0.01
    
    def test_calculate_equity_with_board(self):
        """Test de cálculo con board (flop)."""
        payload = {
            "hero_cards": "AhKh",
            "villain_cards": "QsQd",
            "board": "Kd7c2h",
            "iterations": 10000
        }
        
        response = client.post("/api/v1/equity/calculate", json=payload)
        
        if response.status_code == 503:
            pytest.skip("FFI module not available")
        
        assert response.status_code == 200
    
    def test_calculate_equity_invalid_cards(self):
        """Test de error con cartas inválidas."""
        payload = {
            "hero_cards": "XxYy",
            "villain_cards": "QsQh",
            "board": "",
            "iterations": 10000
        }
        
        response = client.post("/api/v1/equity/calculate", json=payload)
        assert response.status_code == 422  # Validation error
    
    def test_calculate_equity_invalid_board(self):
        """Test de error con board inválido (2 cartas)."""
        payload = {
            "hero_cards": "AhKd",
            "villain_cards": "QsQh",
            "board": "KdQh",  # Solo 2 cartas (inválido)
            "iterations": 10000
        }
        
        response = client.post("/api/v1/equity/calculate", json=payload)
        assert response.status_code == 422
    
    def test_calculate_equity_too_many_iterations(self):
        """Test de validación de iteraciones máximas."""
        payload = {
            "hero_cards": "AhKd",
            "villain_cards": "QsQh",
            "board": "",
            "iterations": 10000000  # Más del límite
        }
        
        response = client.post("/api/v1/equity/calculate", json=payload)
        assert response.status_code == 422
    
    def test_calculate_multiway_equity(self):
        """Test de cálculo multiway."""
        payload = {
            "hands": ["AhKd", "QsQh", "8c8d"],
            "board": "",
            "iterations": 10000
        }
        
        response = client.post("/api/v1/equity/calculate/multiway", json=payload)
        
        if response.status_code == 503:
            pytest.skip("FFI module not available")
        
        assert response.status_code == 200
        
        data = response.json()
        assert "equities" in data
        assert len(data["equities"]) == 3
        
        # La suma debe ser aproximadamente 1.0
        total = sum(data["equities"])
        assert abs(total - 1.0) < 0.01
    
    def test_calculate_multiway_invalid_hands(self):
        """Test de error con menos de 2 manos."""
        payload = {
            "hands": ["AhKd"],
            "board": "",
            "iterations": 10000
        }
        
        response = client.post("/api/v1/equity/calculate/multiway", json=payload)
        assert response.status_code == 422


class TestOpenAPIDocumentation:
    """Tests para verificar que la documentación OpenAPI está disponible."""
    
    def test_swagger_ui_available(self):
        """Verifica que Swagger UI esté accesible."""
        response = client.get("/docs")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
    
    def test_redoc_available(self):
        """Verifica que ReDoc esté accesible."""
        response = client.get("/redoc")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
    
    def test_openapi_json_available(self):
        """Verifica que el schema OpenAPI esté disponible."""
        response = client.get("/api/v1/openapi.json")
        assert response.status_code == 200
        
        data = response.json()
        assert "openapi" in data
        assert "info" in data
        assert "paths" in data
        
        # Verificar que los endpoints estén documentados
        paths = data["paths"]
        assert "/api/v1/stats/player/{player_name}" in paths
        assert "/api/v1/hands/recent" in paths
        assert "/api/v1/hands/{hand_id}" in paths
        assert "/api/v1/equity/calculate" in paths

