// Configuración de API backend
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
}

export async function fetchAPI<T>(endpoint: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  })

  if (!response.ok) {
    throw new Error(`API Error: ${response.statusText}`)
  }

  return response.json()
}

