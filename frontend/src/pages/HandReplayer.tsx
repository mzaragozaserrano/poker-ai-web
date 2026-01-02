import { useParams } from 'react-router-dom'
import { useRef, useState, useEffect, useCallback } from 'react'
import { PokerTable, ReplayerControls, ReplayerTimeline } from '../features/replayer'
import { useReplayerState, useAmountFormat } from '../hooks'
import type { TableState, SeatPosition } from '../lib/canvas'
import type { ReplayerActionStep } from '../types/poker'

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

// Acciones de ejemplo para demostración
const DEMO_ACTIONS: ReplayerActionStep[] = [
  {
    index: 0,
    street: 'preflop',
    player: 'UTG',
    action: 'raise',
    amount: 300,
    description: 'UTG abre a 3x',
  },
  {
    index: 1,
    street: 'preflop',
    player: 'thesmoy',
    action: 'raise',
    amount: 900,
    description: '3-bet a 9x',
  },
  {
    index: 2,
    street: 'preflop',
    player: 'SB',
    action: 'fold',
    amount: 0,
    description: 'SB foldea',
  },
  {
    index: 3,
    street: 'preflop',
    player: 'UTG',
    action: 'call',
    amount: 600,
    description: 'UTG iguala',
  },
  {
    index: 4,
    street: 'flop',
    player: 'UTG',
    action: 'check',
    amount: 0,
    description: 'UTG checkea',
  },
  {
    index: 5,
    street: 'flop',
    player: 'thesmoy',
    action: 'bet',
    amount: 600,
    description: 'thesmoy apuesta',
  },
  {
    index: 6,
    street: 'flop',
    player: 'UTG',
    action: 'raise',
    amount: 1800,
    description: 'UTG sube',
  },
  {
    index: 7,
    street: 'flop',
    player: 'thesmoy',
    action: 'call',
    amount: 1200,
    description: 'thesmoy iguala',
  },
  {
    index: 8,
    street: 'turn',
    player: 'UTG',
    action: 'check',
    amount: 0,
    description: 'UTG checkea',
  },
  {
    index: 9,
    street: 'turn',
    player: 'thesmoy',
    action: 'bet',
    amount: 1200,
    description: 'thesmoy apuesta',
  },
  {
    index: 10,
    street: 'turn',
    player: 'UTG',
    action: 'fold',
    amount: 0,
    description: 'UTG foldea',
  },
]

export const HandReplayer = () => {
  const { handId } = useParams()
  const containerRef = useRef<HTMLDivElement>(null)
  const [dimensions, setDimensions] = useState({ width: 800, height: 500 })
  const [currentStreet, setCurrentStreet] = useState<'preflop' | 'flop' | 'turn' | 'river'>('preflop')
  const [selectedPlayer, setSelectedPlayer] = useState<string | null>(null)
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  // Hook para controlar la reproducción
  const replayer = useReplayerState({
    totalActions: DEMO_ACTIONS.length,
    initialSpeed: 1,
  })

  // Hook para gestionar el formato de cantidades
  const { format: amountFormat, toggleFormat } = useAmountFormat()

  // Combinar estado base con estado de la calle actual
  const tableState: TableState = {
    ...DEMO_TABLE_STATE,
    ...DEMO_STATES[currentStreet],
  } as TableState

  // Obtener la acción actual
  const currentAction = DEMO_ACTIONS[replayer.state.currentActionIndex]

  // Actualizar la calle basada en la acción actual
  useEffect(() => {
    if (currentAction) {
      setCurrentStreet(currentAction.street)
    }
  }, [currentAction])

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
          height: Math.max(300, height),
        })
      }
    })

    resizeObserver.observe(container)
    return () => resizeObserver.disconnect()
  }, [])

  // Lógica de reproducción automática con velocidad variable
  useEffect(() => {
    if (replayer.state.state === 'playing' && replayer.state.currentActionIndex < replayer.state.totalActions - 1) {
      // Calcular delay basado en velocidad (velocidad x10 = delay 100ms, etc)
      const baseDelay = 800 // ms entre acciones
      const delay = baseDelay / replayer.state.playbackSpeed

      timerRef.current = setTimeout(() => {
        replayer.stepForward()
      }, delay)

      return () => {
        if (timerRef.current) {
          clearTimeout(timerRef.current)
        }
      }
    }

    // Si se llegó al final, cambiar a finished
    if (replayer.state.state === 'playing' && replayer.state.currentActionIndex >= replayer.state.totalActions - 1) {
      replayer.finish()
    }
  }, [replayer.state.state, replayer.state.currentActionIndex, replayer.state.playbackSpeed, replayer])

  const handlePlayerClick = useCallback((playerId: string) => {
    setSelectedPlayer(playerId === selectedPlayer ? null : playerId)
  }, [selectedPlayer])

  const handleActionClick = useCallback(
    (index: number) => {
      replayer.jumpToAction(index)
    },
    [replayer]
  )

  return (
    <div className="h-full flex flex-col gap-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-white">Hand Replayer</h1>
          <p className="text-slate-400 text-sm">
            {handId ? `Mano ID: ${handId}` : 'Demo Mode'} - {currentAction?.description}
          </p>
        </div>

        {/* Estado de reproducción */}
        <div className="text-right">
          <div className="text-xs text-slate-400 mb-1">Estado</div>
          <div className={`text-sm font-semibold capitalize ${
            replayer.state.state === 'playing'
              ? 'text-green-400'
              : replayer.state.state === 'finished'
                ? 'text-amber-400'
                : 'text-slate-300'
          }`}>
            {replayer.state.state}
          </div>
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
          amountFormat={amountFormat}
          bigBlind={200}
        />
      </div>

      {/* Controles de reproducción */}
      <ReplayerControls
        isPlaying={replayer.state.state === 'playing'}
        currentActionIndex={replayer.state.currentActionIndex}
        totalActions={replayer.state.totalActions}
        playbackSpeed={replayer.state.playbackSpeed}
        amountFormat={amountFormat}
        onPlay={replayer.play}
        onPause={replayer.pause}
        onStop={replayer.stop}
        onStepForward={replayer.stepForward}
        onStepBackward={replayer.stepBackward}
        onSetSpeed={replayer.setSpeed}
        onToggleAmountFormat={toggleFormat}
      />

      {/* Timeline de acciones */}
      <ReplayerTimeline
        actions={DEMO_ACTIONS}
        currentActionIndex={replayer.state.currentActionIndex}
        onActionClick={handleActionClick}
      />

      {/* Player info panel */}
      {selectedPlayer && (
        <div className="p-4 bg-slate-800 rounded-lg border border-slate-700">
          <h3 className="text-white font-medium mb-2">Jugador Seleccionado</h3>
          <p className="text-slate-400 text-sm">
            {tableState.players.find((p) => p.id === selectedPlayer)?.name ?? 'Desconocido'}
          </p>
        </div>
      )}

      {/* Info footer */}
      <div className="flex items-center justify-between text-sm text-slate-500">
        <div className="flex items-center gap-4">
          <span>
            Pot: <span className="text-amber-400 font-mono">{tableState.pot.main}</span>
          </span>
          <span>
            Street: <span className="text-slate-300">{tableState.currentStreet}</span>
          </span>
          <span>
            Cards: <span className="text-slate-300">{tableState.communityCards.length}/5</span>
          </span>
        </div>
        <div className="text-slate-600">Canvas: {dimensions.width}x{dimensions.height}</div>
      </div>
    </div>
  )
}
