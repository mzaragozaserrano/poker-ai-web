/**
 * hooks/useWebSocket.ts
 * Hook para gestionar la conexión WebSocket con el backend
 * WS Endpoint: ws://127.0.0.1:8000/ws
 */

import { useEffect, useRef, useState, useCallback } from 'react'
import {
  WebSocketMessage,
  ConnectionStatus,
  NewHandMessage,
  ConnectionAckMessage,
  HeartbeatMessage,
  ErrorMessage,
} from '../types/api'

// ============================================================================
// CONSTANTES
// ============================================================================

const WS_URL = 'ws://127.0.0.1:8000/ws'
const INITIAL_RECONNECT_DELAY = 5000 // 5 segundos
const MAX_RECONNECT_DELAY = 60000 // 60 segundos
const RECONNECT_BACKOFF_MULTIPLIER = 2

// ============================================================================
// TIPOS
// ============================================================================

export interface UseWebSocketOptions {
  /**
   * URL del WebSocket (default: ws://127.0.0.1:8000/ws)
   */
  url?: string

  /**
   * Nombre del cliente (opcional, se pasa como query param)
   */
  clientName?: string

  /**
   * Si debe conectarse automáticamente al montar (default: true)
   */
  autoConnect?: boolean

  /**
   * Si debe reconectarse automáticamente (default: true)
   */
  autoReconnect?: boolean

  /**
   * Callback cuando se recibe un mensaje de nueva mano
   */
  onNewHand?: (message: NewHandMessage) => void

  /**
   * Callback cuando se recibe confirmación de conexión
   */
  onConnectionAck?: (message: ConnectionAckMessage) => void

  /**
   * Callback cuando se recibe un heartbeat
   */
  onHeartbeat?: (message: HeartbeatMessage) => void

  /**
   * Callback cuando se recibe un error
   */
  onError?: (message: ErrorMessage) => void

  /**
   * Callback para cualquier mensaje (útil para debugging)
   */
  onMessage?: (message: WebSocketMessage) => void
}

export interface UseWebSocketReturn {
  /**
   * Estado actual de la conexión
   */
  status: ConnectionStatus

  /**
   * Si está conectado
   */
  isConnected: boolean

  /**
   * Último mensaje recibido
   */
  lastMessage: WebSocketMessage | null

  /**
   * ID del cliente (asignado por el servidor en connection_ack)
   */
  clientId: string | null

  /**
   * Historial de mensajes (últimos 50)
   */
  messageHistory: WebSocketMessage[]

  /**
   * Función para conectar manualmente
   */
  connect: () => void

  /**
   * Función para desconectar manualmente
   */
  disconnect: () => void

  /**
   * Función para enviar un mensaje (si fuera necesario en el futuro)
   */
  send: (data: string) => void
}

// ============================================================================
// HOOK
// ============================================================================

