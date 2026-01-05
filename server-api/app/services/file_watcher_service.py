"""
File watcher service para integración con WebSocket.

Conecta el file watcher de Rust con el WebSocket manager de FastAPI.
"""

from datetime import datetime
from typing import List, Optional
from pathlib import Path

from app.services.websocket_manager import get_ws_manager
from app.utils.logger import get_logger, audit_log

logger = get_logger(__name__)


class FileWatcherService:
    """
    Servicio que integra el file watcher de Rust con WebSocket.
    
    Detecta nuevos archivos de historial y notifica a clientes conectados
    vía WebSocket.
    """
    
    def __init__(self, watch_path: str):
        """
        Inicializa el servicio.
        
        Args:
            watch_path: Ruta del directorio a monitorear
        """
        self.watch_path = watch_path
        self._is_running = False
        logger.info("file_watcher_service_initialized", watch_path=watch_path)
    
    def start(self) -> None:
        """
        Inicia el file watcher con integración a WebSocket.
        
        El watcher detecta nuevos archivos, los parsea automáticamente
        y notifica a todos los clientes WebSocket conectados.
        """
        if self._is_running:
            logger.warning("file_watcher_already_running", watch_path=self.watch_path)
            return
        
        # Verificar que el directorio existe
        if not Path(self.watch_path).exists():
            logger.error("watch_path_not_found", watch_path=self.watch_path)
            raise FileNotFoundError(f"Watch path does not exist: {self.watch_path}")
        
        try:
            # Importar FFI
            from app.bridge import poker_ffi
            
            # Crear configuración
            config = poker_ffi.PyWatcherConfig(
                watch_path=self.watch_path,
                max_retries=3,
                retry_delay_ms=100
            )
            
            # Iniciar watcher con callback
            poker_ffi.start_file_watcher_with_parsing(
                config,
                self._on_hands_detected
            )
            
            self._is_running = True
            audit_log(
                "file_watcher_started",
                watch_path=self.watch_path,
                max_retries=3,
            )
            
        except Exception as e:
            logger.error(
                "file_watcher_start_failed",
                watch_path=self.watch_path,
                error=str(e),
                exc_info=True,
            )
            raise
    
    def _on_hands_detected(self, hands: List) -> None:
        """
        Callback que se ejecuta cuando se detectan y parsean nuevas manos.
        
        Args:
            hands: Lista de PyHandSummary con las manos detectadas
        """
        if not hands:
            logger.debug("no_hands_detected_in_file")
            return
        
        audit_log("new_hands_detected", hands_count=len(hands))
        
        # Obtener el gestor de WebSocket
        ws_manager = get_ws_manager()
        
        # Notificar cada mano a los clientes WebSocket
        for hand in hands:
            try:
                # Extraer información de la mano
                hand_id = hand.hand_id
                timestamp = datetime.fromisoformat(hand.timestamp) if hand.timestamp else datetime.utcnow()
                
                # Calcular resultado del hero (si participó)
                hero_result = None
                if hand.hero_played:
                    # Por ahora no tenemos el resultado exacto, se necesitaría
                    # parsear más información de la mano. Lo dejamos como None.
                    hero_result = None
                
                # Notificar vía WebSocket (usar queue_broadcast para llamadas desde threads externos)
                from app.models.websocket import NewHandMessage
                message = NewHandMessage(
                    hand_id=hand_id,
                    timestamp=timestamp,
                    hero_result=hero_result,
                    hero_position=None,  # Se podría extraer del parsing
                    stakes="0.05/0.10",  # Se podría extraer del parsing
                )
                ws_manager.queue_broadcast(message.model_dump_json())
                
                logger.debug("hand_notification_sent", hand_id=hand_id)
                
            except Exception as e:
                logger.error(
                    "hand_notification_failed",
                    hand_id=hand.hand_id,
                    error=str(e),
                    exc_info=True,
                )
    
    def stop(self) -> None:
        """
        Detiene el file watcher.
        
        Nota: Por ahora el watcher de Rust no tiene un método stop explícito,
        se detiene cuando el proceso termina.
        """
        self._is_running = False
        audit_log("file_watcher_stopped", watch_path=self.watch_path)
    
    @property
    def is_running(self) -> bool:
        """Retorna True si el watcher está corriendo."""
        return self._is_running


# Singleton global del servicio
_file_watcher_service: Optional[FileWatcherService] = None


def get_file_watcher_service(watch_path: Optional[str] = None) -> FileWatcherService:
    """
    Obtiene la instancia global del FileWatcherService.
    
    Args:
        watch_path: Ruta del directorio a monitorear (solo necesaria en primera llamada)
        
    Returns:
        Instancia singleton del FileWatcherService
    """
    global _file_watcher_service
    if _file_watcher_service is None:
        if watch_path is None:
            raise ValueError("watch_path is required for first initialization")
        _file_watcher_service = FileWatcherService(watch_path)
    return _file_watcher_service

