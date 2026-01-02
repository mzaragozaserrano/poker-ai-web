/**
 * hooks/usePlayerStats.ts
 * Hook para obtener estadísticas de un jugador específico
 * GET /api/v1/stats/player/{name}
 */

import { useQuery, UseQueryResult } from '@tanstack/react-query'
import { PlayerStats, GetPlayerStatsResponse } from '../types/api'
import { apiClient, API_ENDPOINTS, ApiError } from '../utils/api-client'

interface UsePlayerStatsOptions {
  playerName: string
  enabled?: boolean
}

/**
 * Hook para obtener estadísticas de un jugador
 * @param playerName - Nombre del jugador (e.g., 'thesmoy')
 * @param enabled - Si está habilitada la query (default: true)
 * @returns Query result con estadísticas del jugador
 */
export function usePlayerStats(
  options: UsePlayerStatsOptions,
): UseQueryResult<PlayerStats, ApiError> {
  const { playerName, enabled = true } = options

  return useQuery({
    queryKey: ['player-stats', playerName],
    queryFn: async () => {
      const response = await apiClient.get<GetPlayerStatsResponse>(
        API_ENDPOINTS.stats.player(playerName),
      )
      return response.data
    },
    enabled: enabled && Boolean(playerName),
    staleTime: 5 * 60 * 1000, // 5 minutos
    gcTime: 10 * 60 * 1000, // 10 minutos
  })
}

/**
 * Hook más simple si solo necesitas las estadísticas sin opciones avanzadas
 */
export function useSimplePlayerStats(playerName: string): UseQueryResult<PlayerStats, ApiError> {
  return usePlayerStats({ playerName })
}