export function useWebSocket(options: UseWebSocketOptions = {}): UseWebSocketReturn {
  const {
    url = WS_URL,
    clientName = 'react_client',
    autoConnect = true,
    autoReconnect = true,
    onNewHand,
    onConnectionAck,
    onHeartbeat,
    onError,
    onMessage,
  } = options

  // Estados
  const [status, setStatus] = useState<ConnectionStatus>('disconnected')
  const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null)
  const [clientId, setClientId] = useState<string | null>(null)
  const [messageHistory, setMessageHistory] = useState<WebSocketMessage[]>([])

  // Referencias para evitar recreación
  const wsRef = useRef<WebSocket | null>(null)
  const reconnectTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const reconnectDelayRef = useRef(INITIAL_RECONNECT_DELAY)
  const shouldReconnectRef = useRef(autoReconnect)
  const isManualDisconnectRef = useRef(false)

  // Construir URL con query params
  const buildUrl = useCallback(() => {
    const wsUrl = new URL(url)
    if (clientName) {
      wsUrl.searchParams.set('client_name', clientName)
    }
    return wsUrl.toString()
  }, [url, clientName])

  // Agregar mensaje al historial (mantener últimos 50)
  const addToHistory = useCallback((message: WebSocketMessage) => {
    setMessageHistory((prev) => {
      const newHistory = [message, ...prev]
      return newHistory.slice(0, 50) // Mantener solo últimos 50
    })
  }, [])

  // Manejar mensajes recibidos
  const handleMessage = useCallback(
    (event: MessageEvent) => {
      try {
        const message = JSON.parse(event.data) as WebSocketMessage

        // Actualizar último mensaje y agregar al historial
        setLastMessage(message)
        addToHistory(message)

        // Callback genérico
        onMessage?.(message)

        // Callbacks específicos por tipo de mensaje
        switch (message.type) {
          case 'connection_ack': {
            const ackMessage = message as ConnectionAckMessage
            setClientId(ackMessage.client_id)
            onConnectionAck?.(ackMessage)
            console.log('[WebSocket] Connection acknowledged:', ackMessage.client_id)
            break
          }

          case 'heartbeat': {
            const heartbeatMessage = message as HeartbeatMessage
            onHeartbeat?.(heartbeatMessage)
            // No logueamos heartbeats para no saturar consola
            break
          }

          case 'new_hand': {
            const newHandMessage = message as NewHandMessage
            onNewHand?.(newHandMessage)
            console.log('[WebSocket] New hand detected:', newHandMessage.hand_id)
            break
          }

          case 'error': {
            const errorMessage = message as ErrorMessage
            onError?.(errorMessage)
            console.error('[WebSocket] Error message:', errorMessage.message)
            break
          }

          default:
            console.warn('[WebSocket] Unknown message type:', message)
        }
      } catch (error) {
        console.error('[WebSocket] Failed to parse message:', error)
      }
    },
    [onMessage, onConnectionAck, onHeartbeat, onNewHand, onError, addToHistory],
  )

  // Función para conectar
  const connect = useCallback(() => {
    // Si ya hay una conexión activa, no hacer nada
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      console.log('[WebSocket] Already connected')
      return
    }

    // Si está conectando, esperar
    if (wsRef.current?.readyState === WebSocket.CONNECTING) {
      console.log('[WebSocket] Already connecting')
      return
    }

    console.log('[WebSocket] Connecting to:', buildUrl())
    setStatus('connecting')
    isManualDisconnectRef.current = false

    try {
      const ws = new WebSocket(buildUrl())
      wsRef.current = ws

      // Evento: conexión abierta
      ws.onopen = () => {
        console.log('[WebSocket] Connected')
        setStatus('connected')
        reconnectDelayRef.current = INITIAL_RECONNECT_DELAY // Reset delay
      }

      // Evento: mensaje recibido
      ws.onmessage = handleMessage

      // Evento: error
      ws.onerror = (error) => {
        console.error('[WebSocket] Connection error:', error)
      }

      // Evento: conexión cerrada
      ws.onclose = (event) => {
        console.log('[WebSocket] Disconnected:', event.code, event.reason)
        setStatus('disconnected')
        wsRef.current = null

        // Reconexión automática si no fue desconexión manual
        if (shouldReconnectRef.current && !isManualDisconnectRef.current) {
          const delay = reconnectDelayRef.current
          console.log(`[WebSocket] Reconnecting in ${delay / 1000}s...`)
          setStatus('reconnecting')

          reconnectTimeoutRef.current = setTimeout(() => {
            // Backoff exponencial
            reconnectDelayRef.current = Math.min(
              reconnectDelayRef.current * RECONNECT_BACKOFF_MULTIPLIER,
              MAX_RECONNECT_DELAY,
            )
            connect()
          }, delay)
        }
      }
    } catch (error) {
      console.error('[WebSocket] Failed to create connection:', error)
      setStatus('disconnected')
    }
  }, [buildUrl, handleMessage])

  // Función para desconectar
  const disconnect = useCallback(() => {
    console.log('[WebSocket] Disconnecting manually')
    isManualDisconnectRef.current = true
    shouldReconnectRef.current = false

    // Cancelar reconexión pendiente
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current)
      reconnectTimeoutRef.current = null
    }

    // Cerrar WebSocket
    if (wsRef.current) {
      wsRef.current.close()
      wsRef.current = null
    }

    setStatus('disconnected')
  }, [])

  // Función para enviar mensajes (por si se necesita en el futuro)
  const send = useCallback((data: string) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(data)
    } else {
      console.warn('[WebSocket] Cannot send message: not connected')
    }
  }, [])

  // Efecto: Conectar automáticamente al montar
  useEffect(() => {
    if (autoConnect) {
      connect()
    }

    // Cleanup al desmontar
    return () => {
      console.log('[WebSocket] Cleaning up...')
      isManualDisconnectRef.current = true
      shouldReconnectRef.current = false

      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current)
      }

      if (wsRef.current) {
        wsRef.current.close()
      }
    }
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  // Actualizar ref de auto-reconexión cuando cambie la opción
  useEffect(() => {
    shouldReconnectRef.current = autoReconnect
  }, [autoReconnect])

  return {
    status,
    isConnected: status === 'connected',
    lastMessage,
    clientId,
    messageHistory,
    connect,
    disconnect,
    send,
  }
}

