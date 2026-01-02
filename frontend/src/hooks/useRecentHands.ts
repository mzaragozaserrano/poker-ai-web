/**
 * hooks/useRecentHands.ts
 * Hook para obtener el historial reciente de manos
 * GET /api/v1/hands/recent
 */

import { useQuery, UseQueryResult } from '@tanstack/react-query'
import { HandSummary, GetRecentHandsResponse } from '../types/api'
import { apiClient, API_ENDPOINTS, ApiError } from '../utils/api-client'

interface UseRecentHandsOptions {
  limit?: number // Default: 20
  enabled?: boolean
}

interface RecentHandsData {
  hands: HandSummary[]
  total: number
  limit: number
}

/**
 * Hook para obtener manos recientes
 * @param limit - Número máximo de manos a obtener (default: 20)
 * @param enabled - Si está habilitada la query (default: true)
 * @returns Query result con historial de manos
 */
export function useRecentHands(
  options?: UseRecentHandsOptions,
): UseQueryResult<RecentHandsData, ApiError> {
  const { limit = 20, enabled = true } = options || {}

  return useQuery({
    queryKey: ['recent-hands', limit],
    queryFn: async () => {
      const response = await apiClient.get<GetRecentHandsResponse>(
        `${API_ENDPOINTS.hands.recent}?limit=${limit}`,
      )
      return {
        hands: response.data.hands,
        total: response.data.total,
        limit: response.data.limit,
      }
    },
    enabled: enabled,
    staleTime: 1 * 60 * 1000, // 1 minuto
    gcTime: 5 * 60 * 1000, // 5 minutos
  })
}

