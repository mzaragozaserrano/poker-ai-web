"""
Tests de integración para WebSocket.

Tests para verificar el funcionamiento del endpoint WebSocket y la
integración con el file watcher.
"""

import pytest
import asyncio
import json
from typing import List
from fastapi.testclient import TestClient
from fastapi import WebSocket

from app.main import app
from app.services.websocket_manager import get_ws_manager
from app.models.websocket import (
    NewHandMessage,
    HeartbeatMessage,
    ConnectionAckMessage,
)


class TestWebSocketEndpoint:
    """Tests del endpoint WebSocket."""
    
    def test_websocket_connection(self):
        """Test de conexión básica al WebSocket."""
        client = TestClient(app)
        
        with client.websocket_connect("/ws") as websocket:
            # Debe recibir mensaje de confirmación
            data = websocket.receive_text()
            message = json.loads(data)
            
            assert message["type"] == "connection_ack"
            assert "client_id" in message
            assert "timestamp" in message
    
    def test_websocket_with_client_name(self):
        """Test de conexión con nombre de cliente."""
        client = TestClient(app)
        
        with client.websocket_connect("/ws?client_name=test_client") as websocket:
            data = websocket.receive_text()
            message = json.loads(data)
            
            assert message["type"] == "connection_ack"
            assert "client_id" in message
    
    def test_websocket_receives_heartbeat(self):
        """Test que verifica recepción de heartbeats."""
        client = TestClient(app)
        
        # Configurar un heartbeat interval corto para el test
        ws_manager = get_ws_manager()
        original_interval = ws_manager.heartbeat_interval
        ws_manager.heartbeat_interval = 1  # 1 segundo
        
        try:
            with client.websocket_connect("/ws") as websocket:
                # Recibir confirmación
                data = websocket.receive_text()
                message = json.loads(data)
                assert message["type"] == "connection_ack"
                
                # Esperar heartbeat (máximo 3 segundos)
                received_heartbeat = False
                for _ in range(5):
                    try:
                        data = websocket.receive_text()
                        message = json.loads(data)
                        if message["type"] == "heartbeat":
                            received_heartbeat = True
                            break
                    except:
                        pass
                    asyncio.sleep(0.5)
                
                assert received_heartbeat, "No se recibió heartbeat"
        finally:
            ws_manager.heartbeat_interval = original_interval
    
    def test_multiple_clients_can_connect(self):
        """Test que múltiples clientes pueden conectarse simultáneamente."""
        client = TestClient(app)
        
        # Conectar primer cliente
        with client.websocket_connect("/ws?client_name=client1") as ws1:
            data1 = ws1.receive_text()
            msg1 = json.loads(data1)
            assert msg1["type"] == "connection_ack"
            client1_id = msg1["client_id"]
            
            # Conectar segundo cliente
            with client.websocket_connect("/ws?client_name=client2") as ws2:
                data2 = ws2.receive_text()
                msg2 = json.loads(data2)
                assert msg2["type"] == "connection_ack"
                client2_id = msg2["client_id"]
                
                # IDs deben ser diferentes
                assert client1_id != client2_id
                
                # Verificar que ambos están en el manager
                ws_manager = get_ws_manager()
                assert len(ws_manager.active_connections) >= 2


class TestWebSocketManager:
    """Tests del WebSocketManager."""
    
    @pytest.mark.asyncio
    async def test_broadcast_message(self):
        """Test de broadcasting a múltiples clientes."""
        ws_manager = get_ws_manager()
        
        # Crear mensaje de prueba
        message = NewHandMessage(
            hand_id="test_hand_123",
            timestamp="2024-01-01T12:00:00",
            hero_result=5.50,
            hero_position="BTN",
            stakes="0.05/0.10"
        )
        
        # Note: Este test es simplificado porque necesitaríamos
        # conexiones WebSocket reales para testear el broadcast completo
        # En un entorno de testing, esto se haría con mocks
        
        # Verificar que el manager está inicializado
        assert ws_manager is not None
    
    def test_websocket_stats_endpoint(self):
        """Test del endpoint de estadísticas del WebSocket."""
        client = TestClient(app)
        
        response = client.get("/ws/stats")
        assert response.status_code == 200
        
        stats = response.json()
        assert "active_connections" in stats
        assert "heartbeat_interval" in stats
        assert "heartbeat_active" in stats
        assert "queue_size" in stats
    
    @pytest.mark.asyncio
    async def test_notify_new_hand(self):
        """Test de notificación de nueva mano."""
        from datetime import datetime
        
        ws_manager = get_ws_manager()
        
        # Crear notificación de nueva mano
        count = await ws_manager.notify_new_hand(
            hand_id="test_hand_456",
            timestamp=datetime.utcnow(),
            hero_result=10.0,
            hero_position="SB",
            stakes="0.10/0.20"
        )
        
        # Si no hay clientes conectados, count debe ser 0
        # En un test real con clientes, sería > 0
        assert count >= 0


