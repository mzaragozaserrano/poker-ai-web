/**
 * hooks/useHand.ts
 * Hook para obtener los detalles de una mano específica
 * GET /api/v1/hands/{hand_id}
 */

import { useQuery, UseQueryResult } from '@tanstack/react-query'
import { Hand, GetHandResponse } from '../types/api'
import { apiClient, API_ENDPOINTS, ApiError } from '../utils/api-client'

interface UseHandOptions {
  handId: string
  enabled?: boolean
}

/**
 * Hook para obtener detalles de una mano específica
 * @param handId - ID de la mano a obtener
 * @param enabled - Si está habilitada la query (default: true)
 * @returns Query result con detalles de la mano
 */
export function useHand(
  options: UseHandOptions,
): UseQueryResult<Hand, ApiError> {
  const { handId, enabled = true } = options

  return useQuery({
    queryKey: ['hand', handId],
    queryFn: async () => {
      const response = await apiClient.get<GetHandResponse>(
        API_ENDPOINTS.hands.byId(handId),
      )
      return response.data
    },
    enabled: enabled && Boolean(handId),
    staleTime: Infinity, // Las manos son inmutables una vez creadas
    gcTime: 60 * 60 * 1000, // 1 hora
  })
}

