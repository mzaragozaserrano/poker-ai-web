/**
 * pages/Dashboard.tsx
 * Dashboard principal con KPIs del Hero
 */

import { useSimplePlayerStats } from '../hooks/usePlayerStats'
import { DashboardHeader, StatCard, ProfitChart } from '../features/dashboard'

// Hero por defecto del proyecto
const HERO_NAME = 'thesmoy'

/**
 * Determina el color de un KPI basado en valores de referencia de póker 6-max
 */
const getKpiColor = (
  kpi: 'vpip' | 'pfr' | 'threeBet' | 'winrate',
  value: number,
): 'green' | 'red' | 'blue' | 'violet' | 'slate' => {
  // Rangos óptimos para 6-max cash game
  const optimalRanges = {
    vpip: { min: 20, max: 30 }, // VPIP óptimo: 20-30%
    pfr: { min: 15, max: 25 }, // PFR óptimo: 15-25%
    threeBet: { min: 5, max: 10 }, // 3Bet óptimo: 5-10%
    winrate: { min: 3, max: Infinity }, // bb/100 óptimo: > 3
  }

  const range = optimalRanges[kpi]

  if (value >= range.min && value <= range.max) {
    return 'green'
  } else if (value < range.min) {
    return 'blue'
  } else {
    return 'red'
  }
}

/**
 * Determina la tendencia de un KPI comparado con el valor óptimo
 */
const getKpiTrend = (
  kpi: 'vpip' | 'pfr' | 'threeBet' | 'winrate',
  value: number,
): 'up' | 'down' | 'neutral' => {
  const optimalRanges = {
    vpip: { min: 20, max: 30, optimal: 25 },
    pfr: { min: 15, max: 25, optimal: 20 },
    threeBet: { min: 5, max: 10, optimal: 7.5 },
    winrate: { min: 3, max: Infinity, optimal: 5 },
  }

  const range = optimalRanges[kpi]

  if (value >= range.min && value <= range.max) {
    return 'up'
  } else if (value < range.min) {
    return 'down'
  } else if (value > range.max) {
    return 'down'
  }

  return 'neutral'
}

export const Dashboard = () => {
  const { data: stats, isLoading, isError, error } = useSimplePlayerStats(HERO_NAME)

  // Estado de error
  if (isError) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-center">
          <h2 className="text-2xl font-bold text-red-500 mb-2">Error al cargar estadísticas</h2>
          <p className="text-slate-400">{error?.message || 'Error desconocido'}</p>
          <p className="text-slate-500 text-sm mt-4">
            Verifica que el backend esté corriendo en http://127.0.0.1:8000
          </p>
        </div>
      </div>
    )
  }

  return (
    <div>
      <DashboardHeader
        playerName={HERO_NAME}
        totalHands={stats?.totalHands ?? 0}
        totalProfit={stats?.totalProfit ?? 0}
        isLoading={isLoading}
      />

      {/* Grid de KPIs */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4 mb-8">
        <StatCard
          label="VPIP"
          value={stats ? `${stats.vpip.toFixed(1)}%` : '0%'}
          color={stats ? getKpiColor('vpip', stats.vpip) : 'slate'}
          trend={stats ? getKpiTrend('vpip', stats.vpip) : 'neutral'}
          isLoading={isLoading}
          helpText="Voluntarily Put In Pot - % de manos en las que pones dinero voluntariamente"
        />

        <StatCard
          label="PFR"
          value={stats ? `${stats.pfr.toFixed(1)}%` : '0%'}
          color={stats ? getKpiColor('pfr', stats.pfr) : 'slate'}
          trend={stats ? getKpiTrend('pfr', stats.pfr) : 'neutral'}
          isLoading={isLoading}
          helpText="Pre-Flop Raise - % de manos en las que subes preflop"
        />

        <StatCard
          label="3Bet"
          value={stats ? `${stats.threeBet.toFixed(1)}%` : '0%'}
          color={stats ? getKpiColor('threeBet', stats.threeBet) : 'slate'}
          trend={stats ? getKpiTrend('threeBet', stats.threeBet) : 'neutral'}
          isLoading={isLoading}
          helpText="3-Bet Percentage - % de veces que haces 3bet cuando enfrentas una subida"
        />

        <StatCard
          label="bb/100"
          value={stats ? `${stats.winrate.toFixed(1)}` : '0.0'}
          color={stats ? getKpiColor('winrate', stats.winrate) : 'slate'}
          trend={stats ? getKpiTrend('winrate', stats.winrate) : 'neutral'}
          isLoading={isLoading}
          helpText="Winrate en Big Blinds por 100 manos"
        />

        {/* WTSD - Opcional, solo se muestra si está disponible */}
        {(isLoading || (stats && stats.wtsd !== undefined)) && (
          <StatCard
            label="WTSD"
            value={stats && stats.wtsd !== undefined ? `${stats.wtsd.toFixed(1)}%` : 'N/A'}
            color="violet"
            trend="neutral"
            isLoading={isLoading}
            helpText="Went To ShowDown - % de veces que llegas al showdown"
          />
        )}
      </div>

      {/* Gráfico de evolución de beneficios */}
      <div className="mt-8">
        <ProfitChart playerName={HERO_NAME} height={400} />
      </div>

      {/* Sección adicional para futuras features */}
      <div className="mt-8">
        <h2 className="text-2xl font-bold text-white mb-4">Próximamente</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-slate-800 p-6 rounded-lg border border-slate-700 border-dashed">
            <p className="text-slate-400">Análisis de rangos 13x13</p>
          </div>
          <div className="bg-slate-800 p-6 rounded-lg border border-slate-700 border-dashed">
            <p className="text-slate-400">Detección de leaks</p>
          </div>
        </div>
      </div>
    </div>
  )
}

