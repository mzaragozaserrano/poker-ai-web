// Types for canvas rendering

import type { SeatPosition } from './constants'
import type { AmountFormat } from '../../hooks/useAmountFormat'

/** Coordenadas 2D */
export interface Point {
  x: number
  y: number
}

/** Dimensiones del canvas */
export interface CanvasDimensions {
  width: number
  height: number
}

/** Estado de un jugador en la mesa */
export interface PlayerState {
  id: string
  name: string
  stack: number
  position: SeatPosition
  isHero: boolean
  isFolded: boolean
  isActive: boolean
  /** Cards si son visibles (hole cards del hero o showdown) */
  cards?: [string, string]
  /** Bet actual en la calle */
  currentBet?: number
}

/** Estado del pot */
export interface PotState {
  main: number
  side?: number[]
}

/** Estado de la mesa completa */
export interface TableState {
  players: PlayerState[]
  pot: PotState
  dealerPosition: SeatPosition
  communityCards: string[]
  currentStreet: 'preflop' | 'flop' | 'turn' | 'river'
}

/** Props para el componente PokerTable */
export interface PokerTableProps {
  tableState: TableState
  width?: number
  height?: number
  onPlayerClick?: (playerId: string) => void
  amountFormat?: AmountFormat
  bigBlind?: number
}

/** Props para el componente PlayerSeat */
export interface PlayerSeatProps {
  player: PlayerState
  position: Point
  isDealer: boolean
  scale?: number
  onClick?: () => void
}

/** Configuracion de escala para responsive */
export interface ScaleConfig {
  scale: number
  offsetX: number
  offsetY: number
}

