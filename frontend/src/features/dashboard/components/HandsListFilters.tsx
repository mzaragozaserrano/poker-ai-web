/**
 * features/dashboard/components/HandsListFilters.tsx
 * Componente de filtros para HandsList
 */

interface HandsListFiltersProps {
  availableStakes: string[]
  selectedStakes: string[]
  onStakesChange: (stakes: string[]) => void
  selectedPositions: string[]
  onPositionsChange: (positions: string[]) => void
  resultFilter: 'all' | 'won' | 'lost'
  onResultFilterChange: (filter: 'all' | 'won' | 'lost') => void
  searchQuery: string
  onSearchQueryChange: (query: string) => void
  onResetFilters: () => void
}

const POSITIONS = ['BTN', 'SB', 'BB', 'UTG', 'MP', 'CO']

export const HandsListFilters = ({
  availableStakes,
  selectedStakes,
  onStakesChange,
  selectedPositions,
  onPositionsChange,
  resultFilter,
  onResultFilterChange,
  searchQuery,
  onSearchQueryChange,
  onResetFilters,
}: HandsListFiltersProps) => {
  const handleStakeToggle = (stake: string) => {
    if (selectedStakes.includes(stake)) {
      onStakesChange(selectedStakes.filter((s) => s !== stake))
    } else {
      onStakesChange([...selectedStakes, stake])
    }
  }

  const handlePositionToggle = (position: string) => {
    if (selectedPositions.includes(position)) {
      onPositionsChange(selectedPositions.filter((p) => p !== position))
    } else {
      onPositionsChange([...selectedPositions, position])
    }
  }

  const hasActiveFilters =
    selectedStakes.length > 0 ||
    selectedPositions.length > 0 ||
    resultFilter !== 'all' ||
    searchQuery.trim() !== ''

  return (
    <div className="mb-6 space-y-4">
      {/* Búsqueda por ID */}
      <div>
        <label className="block text-sm font-medium text-slate-300 mb-2">Buscar por ID</label>
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => onSearchQueryChange(e.target.value)}
          placeholder="Introduce el ID de la mano..."
          className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-violet-500 transition-colors"
        />
      </div>

      {/* Filtros en fila */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* Filtro por Stake */}
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">Stakes</label>
          <div className="flex flex-wrap gap-2">
            {availableStakes.length === 0 ? (
              <span className="text-sm text-slate-500">No hay stakes disponibles</span>
            ) : (
              availableStakes.map((stake) => {
                const isSelected = selectedStakes.includes(stake)
                return (
                  <button
                    key={stake}
                    onClick={() => handleStakeToggle(stake)}
                    className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                      isSelected
                        ? 'bg-violet-600 text-white'
                        : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                    }`}
                  >
                    {stake}
                  </button>
                )
              })
            )}
          </div>
        </div>

        {/* Filtro por Posición */}
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">Posición</label>
          <div className="flex flex-wrap gap-2">
            {POSITIONS.map((position) => {
              const isSelected = selectedPositions.includes(position)
              return (
                <button
                  key={position}
                  onClick={() => handlePositionToggle(position)}
                  className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                    isSelected
                      ? 'bg-blue-600 text-white'
                      : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                  }`}
                >
                  {position}
                </button>
              )
            })}
          </div>
        </div>

        {/* Filtro por Resultado */}
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">Resultado</label>
          <div className="flex flex-wrap gap-2">
            <button
              onClick={() => onResultFilterChange('all')}
              className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                resultFilter === 'all'
                  ? 'bg-slate-600 text-white'
                  : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
              }`}
            >
              Todas
            </button>
            <button
              onClick={() => onResultFilterChange('won')}
              className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                resultFilter === 'won'
                  ? 'bg-green-600 text-white'
                  : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
              }`}
            >
              Ganadas
            </button>
            <button
              onClick={() => onResultFilterChange('lost')}
              className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                resultFilter === 'lost'
                  ? 'bg-red-600 text-white'
                  : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
              }`}
            >
              Perdidas
            </button>
          </div>
        </div>
      </div>

      {/* Botón de limpiar filtros */}
      {hasActiveFilters && (
        <div className="flex justify-end">
          <button
            onClick={onResetFilters}
            className="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white rounded-lg transition-colors text-sm"
          >
            Limpiar filtros
          </button>
        </div>
      )}
    </div>
  )
}

