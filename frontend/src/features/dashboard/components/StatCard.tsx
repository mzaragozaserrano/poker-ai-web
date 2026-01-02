/**
 * features/dashboard/components/StatCard.tsx
 * Componente reutilizable para mostrar un KPI individual con indicador de tendencia
 */

import { type ReactNode } from 'react'

export interface StatCardProps {
  label: string
  value: string | number
  trend?: 'up' | 'down' | 'neutral'
  color?: 'green' | 'red' | 'blue' | 'violet' | 'slate'
  icon?: ReactNode
  isLoading?: boolean
  helpText?: string
}

const colorClasses = {
  green: 'text-green-500',
  red: 'text-red-500',
  blue: 'text-blue-500',
  violet: 'text-violet-500',
  slate: 'text-slate-400',
}

const trendClasses = {
  up: 'text-green-500',
  down: 'text-red-500',
  neutral: 'text-slate-500',
}

const trendIcons = {
  up: '↑',
  down: '↓',
  neutral: '→',
}

/**
 * Componente StatCard para mostrar un KPI individual
 * @param label - Etiqueta del KPI (ej: "VPIP", "PFR")
 * @param value - Valor a mostrar (puede incluir % o unidades)
 * @param trend - Tendencia: up (verde), down (rojo), neutral (gris)
 * @param color - Color del valor principal
 * @param icon - Icono opcional a mostrar junto al label
 * @param isLoading - Si está cargando, muestra skeleton
 * @param helpText - Texto de ayuda opcional (tooltip futuro)
 */
export const StatCard = ({
  label,
  value,
  trend = 'neutral',
  color = 'slate',
  icon,
  isLoading = false,
  helpText,
}: StatCardProps) => {
  if (isLoading) {
    return (
      <div className="bg-slate-800 p-6 rounded-lg border border-slate-700 animate-pulse">
        <div className="h-4 bg-slate-700 rounded w-20 mb-3"></div>
        <div className="h-8 bg-slate-700 rounded w-24"></div>
      </div>
    )
  }

  return (
    <div
      className="bg-slate-800 p-6 rounded-lg border border-slate-700 hover:border-slate-600 transition-colors"
      title={helpText}
    >
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-slate-400 text-sm font-semibold uppercase tracking-wide flex items-center gap-2">
          {icon && <span className="text-slate-500">{icon}</span>}
          {label}
        </h3>
        {trend !== 'neutral' && (
          <span className={`text-lg font-bold ${trendClasses[trend]}`}>{trendIcons[trend]}</span>
        )}
      </div>
      <p className={`text-3xl font-bold ${colorClasses[color]}`}>{value}</p>
    </div>
  )
}

