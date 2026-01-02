/**
 * features/dashboard/components/HandsList.tsx
 * Lista de manos recientes con filtros y navegación al Replayer
 */

import { useState, useMemo } from 'react'
import { useNavigate } from 'react-router-dom'
import { useRecentHands } from '../../../hooks/useRecentHands'
import { useAmountFormat } from '../../../hooks/useAmountFormat'
import { HandSummary } from '../../../types/api'
import { HandsListFilters } from './HandsListFilters'

interface HandsListProps {
  limit?: number
  showFilters?: boolean
  showPagination?: boolean
}

type ResultFilter = 'all' | 'won' | 'lost'

/**
 * Extrae el stake en formato legible (ej: "NL10", "NL25")
 * @param stakes - String de stakes "0.05/0.10"
 */
const formatStake = (stakes: string): string => {
  const parts = stakes.split('/')
  if (parts.length !== 2) return stakes

  const bb = parseFloat(parts[1])
  const stakeAmount = Math.round(bb * 100)
  return `NL${stakeAmount}`
}

/**
 * Formatea la fecha en formato legible
 */
const formatDate = (timestamp: string): string => {
  const date = new Date(timestamp)
  const today = new Date()
  const yesterday = new Date(today)
  yesterday.setDate(yesterday.getDate() - 1)

  const isToday = date.toDateString() === today.toDateString()
  const isYesterday = date.toDateString() === yesterday.toDateString()

  const timeStr = date.toLocaleTimeString('es-ES', { hour: '2-digit', minute: '2-digit' })

  if (isToday) return `Hoy ${timeStr}`
  if (isYesterday) return `Ayer ${timeStr}`

  return date.toLocaleDateString('es-ES', {
    day: '2-digit',
    month: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

export const HandsList = ({
  limit = 50,
  showFilters = true,
  showPagination = true,
}: HandsListProps) => {
  const navigate = useNavigate()
  const { data, isLoading, isError } = useRecentHands({ limit })
  const { formatAmount } = useAmountFormat()

  // Estado de filtros
  const [selectedStakes, setSelectedStakes] = useState<string[]>([])
  const [selectedPositions, setSelectedPositions] = useState<string[]>([])
  const [resultFilter, setResultFilter] = useState<ResultFilter>('all')
  const [searchQuery, setSearchQuery] = useState<string>('')

  // Estado de paginación
  const [currentPage, setCurrentPage] = useState(1)
  const handsPerPage = 20

  // Filtrar y procesar manos
  const filteredHands = useMemo(() => {
    if (!data?.hands) return []

    let filtered = [...data.hands]

    // Filtro por stake
    if (selectedStakes.length > 0) {
      filtered = filtered.filter((hand) => {
        const stake = formatStake(hand.stakes)
        return selectedStakes.includes(stake)
      })
    }

    // Filtro por posición
    if (selectedPositions.length > 0) {
      filtered = filtered.filter((hand) => selectedPositions.includes(hand.heroPosition))
    }

    // Filtro por resultado
    if (resultFilter === 'won') {
      filtered = filtered.filter((hand) => hand.result > 0)
    } else if (resultFilter === 'lost') {
      filtered = filtered.filter((hand) => hand.result < 0)
    }

    // Búsqueda por ID
    if (searchQuery.trim()) {
      filtered = filtered.filter((hand) =>
        hand.id.toLowerCase().includes(searchQuery.toLowerCase()),
      )
    }

    return filtered
  }, [data?.hands, selectedStakes, selectedPositions, resultFilter, searchQuery])

  // Paginación
  const totalPages = Math.ceil(filteredHands.length / handsPerPage)
  const paginatedHands = useMemo(() => {
    const startIndex = (currentPage - 1) * handsPerPage
    const endIndex = startIndex + handsPerPage
    return filteredHands.slice(startIndex, endIndex)
  }, [filteredHands, currentPage])

  // Obtener stakes únicos para filtros
  const availableStakes = useMemo(() => {
    if (!data?.hands) return []
    const stakes = data.hands.map((hand) => formatStake(hand.stakes))
    return Array.from(new Set(stakes)).sort()
  }, [data?.hands])

  // Handlers
  const handleRowClick = (handId: string) => {
    navigate(`/hands/${handId}`)
  }

  const handlePreviousPage = () => {
    setCurrentPage((prev) => Math.max(1, prev - 1))
  }

  const handleNextPage = () => {
    setCurrentPage((prev) => Math.min(totalPages, prev + 1))
  }

  const handleResetFilters = () => {
    setSelectedStakes([])
    setSelectedPositions([])
    setResultFilter('all')
    setSearchQuery('')
    setCurrentPage(1)
  }

  // Estados
  if (isLoading) {
    return (
      <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-violet-500 border-r-transparent mb-4"></div>
            <p className="text-slate-400">Cargando manos...</p>
          </div>
        </div>
      </div>
    )
  }

  if (isError) {
    return (
      <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <p className="text-red-500 font-semibold mb-2">Error al cargar las manos</p>
            <p className="text-slate-400 text-sm">
              Verifica que el backend esté corriendo en http://127.0.0.1:8000
            </p>
          </div>
        </div>
      </div>
    )
  }

  if (!data?.hands || data.hands.length === 0) {
    return (
      <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
        <h2 className="text-xl font-bold text-white mb-4">Manos Recientes</h2>
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <p className="text-slate-400">No hay manos disponibles</p>
            <p className="text-slate-500 text-sm mt-2">
              Las manos aparecerán aquí cuando el sistema detecte archivos de historial
            </p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold text-white">Manos Recientes</h2>
        <div className="text-sm text-slate-400">
          {filteredHands.length} {filteredHands.length === 1 ? 'mano' : 'manos'}
        </div>
      </div>

      {/* Filtros */}
      {showFilters && (
        <HandsListFilters
          availableStakes={availableStakes}
          selectedStakes={selectedStakes}
          onStakesChange={setSelectedStakes}
          selectedPositions={selectedPositions}
          onPositionsChange={setSelectedPositions}
          resultFilter={resultFilter}
          onResultFilterChange={setResultFilter}
          searchQuery={searchQuery}
          onSearchQueryChange={setSearchQuery}
          onResetFilters={handleResetFilters}
        />
      )}

      {/* Tabla */}
      {filteredHands.length === 0 ? (
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <p className="text-slate-400">No hay manos que coincidan con los filtros</p>
            <button
              onClick={handleResetFilters}
              className="mt-4 px-4 py-2 bg-violet-600 hover:bg-violet-700 text-white rounded-lg transition-colors text-sm"
            >
              Limpiar filtros
            </button>
          </div>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-slate-700">
                <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300">Fecha</th>
                <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300">Stake</th>
                <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300">
                  Posición
                </th>
                <th className="text-right py-3 px-4 text-sm font-semibold text-slate-300">
                  Resultado
                </th>
                <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300">ID</th>
              </tr>
            </thead>
            <tbody>
              {paginatedHands.map((hand) => {
                const isWin = hand.result > 0
                const isLoss = hand.result < 0
                const resultColor = isWin
                  ? 'text-green-400'
                  : isLoss
                    ? 'text-red-400'
                    : 'text-slate-400'

                return (
                  <tr
                    key={hand.id}
                    onClick={() => handleRowClick(hand.id)}
                    className="border-b border-slate-700/50 hover:bg-slate-700/30 cursor-pointer transition-colors"
                  >
                    <td className="py-3 px-4 text-sm text-slate-300">
                      {formatDate(hand.timestamp)}
                    </td>
                    <td className="py-3 px-4 text-sm">
                      <span className="inline-flex items-center px-2 py-1 rounded bg-slate-700 text-violet-400 font-medium">
                        {formatStake(hand.stakes)}
                      </span>
                    </td>
                    <td className="py-3 px-4 text-sm">
                      <span className="inline-flex items-center px-2 py-1 rounded bg-slate-900 text-blue-400 font-medium">
                        {hand.heroPosition}
                      </span>
                    </td>
                    <td className={`py-3 px-4 text-sm font-semibold text-right ${resultColor}`}>
                      {isWin && '+'}
                      {formatAmount(hand.result)}
                    </td>
                    <td className="py-3 px-4 text-sm text-slate-500 font-mono text-xs truncate max-w-[120px]">
                      {hand.id}
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
      )}

      {/* Paginación */}
      {showPagination && filteredHands.length > handsPerPage && (
        <div className="flex items-center justify-between mt-6 pt-4 border-t border-slate-700">
          <div className="text-sm text-slate-400">
            Mostrando {(currentPage - 1) * handsPerPage + 1} -{' '}
            {Math.min(currentPage * handsPerPage, filteredHands.length)} de {filteredHands.length}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handlePreviousPage}
              disabled={currentPage === 1}
              className="px-4 py-2 bg-slate-700 hover:bg-slate-600 disabled:bg-slate-800 disabled:text-slate-600 text-white rounded-lg transition-colors text-sm disabled:cursor-not-allowed"
            >
              Anterior
            </button>
            <span className="text-sm text-slate-400">
              Página {currentPage} de {totalPages}
            </span>
            <button
              onClick={handleNextPage}
              disabled={currentPage === totalPages}
              className="px-4 py-2 bg-slate-700 hover:bg-slate-600 disabled:bg-slate-800 disabled:text-slate-600 text-white rounded-lg transition-colors text-sm disabled:cursor-not-allowed"
            >
              Siguiente
            </button>
          </div>
        </div>
      )}
    </div>
  )
}

