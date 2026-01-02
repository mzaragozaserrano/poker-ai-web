/**
 * hooks/useProfitHistory.ts
 * Hook para obtener historial de beneficios temporales (Net Won y All-in EV)
 * 
 * TODO: Conectar con endpoint real cuando esté disponible en backend
 * Endpoint futuro: GET /api/v1/stats/player/{name}/profit-history
 */

import { useQuery } from '@tanstack/react-query'

export interface ProfitDataPoint {
  date: string // ISO-8601
  netWon: number // En centavos
  allInEV: number // En centavos (Expected Value ajustado por all-ins)
  cumulativeNetWon: number // Beneficio acumulado real
  cumulativeEV: number // Beneficio acumulado esperado
  hands: number // Número de manos en ese punto
}

export interface ProfitHistoryResponse {
  playerName: string
  dataPoints: ProfitDataPoint[]
  totalNetWon: number
  totalEV: number
  startDate: string
  endDate: string
}

/**
 * Hook para obtener el historial de beneficios de un jugador
 * @param playerName - Nombre del jugador (Hero)
 * @param startDate - Fecha de inicio (opcional)
 * @param endDate - Fecha de fin (opcional)
 */
export function useProfitHistory(
  playerName: string,
  startDate?: string,
  endDate?: string,
) {
  return useQuery({
    queryKey: ['profitHistory', playerName, startDate, endDate],
    queryFn: async (): Promise<ProfitHistoryResponse> => {
      // TODO: Reemplazar con llamada real a la API
      // const response = await apiClient.get<GetProfitHistoryResponse>(
      //   API_ENDPOINTS.stats.profitHistory(playerName),
      //   { params: { startDate, endDate } }
      // )
      // return response.data

      // MOCK DATA para desarrollo (últimos 30 días)
      const mockData = generateMockProfitData(playerName, 30)
      
      // Simular latencia de red
      await new Promise((resolve) => setTimeout(resolve, 500))
      
      return mockData
    },
    staleTime: 5 * 60 * 1000, // 5 minutos
    gcTime: 30 * 60 * 1000, // 30 minutos
  })
}

/**
 * Genera datos de ejemplo para desarrollo
 * Simula un bankroll con varianza realista
 */
function generateMockProfitData(playerName: string, days: number): ProfitHistoryResponse {
  const dataPoints: ProfitDataPoint[] = []
  const now = new Date()
  
  let cumulativeNetWon = 0
  let cumulativeEV = 0
  
  // Simular datos diarios con varianza
  for (let i = days; i >= 0; i--) {
    const date = new Date(now)
    date.setDate(date.getDate() - i)
    
    // Simular beneficio diario con varianza (entre -50€ y +120€)
    const dailyVariance = (Math.random() - 0.3) * 17000 // Más probabilidad de ganar
    const netWonDaily = Math.round(dailyVariance)
    
    // EV siempre un poco más estable que net won (menor varianza)
    const evVariance = dailyVariance * 0.85 + (Math.random() - 0.5) * 3000
    const evDaily = Math.round(evVariance)
    
    cumulativeNetWon += netWonDaily
    cumulativeEV += evDaily
    
    dataPoints.push({
      date: date.toISOString().split('T')[0], // YYYY-MM-DD
      netWon: netWonDaily,
      allInEV: evDaily,
      cumulativeNetWon,
      cumulativeEV,
      hands: (i + 1) * 45, // ~45 manos por día
    })
  }
  
  return {
    playerName,
    dataPoints,
    totalNetWon: cumulativeNetWon,
    totalEV: cumulativeEV,
    startDate: dataPoints[0].date,
    endDate: dataPoints[dataPoints.length - 1].date,
  }
}

