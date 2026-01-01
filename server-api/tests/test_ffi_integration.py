"""
Tests de integración para el puente FFI Rust-Python.

Estos tests verifican que las funciones Rust expuestas via PyO3
funcionan correctamente desde Python.

Requisitos:
- El módulo poker_ffi debe estar compilado con maturin
- Ejecutar: cd backend/ffi && maturin develop

Ejecución:
    pytest tests/test_ffi_integration.py -v
"""

import pytest
from unittest.mock import patch, MagicMock
import sys


# ============================================================================
# FIXTURES
# ============================================================================

@pytest.fixture
def mock_ffi():
    """Mock del módulo FFI para tests sin compilación Rust."""
    mock_module = MagicMock()
    
    # Mock parse_winamax_files
    mock_parse_result = MagicMock()
    mock_parse_result.total_hands = 150
    mock_parse_result.successful_files = 3
    mock_parse_result.failed_files = 0
    mock_parse_result.elapsed_ms = 45
    mock_module.parse_winamax_files.return_value = mock_parse_result
    
    # Mock calculate_equity
    mock_equity_result = MagicMock()
    mock_equity_result.hero_equity = 0.82
    mock_equity_result.villain_equity = 0.18
    mock_equity_result.tie_equity = 0.0
    mock_equity_result.simulations_run = 100000
    mock_equity_result.converged_early = False
    mock_equity_result.standard_error = 0.001
    mock_module.calculate_equity.return_value = mock_equity_result
    
    # Mock calculate_equity_multiway
    mock_module.calculate_equity_multiway.return_value = [0.65, 0.25, 0.10]
    
    # Mock utilities
    mock_module.is_simd_available.return_value = True
    mock_module.version.return_value = "0.1.0"
    mock_module.system_info.return_value = "Poker FFI v0.1.0\n- SIMD AVX2: Habilitado"
    
    return mock_module


@pytest.fixture
def bridge_with_mock(mock_ffi):
    """Bridge module con FFI mockeado."""
    # Patch el import del módulo FFI
    with patch.dict(sys.modules, {'poker_ffi': mock_ffi}):
        # Reimportar el módulo bridge para que use el mock
        from app import bridge
        # Forzar recarga del módulo _ffi
        bridge._ffi = mock_ffi
        yield bridge


# ============================================================================
# TESTS DE DISPONIBILIDAD
# ============================================================================

class TestFFIAvailability:
    """Tests para verificar disponibilidad del módulo FFI."""
    
    def test_is_ffi_available_with_mock(self, bridge_with_mock):
        """Verifica que is_ffi_available retorna True cuando FFI está disponible."""
        assert bridge_with_mock.is_ffi_available() is True
    
    def test_get_version(self, bridge_with_mock):
        """Verifica que get_version retorna la versión correcta."""
        version = bridge_with_mock.get_version()
        assert version == "0.1.0"
    
    def test_get_system_info(self, bridge_with_mock):
        """Verifica que get_system_info retorna información del sistema."""
        info = bridge_with_mock.get_system_info()
        assert "Poker FFI" in info
        assert "SIMD" in info


# ============================================================================
# TESTS DE PARSING
# ============================================================================

class TestParsing:
    """Tests para funciones de parsing de historiales Winamax."""
    
    def test_parse_files_success(self, bridge_with_mock, mock_ffi):
        """Verifica que parse_files procesa archivos correctamente."""
        result = bridge_with_mock.parse_files(["file1.txt", "file2.txt", "file3.txt"])
        
        assert result["total_hands"] == 150
        assert result["successful_files"] == 3
        assert result["failed_files"] == 0
        assert result["elapsed_ms"] == 45
        
        # Verificar que se llamó al FFI
        mock_ffi.parse_winamax_files.assert_called_once_with(
            ["file1.txt", "file2.txt", "file3.txt"]
        )
    
    def test_parse_files_empty_list(self, bridge_with_mock, mock_ffi):
        """Verifica que parse_files maneja listas vacías."""
        # Configurar mock para lista vacía
        mock_result = MagicMock()
        mock_result.total_hands = 0
        mock_result.successful_files = 0
        mock_result.failed_files = 0
        mock_result.elapsed_ms = 0
        mock_ffi.parse_winamax_files.return_value = mock_result
        
        result = bridge_with_mock.parse_files([])
        
        assert result["total_hands"] == 0
        assert result["successful_files"] == 0
    
    def test_parse_files_with_details(self, bridge_with_mock, mock_ffi):
        """Verifica que parse_files_with_details retorna detalles de manos."""
        # Configurar mock
        mock_summary = MagicMock()
        mock_summary.hand_id = "123-456-789"
        mock_summary.timestamp = "2024-01-15 20:30:00"
        mock_summary.table_name = "Lyon 01"
        mock_summary.player_count = 6
        mock_summary.hero_played = True
        mock_summary.total_pot_cents = 250
        mock_ffi.parse_winamax_with_details.return_value = [mock_summary]
        
        result = bridge_with_mock.parse_files_with_details(["file.txt"])
        
        assert len(result) == 1
        assert result[0]["hand_id"] == "123-456-789"
        assert result[0]["hero_played"] is True
        assert result[0]["player_count"] == 6


