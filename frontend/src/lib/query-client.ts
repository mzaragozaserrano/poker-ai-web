/**
 * lib/query-client.ts
 * Configuración centralizada de React Query
 * Optimizado para análisis de poker con cache strategy agresivo
 */

import { QueryClient } from '@tanstack/react-query'

/**
 * Crear instancia única de QueryClient con defaults optimizados
 * para el contexto de análisis de poker (6-max NLHE)
 */
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Cache agresivo: Las estadísticas de jugadores son relativamente estables
      staleTime: 5 * 60 * 1000, // 5 minutos
      gcTime: 10 * 60 * 1000, // 10 minutos (antiguamente cacheTime)

      // Reintentos automáticos en caso de fallo
      retry: 2,
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),

      // No hacer refetch automático al cambiar focus (lo haremos manualmente)
      refetchOnWindowFocus: false,
      refetchOnMount: true,
      refetchOnReconnect: true,

      // Networking
      networkMode: 'online',
    },
    mutations: {
      // Reintentos para mutaciones (POST/PUT/DELETE)
      retry: 1,
      retryDelay: 1000,

      // No refetchar automáticamente after mutaciones
      networkMode: 'online',
    },
  },
})

/**
 * Presets de configuración para diferentes tipos de queries
 */

/**
 * Para queries que se actualizan frecuentemente (e.g., nueva mano detectada)
 */
export const frequentUpdateConfig = {
  staleTime: 10 * 1000, // 10 segundos
  gcTime: 1 * 60 * 1000, // 1 minuto
}

/**
 * Para queries que cambian raramente (e.g., estadísticas de sesión)
 */
export const rareUpdateConfig = {
  staleTime: 30 * 60 * 1000, // 30 minutos
  gcTime: 60 * 60 * 1000, // 1 hora
}

/**
 * Para queries que no deben cacharse (e.g., cálculos dinámicos de equidad)
 */
export const noCacheConfig = {
  staleTime: 0,
  gcTime: 0,
}

