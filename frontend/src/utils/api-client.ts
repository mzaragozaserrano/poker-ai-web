/**
 * utils/api-client.ts
 * Cliente HTTP centralizado para comunicación con la API backend
 * Base URL: http://127.0.0.1:8000/api/v1
 */

import { isErrorResponse } from '../types/api'

// ============================================================================
// CONFIGURACIÓN
// ============================================================================

export const API_BASE_URL = 'http://127.0.0.1:8000/api/v1'
export const WS_URL = 'ws://127.0.0.1:8000/ws'

export const API_ENDPOINTS = {
  health: '/health',
  stats: {
    player: (name: string) => `/stats/player/${name}`,
  },
  hands: {
    recent: '/hands/recent',
    byId: (id: string) => `/hands/${id}`,
  },
  equity: {
    calculate: '/equity/calculate',
    calculateMultiway: '/equity/calculate/multiway',
  },
} as const

// ============================================================================
// ERRORES PERSONALIZADOS
// ============================================================================

export class ApiError extends Error {
  constructor(
    public status: number,
    public statusText: string,
    public detail?: string,
  ) {
    super(`API Error [${status}]: ${statusText}${detail ? ` - ${detail}` : ''}`)
    this.name = 'ApiError'
  }
}

// ============================================================================
// CLIENTE HTTP GENÉRICO
// ============================================================================

export interface FetchOptions extends RequestInit {
  timeout?: number // Timeout en ms (default: 30s)
  throwOnError?: boolean // Lanzar error si response no es ok (default: true)
}

/**
 * Cliente HTTP centralizado con manejo de errores
 * Soporta timeout, reintentos, y tipado genérico
 */
export async function fetchAPI<T = unknown>(
  endpoint: string,
  options?: FetchOptions,
): Promise<T> {
  const { timeout = 30000, throwOnError = true, ...fetchOptions } = options || {}

  const controller = new AbortController()
  const timeoutId = setTimeout(() => controller.abort(), timeout)

  try {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...fetchOptions,
      signal: controller.signal,
      headers: {
        'Content-Type': 'application/json',
        ...fetchOptions.headers,
      },
    })

    clearTimeout(timeoutId)

    const data = await response.json()

    // Manejo de respuestas de error
    if (!response.ok) {
      if (throwOnError) {
        throw new ApiError(
          response.status,
          response.statusText,
          isErrorResponse(data) ? data.detail || data.error : undefined,
        )
      }
      return data
    }

    return data as T
  } catch (error) {
    clearTimeout(timeoutId)

    // Manejo específico de timeouts
    if (error instanceof DOMException && error.name === 'AbortError') {
      throw new ApiError(408, 'Request Timeout', `Request timeout after ${timeout}ms`)
    }

    // Re-lanzar errores de API
    if (error instanceof ApiError) {
      throw error
    }

    // Errores de red u otros
    if (error instanceof Error) {
      throw new ApiError(0, 'Network Error', error.message)
    }

    throw new ApiError(0, 'Unknown Error', 'An unknown error occurred')
  }
}

// ============================================================================
// MÉTODOS HTTP CON TIPADO
// ============================================================================

export const apiClient = {
  /**
   * GET request
   */
  get: <T = unknown>(endpoint: string, options?: FetchOptions) =>
    fetchAPI<T>(endpoint, { ...options, method: 'GET' }),

  /**
   * POST request
   */
  post: <T = unknown>(endpoint: string, body?: unknown, options?: FetchOptions) =>
    fetchAPI<T>(endpoint, {
      ...options,
      method: 'POST',
      body: body ? JSON.stringify(body) : undefined,
    }),

  /**
   * PUT request
   */
  put: <T = unknown>(endpoint: string, body?: unknown, options?: FetchOptions) =>
    fetchAPI<T>(endpoint, {
      ...options,
      method: 'PUT',
      body: body ? JSON.stringify(body) : undefined,
    }),

  /**
   * DELETE request
   */
  delete: <T = unknown>(endpoint: string, options?: FetchOptions) =>
    fetchAPI<T>(endpoint, { ...options, method: 'DELETE' }),
}

