import { useState } from 'react'
import { RangeMatrix, RangePresets } from '../features/stats/components'
import type { RangePreset, RangeData } from '../types/ranges'

export const Stats = () => {
  const [selectedRange, setSelectedRange] = useState<RangeData | undefined>(undefined)
  const [selectedPresetId, setSelectedPresetId] = useState<string | undefined>(undefined)
  const [selectedHands, setSelectedHands] = useState<string[]>([])

  const handlePresetSelect = (preset: RangePreset) => {
    setSelectedRange(preset.range)
    setSelectedPresetId(preset.id)
  }

  const handleCellClick = (hand: string) => {
    console.log('Cell clicked:', hand)
  }

  const handleSelectionChange = (hands: string[]) => {
    setSelectedHands(hands)
  }

  return (
    <div>
      <h1 className="text-4xl font-bold text-white mb-6">Estadísticas y Análisis de Rangos</h1>

      {/* Sección de Matriz de Rangos */}
      <div className="grid grid-cols-1 xl:grid-cols-4 gap-6 mb-8">
        {/* Sidebar de Presets */}
        <div className="xl:col-span-1">
          <div className="bg-slate-800 rounded-lg border border-slate-700 p-6 sticky top-6">
            <RangePresets
              onPresetSelect={handlePresetSelect}
              selectedPresetId={selectedPresetId}
            />
          </div>
        </div>

        {/* Matriz de Rangos */}
        <div className="xl:col-span-3">
          <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-xl font-bold text-white">
                Matriz de Rangos 13x13
              </h2>
              {selectedHands.length > 0 && (
                <span className="text-sm text-slate-400">
                  {selectedHands.length} mano{selectedHands.length !== 1 ? 's' : ''} seleccionada{selectedHands.length !== 1 ? 's' : ''}
                </span>
              )}
            </div>

            {/* Leyenda de colores */}
            <div className="mb-4 flex gap-4 text-xs">
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-red-500 rounded" />
                <span className="text-slate-400">Raise</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-blue-500 rounded" />
                <span className="text-slate-400">Call</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-violet-500 rounded" />
                <span className="text-slate-400">Marginal</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 bg-amber-500 rounded" />
                <span className="text-slate-400">All-In</span>
              </div>
            </div>

            {/* Matriz */}
            <div className="flex justify-center">
              <RangeMatrix
                range={selectedRange}
                onCellClick={handleCellClick}
                onSelectionChange={handleSelectionChange}
              />
            </div>

            {/* Información adicional */}
            <div className="mt-6 p-4 bg-slate-900 rounded-lg">
              <h3 className="text-sm font-semibold text-white mb-2">Instrucciones</h3>
              <ul className="text-xs text-slate-400 space-y-1">
                <li>• Selecciona un preset del sidebar para cargar un rango predefinido</li>
                <li>• Haz clic en una celda para seleccionarla individualmente</li>
                <li>• Arrastra para seleccionar múltiples celdas (drag-to-select)</li>
                <li>• Pasa el cursor sobre una celda para ver el desglose de acciones</li>
                <li>• La intensidad del color representa la frecuencia (0% = transparente, 100% = opaco)</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      {/* Sección de Estadísticas (existente) */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h2 className="text-xl font-bold text-white mb-4">Evolución de Ganancia</h2>
          <div className="h-64 bg-slate-900 rounded flex items-center justify-center">
            <p className="text-slate-500">Gráfico con Recharts</p>
          </div>
        </div>
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h2 className="text-xl font-bold text-white mb-4">Resumen</h2>
          <div className="space-y-4">
            <div>
              <p className="text-slate-400 text-sm">Hands/Hour</p>
              <p className="text-2xl font-bold text-white">35.2</p>
            </div>
            <div>
              <p className="text-slate-400 text-sm">Win Rate (bb/100)</p>
              <p className="text-2xl font-bold text-green-500">8.3</p>
            </div>
            <div>
              <p className="text-slate-400 text-sm">ROI</p>
              <p className="text-2xl font-bold text-blue-500">+15.2%</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

