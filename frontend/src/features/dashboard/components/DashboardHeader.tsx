/**
 * features/dashboard/components/DashboardHeader.tsx
 * Header del dashboard con resumen general del Hero
 */

export interface DashboardHeaderProps {
  playerName: string
  totalHands: number
  totalProfit: number // En centavos
  isLoading?: boolean
}

/**
 * Formatea un profit total en EUR
 * @param amountCents - Cantidad en centavos
 * @returns String formateado (ej: "+€245.00")
 */
const formatProfitEUR = (amountCents: number): string => {
  const eur = amountCents / 100
  return `€${Math.abs(eur).toFixed(2)}`
}

/**
 * Header del Dashboard con resumen del Hero
 * @param playerName - Nombre del jugador (Hero)
 * @param totalHands - Total de manos jugadas
 * @param totalProfit - Ganancia total en centavos
 * @param isLoading - Estado de carga
 */
export const DashboardHeader = ({
  playerName,
  totalHands,
  totalProfit,
  isLoading = false,
}: DashboardHeaderProps) => {

  if (isLoading) {
    return (
      <div className="mb-8 animate-pulse">
        <div className="h-10 bg-slate-700 rounded w-64 mb-4"></div>
        <div className="flex gap-8">
          <div>
            <div className="h-4 bg-slate-700 rounded w-20 mb-2"></div>
            <div className="h-6 bg-slate-700 rounded w-16"></div>
          </div>
          <div>
            <div className="h-4 bg-slate-700 rounded w-20 mb-2"></div>
            <div className="h-6 bg-slate-700 rounded w-24"></div>
          </div>
        </div>
      </div>
    )
  }

  const profitColor = totalProfit >= 0 ? 'text-green-500' : 'text-red-500'
  const profitSign = totalProfit >= 0 ? '+' : ''
  const profitFormatted = formatProfitEUR(totalProfit)

  return (
    <div className="mb-8">
      <h1 className="text-4xl font-bold text-white mb-4">
        Dashboard - <span className="text-violet-500">{playerName}</span>
      </h1>
      <div className="flex gap-8 text-slate-300">
        <div>
          <p className="text-sm text-slate-400 uppercase tracking-wide mb-1">Manos Jugadas</p>
          <p className="text-2xl font-bold text-white">{totalHands.toLocaleString()}</p>
        </div>
        <div>
          <p className="text-sm text-slate-400 uppercase tracking-wide mb-1">Ganancia Total</p>
          <p className={`text-2xl font-bold ${profitColor}`}>
            {profitSign}
            {profitFormatted}
          </p>
        </div>
      </div>
    </div>
  )
}

