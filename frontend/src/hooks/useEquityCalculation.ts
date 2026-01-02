/**
 * hooks/useEquityCalculation.ts
 * Hook para calcular equidad entre rangos de manos
 * POST /api/v1/equity/calculate
 */

import { useMutation, UseMutationResult } from '@tanstack/react-query'
import {
  EquityCalculationRequest,
  EquityCalculationResponse,
  PostEquityCalculateResponse,
  EquityCalculationMultiwayRequest,
  EquityCalculationMultiwayResponse,
  PostEquityCalculateMultiwayResponse,
} from '../types/api'
import { apiClient, API_ENDPOINTS, ApiError } from '../utils/api-client'

/**
 * Hook para calcular equidad heads-up entre dos rangos
 * @returns Mutation con función para ejecutar el cálculo
 */
export function useEquityCalculation(): UseMutationResult<
  EquityCalculationResponse,
  ApiError,
  EquityCalculationRequest
> {
  return useMutation({
    mutationFn: async (request: EquityCalculationRequest) => {
      const response = await apiClient.post<PostEquityCalculateResponse>(
        API_ENDPOINTS.equity.calculate,
        request,
      )
      return response.data
    },
  })
}

/**
 * Hook para calcular equidad multiway (3+ rangos)
 * @returns Mutation con función para ejecutar el cálculo
 */
export function useEquityCalculationMultiway(): UseMutationResult<
  EquityCalculationMultiwayResponse,
  ApiError,
  EquityCalculationMultiwayRequest
> {
  return useMutation({
    mutationFn: async (request: EquityCalculationMultiwayRequest) => {
      const response = await apiClient.post<PostEquityCalculateMultiwayResponse>(
        API_ENDPOINTS.equity.calculateMultiway,
        request,
      )
      return response.data
    },
  })
}

