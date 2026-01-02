// Tipos de base de datos y API
// Note: Los tipos más detallados para Hand, Action, etc. están en api.ts

export interface Player {
  id: string
  name: string
  vpip: number
  pfr: number
  threebet: number
  winrate: number
  totalHands: number
}

export interface Action {
  player: string
  action: 'fold' | 'check' | 'call' | 'bet' | 'raise'
  amount: number
  street: 'preflop' | 'flop' | 'turn' | 'river'
}

export interface APIResponse<T> {
  data: T
  error?: string
  timestamp: string
}

// ============================================================================
// REPRODUCTOR DE MANOS - MÁQUINA DE ESTADOS
// ============================================================================

export type ReplayerPlaybackState = 'idle' | 'playing' | 'paused' | 'finished'

export type PlaybackSpeed = 1 | 2 | 5 | 10

export interface ReplayerActionStep {
  index: number
  street: 'preflop' | 'flop' | 'turn' | 'river'
  player: string
  action: 'fold' | 'check' | 'call' | 'bet' | 'raise' | 'all-in'
  amount: number
  description: string
}

export interface ReplayerState {
  state: ReplayerPlaybackState
  currentActionIndex: number
  totalActions: number
  playbackSpeed: PlaybackSpeed
  isPaused: boolean
}

export interface ReplayerAction {
  type: 'PLAY' | 'PAUSE' | 'STOP' | 'STEP_FORWARD' | 'STEP_BACKWARD' | 'SET_SPEED' | 'JUMP_TO_ACTION' | 'FINISH'
  payload?: unknown
}