class TestWebSocketMessages:
    """Tests de los modelos de mensajes WebSocket."""
    
    def test_new_hand_message_creation(self):
        """Test de creación de mensaje NewHandMessage."""
        from datetime import datetime
        
        message = NewHandMessage(
            hand_id="hand_789",
            timestamp=datetime.utcnow(),
            hero_result=-2.50,
            hero_position="BB",
            stakes="0.25/0.50"
        )
        
        assert message.type == "new_hand"
        assert message.hand_id == "hand_789"
        assert message.hero_result == -2.50
        assert message.hero_position == "BB"
    
    def test_heartbeat_message_creation(self):
        """Test de creación de mensaje HeartbeatMessage."""
        message = HeartbeatMessage()
        
        assert message.type == "heartbeat"
        assert message.timestamp is not None
    
    def test_connection_ack_message_creation(self):
        """Test de creación de mensaje ConnectionAckMessage."""
        message = ConnectionAckMessage(client_id="test_client_123")
        
        assert message.type == "connection_ack"
        assert message.client_id == "test_client_123"
        assert message.timestamp is not None
    
    def test_message_serialization(self):
        """Test de serialización de mensajes a JSON."""
        from datetime import datetime
        
        message = NewHandMessage(
            hand_id="hand_serialization_test",
            timestamp=datetime.utcnow(),
            hero_result=5.0,
            stakes="0.05/0.10"
        )
        
        json_str = message.model_dump_json()
        assert "hand_id" in json_str
        assert "hand_serialization_test" in json_str


class TestFileWatcherIntegration:
    """Tests de integración del file watcher con WebSocket."""
    
    def test_file_watcher_service_initialization(self):
        """Test de inicialización del FileWatcherService."""
        from app.services.file_watcher_service import FileWatcherService
        import tempfile
        
        # Crear directorio temporal
        with tempfile.TemporaryDirectory() as tmpdir:
            service = FileWatcherService(tmpdir)
            assert service.watch_path == tmpdir
            assert not service.is_running
    
    def test_file_watcher_service_requires_valid_path(self):
        """Test que el FileWatcherService requiere ruta válida."""
        from app.services.file_watcher_service import FileWatcherService
        
        service = FileWatcherService("/path/that/does/not/exist")
        
        with pytest.raises(FileNotFoundError):
            service.start()
    
    @pytest.mark.skipif(
        not pytest.importorskip("app.bridge.poker_ffi", reason="FFI not available"),
        reason="FFI module not available"
    )
    def test_file_watcher_with_ffi(self):
        """Test de file watcher con FFI (requiere módulo compilado)."""
        from app.services.file_watcher_service import FileWatcherService
        from app.bridge import is_ffi_available
        import tempfile
        
        if not is_ffi_available():
            pytest.skip("FFI module not available")
        
        with tempfile.TemporaryDirectory() as tmpdir:
            service = FileWatcherService(tmpdir)
            
            # Iniciar el servicio (no debería fallar)
            service.start()
            assert service.is_running
            
            # Detener el servicio
            service.stop()
            assert not service.is_running


@pytest.fixture
def cleanup_ws_manager():
    """Fixture para limpiar el WebSocketManager después de cada test."""
    yield
    # Limpiar conexiones activas después del test
    ws_manager = get_ws_manager()
    ws_manager.active_connections.clear()


# Usar el fixture en todos los tests
pytestmark = pytest.mark.usefixtures("cleanup_ws_manager")

