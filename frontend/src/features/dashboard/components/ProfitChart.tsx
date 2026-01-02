/**
 * features/dashboard/components/ProfitChart.tsx
 * Gráfico de evolución de beneficios con Recharts
 * Muestra Net Won (beneficio real) vs All-in EV (beneficio esperado)
 */

import { useMemo } from 'react'
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  Brush,
  TooltipProps,
} from 'recharts'
import { NameType, ValueType } from 'recharts/types/component/DefaultTooltipContent'
import { useProfitHistory } from '../../../hooks/useProfitHistory'

export interface ProfitChartProps {
  playerName: string
  startDate?: string
  endDate?: string
  height?: number
}

/**
 * Formatea una cantidad en centavos a euros para el gráfico de beneficios
 * El gráfico siempre muestra en EUR ya que es un gráfico de bankroll
 */
const formatProfitAmount = (amountCents: number): string => {
  const euros = amountCents / 100
  return `€${euros.toFixed(2)}`
}

/**
 * Tooltip personalizado con formato de moneda
 */
const CustomTooltip = ({
  active,
  payload,
  label,
}: TooltipProps<ValueType, NameType>) => {

  if (!active || !payload || !payload.length) {
    return null
  }

  const netWon = payload.find((p) => p.dataKey === 'cumulativeNetWon')
  const ev = payload.find((p) => p.dataKey === 'cumulativeEV')

  return (
    <div className="bg-slate-900 border border-slate-700 rounded-lg p-3 shadow-xl">
      <p className="text-slate-300 text-sm font-medium mb-2">{label}</p>
      
      {netWon && (
        <div className="flex items-center justify-between gap-4 mb-1">
          <span className="text-xs text-slate-400 flex items-center gap-2">
            <span
              className="w-3 h-0.5 rounded"
              style={{ backgroundColor: netWon.color }}
            />
            Net Won:
          </span>
          <span className="text-sm font-semibold text-violet-400">
            {formatProfitAmount(Number(netWon.value))}
          </span>
        </div>
      )}
      
      {ev && (
        <div className="flex items-center justify-between gap-4">
          <span className="text-xs text-slate-400 flex items-center gap-2">
            <span
              className="w-3 h-0.5 rounded"
              style={{ backgroundColor: ev.color }}
            />
            All-in EV:
          </span>
          <span className="text-sm font-semibold text-slate-400">
            {formatProfitAmount(Number(ev.value))}
          </span>
        </div>
      )}

      {/* Diferencia entre Net Won y EV */}
      {netWon && ev && (
        <div className="mt-2 pt-2 border-t border-slate-700">
          <div className="flex items-center justify-between gap-4">
            <span className="text-xs text-slate-500">Diferencia:</span>
            <span
              className={`text-sm font-medium ${
                Number(netWon.value) > Number(ev.value)
                  ? 'text-green-400'
                  : Number(netWon.value) < Number(ev.value)
                    ? 'text-red-400'
                    : 'text-slate-400'
              }`}
            >
              {formatProfitAmount(Number(netWon.value) - Number(ev.value))}
            </span>
          </div>
        </div>
      )}
    </div>
  )
}

/**
 * Formatea las fechas en el eje X para mayor legibilidad
 */
const formatXAxis = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('es-ES', { month: 'short', day: 'numeric' })
}

/**
 * Formatea los valores del eje Y (convierte centavos a euros)
 */
const formatYAxis = (value: number) => {
  const euros = value / 100
  return `€${euros >= 1000 ? (euros / 1000).toFixed(1) + 'k' : euros.toFixed(0)}`
}

