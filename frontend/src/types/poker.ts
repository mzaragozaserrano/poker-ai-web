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
