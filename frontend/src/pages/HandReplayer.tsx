import { useParams } from 'react-router-dom'
import { useRef, useState, useEffect, useCallback } from 'react'
import { PokerTable } from '../features/replayer'
import type { TableState, SeatPosition } from '../lib/canvas'

// Estado de ejemplo para demostrar la mesa
const DEMO_TABLE_STATE: TableState = {
  players: [
    { 
      id: '1', 
      name: 'thesmoy', 
      stack: 10250, 
      position: 'BTN', 
      isHero: true, 
      isFolded: false, 
      isActive: true,
      cards: ['As', 'Kh'] // Hero siempre ve sus cartas
    },
    { 
      id: '2', 
      name: 'Villain1', 
      stack: 8500, 
      position: 'SB', 
      isHero: false, 
      isFolded: false, 
      isActive: false, 
      currentBet: 50,
      cards: ['??', '??'] // Cartas no visibles
    },
    { 
      id: '3', 
      name: 'Player3', 
      stack: 15000, 
      position: 'BB', 
      isHero: false, 
      isFolded: false, 
      isActive: false, 
      currentBet: 100,
      cards: ['Qd', 'Jc'] // Visible en showdown
    },
    { 
      id: '4', 
      name: 'FishyMcFish', 
      stack: 5200, 
      position: 'UTG', 
      isHero: false, 
      isFolded: true, 
      isActive: false 
    },
    { 
      id: '5', 
      name: 'RegPlayer', 
      stack: 12000, 
      position: 'MP', 
      isHero: false, 
      isFolded: false, 
      isActive: false,
      cards: ['??', '??']
    },
    { 
      id: '6', 
      name: 'NitKing', 
      stack: 9800, 
      position: 'CO', 
      isHero: false, 
      isFolded: false, 
      isActive: false 
    },
  ],
  pot: { main: 150 },
  dealerPosition: 'BTN' as SeatPosition,
  communityCards: [],
  currentStreet: 'preflop',
}

// Estados de demostratcion por calle
const DEMO_STATES: Record<string, Partial<TableState>> = {
  preflop: {
    communityCards: [],
    currentStreet: 'preflop',
    pot: { main: 150 },
  },
  flop: {
    communityCards: ['Ah', 'Kd', '7c'],
    currentStreet: 'flop',
    pot: { main: 450 },
  },
  turn: {
    communityCards: ['Ah', 'Kd', '7c', '2s'],
    currentStreet: 'turn',
    pot: { main: 850 },
  },
  river: {
    communityCards: ['Ah', 'Kd', '7c', '2s', 'Qh'],
    currentStreet: 'river',
    pot: { main: 1650 },
  },
}

export const HandReplayer = () => {
  const { handId } = useParams()
  const containerRef = useRef<HTMLDivElement>(null)
  const [dimensions, setDimensions] = useState({ width: 800, height: 500 })
  const [currentStreet, setCurrentStreet] = useState<'preflop' | 'flop' | 'turn' | 'river'>('preflop')
  const [selectedPlayer, setSelectedPlayer] = useState<string | null>(null)

  // Combinar estado base con estado de la calle actual
  const tableState: TableState = {
    ...DEMO_TABLE_STATE,
    ...DEMO_STATES[currentStreet],
  } as TableState

  // Observar cambios de tamano del contenedor
  useEffect(() => {
    const container = containerRef.current
    if (!container) return

    const resizeObserver = new ResizeObserver((entries) => {
      const entry = entries[0]
      if (entry) {
        const { width, height } = entry.contentRect
        setDimensions({ 
          width: Math.max(400, width), 
          height: Math.max(300, height) 
        })
      }
    })

    resizeObserver.observe(container)
    return () => resizeObserver.disconnect()
  }, [])

  const handlePlayerClick = useCallback((playerId: string) => {
    setSelectedPlayer(playerId === selectedPlayer ? null : playerId)
  }, [selectedPlayer])

  const streets: Array<'preflop' | 'flop' | 'turn' | 'river'> = ['preflop', 'flop', 'turn', 'river']

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h1 className="text-2xl font-bold text-white">Hand Replayer</h1>
          <p className="text-slate-400 text-sm">
            {handId ? `Mano ID: ${handId}` : 'Demo Mode'}
          </p>
        </div>

        {/* Street selector */}
        <div className="flex gap-2">
          {streets.map((street) => (
            <button
              key={street}
              onClick={() => setCurrentStreet(street)}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                currentStreet === street
                  ? 'bg-violet-600 text-white'
                  : 'bg-slate-800 text-slate-400 hover:bg-slate-700 hover:text-slate-200'
              }`}
            >
              {street.charAt(0).toUpperCase() + street.slice(1)}
            </button>
          ))}
        </div>
      </div>

      {/* Canvas container */}
      <div 
        ref={containerRef}
        className="flex-1 bg-slate-900 rounded-xl border border-slate-700 overflow-hidden min-h-[400px]"
      >
        <PokerTable
          tableState={tableState}
          width={dimensions.width}
          height={dimensions.height}
          onPlayerClick={handlePlayerClick}
        />
      </div>

      {/* Player info panel */}
      {selectedPlayer && (
        <div className="mt-4 p-4 bg-slate-800 rounded-lg border border-slate-700">
          <h3 className="text-white font-medium mb-2">Jugador Seleccionado</h3>
          <p className="text-slate-400 text-sm">
            {tableState.players.find(p => p.id === selectedPlayer)?.name ?? 'Desconocido'}
          </p>
        </div>
      )}

      {/* Info footer */}
      <div className="mt-4 flex items-center justify-between text-sm text-slate-500">
        <div className="flex items-center gap-4">
          <span>Pot: <span className="text-amber-400 font-mono">{tableState.pot.main}</span></span>
          <span>Street: <span className="text-slate-300">{tableState.currentStreet}</span></span>
          <span>Cards: <span className="text-slate-300">{tableState.communityCards.length}/5</span></span>
        </div>
        <div className="text-slate-600">
          Canvas: {dimensions.width}x{dimensions.height}
        </div>
      </div>
    </div>
  )
}