export const ProfitChart = ({
  playerName,
  startDate,
  endDate,
  height = 400,
}: ProfitChartProps) => {
  const { data, isLoading, isError } = useProfitHistory(playerName, startDate, endDate)

  // Transformar datos para Recharts
  const chartData = useMemo(() => {
    if (!data) return []
    
    return data.dataPoints.map((point) => ({
      date: point.date,
      cumulativeNetWon: point.cumulativeNetWon,
      cumulativeEV: point.cumulativeEV,
      hands: point.hands,
    }))
  }, [data])

  // Estado de carga
  if (isLoading) {
    return (
      <div
        className="bg-slate-800 rounded-lg border border-slate-700 flex items-center justify-center"
        style={{ height }}
      >
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-violet-500 mx-auto mb-4" />
          <p className="text-slate-400">Cargando datos de beneficios...</p>
        </div>
      </div>
    )
  }

  // Estado de error
  if (isError || !data) {
    return (
      <div
        className="bg-slate-800 rounded-lg border border-slate-700 flex items-center justify-center"
        style={{ height }}
      >
        <div className="text-center">
          <p className="text-red-400 font-semibold mb-2">Error al cargar datos</p>
          <p className="text-slate-500 text-sm">
            No se pudieron obtener los datos de beneficios
          </p>
        </div>
      </div>
    )
  }

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
      {/* Header del gráfico */}
      <div className="mb-6">
        <h3 className="text-xl font-bold text-white mb-2">Evolución de Beneficios</h3>
        <p className="text-sm text-slate-400">
          Comparación entre beneficio real (Net Won) y beneficio esperado (All-in EV)
        </p>
        
        {/* Resumen de totales */}
        <div className="mt-3 flex gap-6">
          <div>
            <span className="text-xs text-slate-500 uppercase tracking-wide">
              Total Net Won:
            </span>
            <span className="ml-2 text-lg font-bold text-violet-400">
              €{(data.totalNetWon / 100).toFixed(2)}
            </span>
          </div>
          <div>
            <span className="text-xs text-slate-500 uppercase tracking-wide">
              Total EV:
            </span>
            <span className="ml-2 text-lg font-bold text-slate-400">
              €{(data.totalEV / 100).toFixed(2)}
            </span>
          </div>
          <div>
            <span className="text-xs text-slate-500 uppercase tracking-wide">
              Diferencia:
            </span>
            <span
              className={`ml-2 text-lg font-bold ${
                data.totalNetWon > data.totalEV
                  ? 'text-green-400'
                  : data.totalNetWon < data.totalEV
                    ? 'text-red-400'
                    : 'text-slate-400'
              }`}
            >
              €{((data.totalNetWon - data.totalEV) / 100).toFixed(2)}
            </span>
          </div>
        </div>
      </div>

      {/* Gráfico */}
      <ResponsiveContainer width="100%" height={height}>
        <LineChart
          data={chartData}
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
        >
          {/* Grid con estilo Dark Mode */}
          <CartesianGrid strokeDasharray="3 3" stroke="#334155" opacity={0.3} />
          
          {/* Eje X (fechas) */}
          <XAxis
            dataKey="date"
            tickFormatter={formatXAxis}
            stroke="#64748B"
            style={{ fontSize: '12px' }}
          />
          
          {/* Eje Y (beneficio en €) */}
          <YAxis
            tickFormatter={formatYAxis}
            stroke="#64748B"
            style={{ fontSize: '12px' }}
          />
          
          {/* Tooltip personalizado */}
          <Tooltip content={<CustomTooltip />} />
          
          {/* Leyenda */}
          <Legend
            verticalAlign="top"
            height={36}
            iconType="line"
            wrapperStyle={{
              paddingBottom: '20px',
              fontSize: '14px',
            }}
          />
          
          {/* Línea Net Won (Beneficio Real) - Accent Violet */}
          <Line
            type="monotone"
            dataKey="cumulativeNetWon"
            stroke="#8B5CF6"
            strokeWidth={2.5}
            dot={false}
            name="Net Won (Real)"
            activeDot={{ r: 6, fill: '#8B5CF6' }}
          />
          
          {/* Línea All-in EV (Beneficio Esperado) - Slate 400 */}
          <Line
            type="monotone"
            dataKey="cumulativeEV"
            stroke="#94A3B8"
            strokeWidth={2.5}
            dot={false}
            name="All-in EV (Esperado)"
            strokeDasharray="5 5"
            activeDot={{ r: 6, fill: '#94A3B8' }}
          />
          
          {/* Brush para zoom/pan en el rango de fechas */}
          <Brush
            dataKey="date"
            height={30}
            stroke="#8B5CF6"
            fill="#1E293B"
            tickFormatter={formatXAxis}
          />
        </LineChart>
      </ResponsiveContainer>

      {/* Footer con información adicional */}
      <div className="mt-4 pt-4 border-t border-slate-700">
        <p className="text-xs text-slate-500">
          <span className="font-semibold">Nota:</span> El Net Won representa tus ganancias
          reales, mientras que el All-in EV muestra cuánto deberías haber ganado basándote en
          tus decisiones de all-in (elimina el factor suerte a largo plazo).
        </p>
      </div>
    </div>
  )
}

