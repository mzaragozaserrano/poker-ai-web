import React from 'react'
import type { ReplayerActionStep } from '../../../types/poker'

interface ReplayerTimelineProps {
  actions: ReplayerActionStep[]
  currentActionIndex: number
  onActionClick: (index: number) => void
}

/**
 * Componente de timeline visual para el reproductor de manos
 * Muestra todas las acciones de la mano y permite saltar a una acción específica
 */
export const ReplayerTimeline: React.FC<ReplayerTimelineProps> = ({
  actions,
  currentActionIndex,
  onActionClick,
}) => {
  if (actions.length === 0) {
    return (
      <div className="bg-slate-800 rounded-lg border border-slate-700 p-4 text-center text-slate-400">
        <p>No hay acciones disponibles</p>
      </div>
    )
  }

  // Agrupar acciones por calle
  const actionsByStreet = actions.reduce(
    (acc, action) => {
      if (!acc[action.street]) {
        acc[action.street] = []
      }
      acc[action.street].push(action)
      return acc
    },
    {} as Record<string, ReplayerActionStep[]>
  )

  const streets: Array<'preflop' | 'flop' | 'turn' | 'river'> = [
    'preflop',
    'flop',
    'turn',
    'river',
  ]
  const streetLabels = {
    preflop: 'Preflop',
    flop: 'Flop',
    turn: 'Turn',
    river: 'River',
  }

  // Determinar color de acción
  const getActionColor = (action: string) => {
    switch (action.toLowerCase()) {
      case 'fold':
        return 'bg-slate-600 text-slate-300'
      case 'check':
        return 'bg-blue-600/20 text-blue-300'
      case 'call':
        return 'bg-blue-600/40 text-blue-300'
      case 'bet':
        return 'bg-amber-600/40 text-amber-300'
      case 'raise':
        return 'bg-red-600/40 text-red-300'
      case 'all-in':
        return 'bg-red-600/70 text-red-100'
      default:
        return 'bg-slate-700 text-slate-300'
    }
  }

  // Traducir acciones al español
  const translateAction = (action: string) => {
    const translations: Record<string, string> = {
      fold: 'Fold',
      check: 'Check',
      call: 'Call',
      bet: 'Bet',
      raise: 'Raise',
      'all-in': 'All-in',
    }
    return translations[action.toLowerCase()] || action
  }

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 p-4 space-y-4">
      <h3 className="text-sm font-semibold text-slate-300">Timeline de Acciones</h3>

      <div className="space-y-3 max-h-64 overflow-y-auto">
        {streets.map((street) => {
          const streetActions = actionsByStreet[street]
          if (!streetActions || streetActions.length === 0) return null

          return (
            <div key={street}>
              {/* Street header */}
              <div className="text-xs font-semibold text-violet-400 mb-2 uppercase">
                {streetLabels[street]}
              </div>

              {/* Actions in this street */}
              <div className="flex flex-wrap gap-1.5">
                {streetActions.map((action: ReplayerActionStep) => {
                  const isCurrentAction = action.index === currentActionIndex
                  const isPastAction = action.index < currentActionIndex

                  return (
                    <button
                      key={action.index}
                      onClick={() => onActionClick(action.index)}
                      className={`
                        px-2.5 py-1.5 rounded text-xs font-medium transition-all cursor-pointer
                        ${
                          isCurrentAction
                            ? 'ring-2 ring-violet-500 bg-violet-600 text-white scale-105'
                            : isPastAction
                              ? 'opacity-60 hover:opacity-80'
                              : 'opacity-40 hover:opacity-60'
                        }
                        ${getActionColor(action.action)}
                      `}
                      title={`${action.player}: ${action.description}`}
                    >
                      <span className="font-mono text-xs">{action.index + 1}</span>
                      <span className="mx-1">{translateAction(action.action)}</span>
                      {action.amount > 0 && (
                        <span className="text-xs opacity-80">{action.amount}</span>
                      )}
                    </button>
                  )
                })}
              </div>
            </div>
          )
        })}
      </div>

      {/* Leyenda */}
      <div className="text-xs text-slate-400 pt-3 border-t border-slate-700 space-y-1">
        <div className="flex gap-2 flex-wrap">
          <span className="inline-flex items-center gap-1">
            <span className="w-3 h-3 rounded bg-slate-600" />
            <span>Fold</span>
          </span>
          <span className="inline-flex items-center gap-1">
            <span className="w-3 h-3 rounded bg-blue-600/40" />
            <span>Call/Check</span>
          </span>
          <span className="inline-flex items-center gap-1">
            <span className="w-3 h-3 rounded bg-amber-600/40" />
            <span>Bet</span>
          </span>
          <span className="inline-flex items-center gap-1">
            <span className="w-3 h-3 rounded bg-red-600/40" />
            <span>Raise</span>
          </span>
        </div>
      </div>
    </div>
  )
}

