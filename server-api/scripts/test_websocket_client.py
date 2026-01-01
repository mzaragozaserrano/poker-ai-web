#!/usr/bin/env python3
"""
Script de ejemplo para probar el WebSocket de notificaciones.

Este script se conecta al WebSocket y muestra los mensajes recibidos.
Útil para testing manual del sistema.
"""

import asyncio
import json
import sys
from datetime import datetime
from typing import Optional

try:
    import websockets
except ImportError:
    print("Error: websockets no está instalado")
    print("Instalar con: pip install websockets")
    sys.exit(1)


class WebSocketClient:
    """Cliente WebSocket para testing."""
    
    def __init__(self, url: str = "ws://localhost:8000/ws", client_name: Optional[str] = None):
        """
        Inicializa el cliente.
        
        Args:
            url: URL del WebSocket
            client_name: Nombre opcional del cliente
        """
        self.url = url
        if client_name:
            self.url += f"?client_name={client_name}"
        self.client_id: Optional[str] = None
        self.running = True
    
    async def connect(self):
        """Conecta al WebSocket y escucha mensajes."""
        try:
            print(f"Conectando a {self.url}...")
            async with websockets.connect(self.url) as websocket:
                print("✓ Conectado exitosamente")
                
                while self.running:
                    try:
                        message = await websocket.recv()
                        self.handle_message(message)
                    except websockets.exceptions.ConnectionClosed:
                        print("✗ Conexión cerrada por el servidor")
                        break
                    except KeyboardInterrupt:
                        print("\n✓ Desconectando...")
                        self.running = False
                        break
        
        except ConnectionRefusedError:
            print("✗ Error: No se puede conectar al servidor")
            print("  Asegúrate de que el servidor está corriendo en localhost:8000")
        except Exception as e:
            print(f"✗ Error: {e}")
    
    def handle_message(self, message: str):
        """
        Procesa un mensaje recibido.
        
        Args:
            message: Mensaje JSON en formato string
        """
        try:
            data = json.loads(message)
            msg_type = data.get("type", "unknown")
            
            if msg_type == "connection_ack":
                self.handle_connection_ack(data)
            elif msg_type == "new_hand":
                self.handle_new_hand(data)
            elif msg_type == "heartbeat":
                self.handle_heartbeat(data)
            elif msg_type == "error":
                self.handle_error(data)
            else:
                print(f"⚠ Mensaje desconocido: {msg_type}")
                print(f"   {data}")
        
        except json.JSONDecodeError:
            print(f"✗ Error al parsear mensaje: {message}")
    
    def handle_connection_ack(self, data: dict):
        """Maneja mensaje de confirmación de conexión."""
        self.client_id = data.get("client_id")
        timestamp = data.get("timestamp")
        print(f"\n{'='*60}")
        print(f"CONEXIÓN CONFIRMADA")
        print(f"{'='*60}")
        print(f"  Client ID: {self.client_id}")
        print(f"  Timestamp: {timestamp}")
        print(f"{'='*60}\n")
    
    def handle_new_hand(self, data: dict):
        """Maneja notificación de nueva mano."""
        hand_id = data.get("hand_id")
        timestamp = data.get("timestamp")
        hero_result = data.get("hero_result")
        hero_position = data.get("hero_position")
        stakes = data.get("stakes")
        
        print(f"\n{'*'*60}")
        print(f"🎴 NUEVA MANO DETECTADA")
        print(f"{'*'*60}")
        print(f"  Hand ID:    {hand_id}")
        print(f"  Timestamp:  {timestamp}")
        print(f"  Stakes:     {stakes}")
        
        if hero_position:
            print(f"  Posición:   {hero_position}")
        
        if hero_result is not None:
            result_str = f"+${hero_result:.2f}" if hero_result > 0 else f"-${abs(hero_result):.2f}"
            emoji = "📈" if hero_result > 0 else "📉"
            print(f"  Resultado:  {emoji} {result_str}")
        else:
            print(f"  Resultado:  Hero no participó")
        
        print(f"{'*'*60}\n")
    
    def handle_heartbeat(self, data: dict):
        """Maneja mensaje de heartbeat."""
        timestamp = data.get("timestamp")
        now = datetime.utcnow().isoformat()
        print(f"💓 Heartbeat recibido (server: {timestamp}, local: {now})")
    
    def handle_error(self, data: dict):
        """Maneja mensaje de error."""
        message = data.get("message")
        timestamp = data.get("timestamp")
        print(f"\n{'!'*60}")
        print(f"ERROR DEL SERVIDOR")
        print(f"{'!'*60}")
        print(f"  Mensaje:   {message}")
        print(f"  Timestamp: {timestamp}")
        print(f"{'!'*60}\n")


async def main():
    """Función principal."""
    import argparse
    
    parser = argparse.ArgumentParser(description="Cliente WebSocket de prueba")
    parser.add_argument(
        "--url",
        default="ws://localhost:8000/ws",
        help="URL del WebSocket (default: ws://localhost:8000/ws)"
    )
    parser.add_argument(
        "--name",
        default="test_client",
        help="Nombre del cliente (default: test_client)"
    )
    
    args = parser.parse_args()
    
    print("="*60)
    print("Cliente WebSocket - Poker AI")
    print("="*60)
    print(f"URL:    {args.url}")
    print(f"Nombre: {args.name}")
    print("="*60)
    print("\nPresiona Ctrl+C para desconectar\n")
    
    client = WebSocketClient(url=args.url, client_name=args.name)
    
    try:
        await client.connect()
    except KeyboardInterrupt:
        print("\n✓ Cliente detenido por el usuario")


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n✓ Saliendo...")

