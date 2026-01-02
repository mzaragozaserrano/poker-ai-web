/**
 * pages/ApiClientDemo.tsx
 * Página de demostración del cliente API y hooks de React Query
 * Muestra cómo usar usePlayerStats, useRecentHands, useHand y useEquityCalculation
 */

import { useState } from 'react'
import {
  usePlayerStats,
  useRecentHands,
  useHand,
  useEquityCalculation,
} from '../hooks'
import { Card, Badge, Button, Input } from '../components'
import { ApiError } from '../utils/api-client'

export function ApiClientDemo() {
  const [selectedPlayerName, setSelectedPlayerName] = useState('thesmoy')
  const [selectedHandId, setSelectedHandId] = useState<string | null>(null)
  const [equityForm, setEquityForm] = useState({
    heroRange: 'AA,KK,AKs',
    villainRange: 'QQ+,AJs',
  })

  // ========== Queries ==========
  const playerStats = usePlayerStats({ playerName: selectedPlayerName })
  const recentHands = useRecentHands({ limit: 10 })
  const hand = useHand({ handId: selectedHandId || '', enabled: Boolean(selectedHandId) })

  // ========== Mutations ==========
  const equityCalc = useEquityCalculation()

  const handleCalculateEquity = () => {
    equityCalc.mutate({
      heroRange: equityForm.heroRange,
      villainRange: equityForm.villainRange,
      runouts: 1000,
    })
  }

  return (
    <div className="min-h-screen bg-slate-950 p-6">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Encabezado */}
        <div>
          <h1 className="text-3xl font-bold text-white">API Client Demo</h1>
          <p className="text-slate-400 mt-1">
            Demostración de React Query con endpoints de la API
          </p>
        </div>

        {/* Grid de contenido */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* ========== SECCIÓN 1: Player Stats ========== */}
          <Card className="col-span-1">
            <div className="p-6">
              <h2 className="text-xl font-semibold text-white mb-4">Estadísticas de Jugador</h2>

              <div className="space-y-4">
                <Input
                  label="Nombre de Jugador"
                  value={selectedPlayerName}
                  onChange={(e) => setSelectedPlayerName(e.target.value)}
                  placeholder="ej: thesmoy"
                />

                {playerStats.isPending && <p className="text-slate-400">Cargando...</p>}
                {playerStats.isError && (
                  <div className="bg-red-900/20 border border-red-600 p-3 rounded">
                    <p className="text-red-400 text-sm">
                      {playerStats.error instanceof ApiError
                        ? playerStats.error.detail || playerStats.error.message
                        : 'Error desconocido'}
                    </p>
                  </div>
                )}

                {playerStats.data && (
                  <div className="space-y-3">
                    <div className="flex justify-between items-center">
                      <span className="text-slate-400">Nombre:</span>
                      <span className="text-white font-medium">{playerStats.data.name}</span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-slate-400">Total Manos:</span>
                      <Badge variant="default">{playerStats.data.totalHands}</Badge>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-slate-400">VPIP:</span>
                      <span className="text-blue-400">{playerStats.data.vpip.toFixed(1)}%</span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-slate-400">PFR:</span>
                      <span className="text-blue-400">{playerStats.data.pfr.toFixed(1)}%</span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-slate-400">Winrate:</span>
                      <span className="text-green-400 font-medium">
                        {playerStats.data.winrate.toFixed(2)} BB/100
                      </span>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </Card>

          {/* ========== SECCIÓN 2: Recent Hands ========== */}
          <Card className="col-span-1">
            <div className="p-6">
              <h2 className="text-xl font-semibold text-white mb-4">Manos Recientes</h2>

              {recentHands.isPending && <p className="text-slate-400">Cargando manos...</p>}
              {recentHands.isError && (
                <div className="bg-red-900/20 border border-red-600 p-3 rounded">
                  <p className="text-red-400 text-sm">Error al cargar manos</p>
                </div>
              )}

              {recentHands.data && (
                <div className="space-y-2">
                  {recentHands.data.hands.slice(0, 5).map((hand) => (
                    <div
                      key={hand.id}
                      className="flex items-center justify-between p-3 bg-slate-800 rounded cursor-pointer hover:bg-slate-700 transition"
                      onClick={() => setSelectedHandId(hand.id)}
                    >
                      <div className="flex-1">
                        <p className="text-white text-sm font-medium">{hand.stakes}</p>
                        <p className="text-slate-400 text-xs">
                          {new Date(hand.timestamp).toLocaleString()}
                        </p>
                      </div>
                      <Badge
                        variant={hand.result > 0 ? 'success' : 'error'}
                        className="text-xs"
                      >
                        {hand.result > 0 ? '+' : ''}
                        {hand.result / 100}
                      </Badge>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </Card>

          {/* ========== SECCIÓN 3: Hand Details ========== */}
          {selectedHandId && (
            <Card className="lg:col-span-2">
              <div className="p-6">
                <h2 className="text-xl font-semibold text-white mb-4">Detalles de Mano</h2>

                {hand.isPending && <p className="text-slate-400">Cargando detalles...</p>}
                {hand.isError && (
                  <div className="bg-red-900/20 border border-red-600 p-3 rounded">
                    <p className="text-red-400 text-sm">Error al cargar detalles</p>
                  </div>
                )}

                {hand.data && (
                  <div className="space-y-4">
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                      <div>
                        <p className="text-slate-400 text-sm">Stakes</p>
                        <p className="text-white font-medium">{hand.data.stakes}</p>
                      </div>
                      <div>
                        <p className="text-slate-400 text-sm">Posición Hero</p>
                        <p className="text-white font-medium">{hand.data.heroPosition}</p>
                      </div>
                      <div>
                        <p className="text-slate-400 text-sm">Resultado</p>
                        <p
                          className={
                            hand.data.result > 0
                              ? 'text-green-400 font-medium'
                              : 'text-red-400 font-medium'
                          }
                        >
                          {hand.data.result > 0 ? '+' : ''}
                          {(hand.data.result / 100).toFixed(2)}
                        </p>
                      </div>
                      <div>
                        <p className="text-slate-400 text-sm">ID</p>
                        <p className="text-white font-mono text-xs">{hand.data.id.slice(0, 8)}</p>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </Card>
          )}

          {/* ========== SECCIÓN 4: Equity Calculator ========== */}
          <Card className="lg:col-span-2">
            <div className="p-6">
              <h2 className="text-xl font-semibold text-white mb-4">Calculador de Equidad</h2>

              <div className="space-y-4">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <Input
                    label="Rango Hero (ej: AA,KK,AKs)"
                    value={equityForm.heroRange}
                    onChange={(e) => setEquityForm({ ...equityForm, heroRange: e.target.value })}
                  />
                  <Input
                    label="Rango Villain (ej: QQ+,AJs)"
                    value={equityForm.villainRange}
                    onChange={(e) =>
                      setEquityForm({ ...equityForm, villainRange: e.target.value })
                    }
                  />
                </div>

                <Button onClick={handleCalculateEquity} disabled={equityCalc.isPending}>
                  {equityCalc.isPending ? 'Calculando...' : 'Calcular Equidad'}
                </Button>

                {equityCalc.isError && (
                  <div className="bg-red-900/20 border border-red-600 p-3 rounded">
                    <p className="text-red-400 text-sm">Error en cálculo de equidad</p>
                  </div>
                )}

                {equityCalc.data && (
                  <div className="grid grid-cols-3 gap-4">
                    <div className="bg-slate-800 p-4 rounded">
                      <p className="text-slate-400 text-sm">Hero Equity</p>
                      <p className="text-green-400 text-2xl font-bold">
                        {(equityCalc.data.heroEquity * 100).toFixed(1)}%
                      </p>
                    </div>
                    <div className="bg-slate-800 p-4 rounded">
                      <p className="text-slate-400 text-sm">Villain Equity</p>
                      <p className="text-red-400 text-2xl font-bold">
                        {(equityCalc.data.villainEquity * 100).toFixed(1)}%
                      </p>
                    </div>
                    <div className="bg-slate-800 p-4 rounded">
                      <p className="text-slate-400 text-sm">Tie</p>
                      <p className="text-slate-300 text-2xl font-bold">
                        {(equityCalc.data.tieEquity * 100).toFixed(1)}%
                      </p>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </Card>
        </div>
      </div>
    </div>
  )
}