# ============================================================================
# TESTS DE EQUITY
# ============================================================================

class TestEquity:
    """Tests para funciones de cálculo de equity."""
    
    def test_calculate_equity_aa_vs_kk(self, bridge_with_mock, mock_ffi):
        """Verifica cálculo de equity AA vs KK."""
        result = bridge_with_mock.calculate_equity("AsAh", "KsKh", "", 100000)
        
        assert result["hero_equity"] == 0.82
        assert result["villain_equity"] == 0.18
        assert result["hero_percent"] == 82.0
        assert result["villain_percent"] == 18.0
        assert result["simulations_run"] == 100000
        
        # Verificar llamada al FFI
        mock_ffi.calculate_equity.assert_called_once_with(
            "AsAh", "KsKh", "", 100000
        )
    
    def test_calculate_equity_with_board(self, bridge_with_mock, mock_ffi):
        """Verifica cálculo de equity con board."""
        bridge_with_mock.calculate_equity("AsAh", "KsKh", "Qh7s2c", 50000)
        
        mock_ffi.calculate_equity.assert_called_once_with(
            "AsAh", "KsKh", "Qh7s2c", 50000
        )
    
    def test_calculate_equity_multiway(self, bridge_with_mock, mock_ffi):
        """Verifica cálculo de equity multiway."""
        result = bridge_with_mock.calculate_equity_multiway(
            ["AsAh", "KsKh", "QsQh"],
            "",
            30000
        )
        
        assert len(result) == 3
        assert result[0] == 0.65  # AA tiene mayor equity
        assert result[1] == 0.25  # KK
        assert result[2] == 0.10  # QQ
        
        mock_ffi.calculate_equity_multiway.assert_called_once_with(
            ["AsAh", "KsKh", "QsQh"], "", 30000
        )
    
    def test_is_simd_available(self, bridge_with_mock):
        """Verifica detección de SIMD AVX2."""
        assert bridge_with_mock.is_simd_available() is True


# ============================================================================
# TESTS DE ERRORES
# ============================================================================

class TestErrorHandling:
    """Tests para manejo de errores FFI."""
    
    def test_ffi_not_available_parse(self):
        """Verifica error cuando FFI no está disponible."""
        from app import bridge
        original_ffi = bridge._ffi
        bridge._ffi = None
        
        try:
            with pytest.raises(RuntimeError, match="FFI no disponible"):
                bridge.parse_files(["file.txt"])
        finally:
            bridge._ffi = original_ffi
    
    def test_ffi_not_available_equity(self):
        """Verifica error cuando FFI no está disponible para equity."""
        from app import bridge
        original_ffi = bridge._ffi
        bridge._ffi = None
        
        try:
            with pytest.raises(RuntimeError, match="FFI no disponible"):
                bridge.calculate_equity("AhKd", "QsQh")
        finally:
            bridge._ffi = original_ffi
    
    def test_io_error_propagation(self, bridge_with_mock, mock_ffi):
        """Verifica que los errores de IO se propagan correctamente."""
        mock_ffi.parse_winamax_files.side_effect = IOError("[101] No se puede acceder")
        
        with pytest.raises(IOError):
            bridge_with_mock.parse_files(["nonexistent.txt"])
    
    def test_value_error_invalid_cards(self, bridge_with_mock, mock_ffi):
        """Verifica que los errores de cartas inválidas se propagan."""
        mock_ffi.calculate_equity.side_effect = ValueError("[201] Carta inválida")
        
        with pytest.raises(ValueError):
            bridge_with_mock.calculate_equity("XxYy", "QsQh")


# ============================================================================
# TESTS DE INTEGRACIÓN REAL (requieren FFI compilado)
# ============================================================================

@pytest.mark.skipif(
    True,  # Cambiar a condición real cuando FFI esté compilado
    reason="Requiere módulo FFI compilado con maturin"
)
class TestRealFFI:
    """
    Tests de integración real con el módulo FFI compilado.
    
    Estos tests solo se ejecutan si poker_ffi está disponible.
    Para habilitarlos:
    1. Compilar FFI: cd backend/ffi && maturin develop
    2. Cambiar skipif a: not _ffi_available()
    """
    
    def test_real_equity_calculation(self):
        """Test real de cálculo de equity."""
        from app.bridge import calculate_equity, is_ffi_available
        
        if not is_ffi_available():
            pytest.skip("FFI no disponible")
        
        result = calculate_equity("AsAh", "KsKh", "", 10000)
        
        # AA vs KK debería ser aproximadamente 82% vs 18%
        assert 0.78 < result["hero_equity"] < 0.86
        assert 0.14 < result["villain_equity"] < 0.22
    
    def test_real_simd_detection(self):
        """Test real de detección SIMD."""
        from app.bridge import is_simd_available, is_ffi_available
        
        if not is_ffi_available():
            pytest.skip("FFI no disponible")
        
        # En hardware moderno (Ryzen 3800X), SIMD debería estar disponible
        result = is_simd_available()
        assert isinstance(result, bool)

