/**
 * tipos/api.ts
 * Definiciones de tipos para todas las respuestas de la API
 * Backend: http://127.0.0.1:8000/api/v1
 */

// ============================================================================
// RESPUESTAS GENÉRICAS
// ============================================================================

export interface ApiErrorResponse {
  error: string
  detail?: string
  timestamp: string
}

export interface ApiSuccessResponse<T> {
  data: T
  timestamp: string
}

// ============================================================================
// ESTADÍSTICAS DE JUGADOR
// ============================================================================

export interface PlayerStats {
  name: string
  totalHands: number
  vpip: number // Voluntarily Put In Pot %
  pfr: number // Preflop Raise %
  threeBet: number // 3-bet %
  fourBet: number // 4-bet %
  winrate: number // BB/100
  totalProfit: number // En centavos
  roi: number // Return On Investment %
  wtsd?: number // Went To ShowDown % (opcional, futuro)
  positionalStats?: PositionalStats
}

export interface PositionalStats {
  btn?: PositionData
  sb?: PositionData
  bb?: PositionData
  utg?: PositionData
  mp?: PositionData
  co?: PositionData
}

export interface PositionData {
  hands: number
  vpip: number
  pfr: number
  winrate: number
}

// ============================================================================
// HISTORIAL DE MANOS
// ============================================================================

export interface HandSummary {
  id: string
  timestamp: string
  gameType: string // 'NLH 6-max'
  stakes: string // '0.25/0.50'
  button: string
  smallBlind: string
  bigBlind: string
  heroPosition: string // 'BTN' | 'SB' | 'BB' | 'UTG' | 'MP' | 'CO'
  result: number // En centavos (positivo o negativo)
  winrate?: number
}

export interface Hand {
  id: string
  timestamp: string
  gameType: string
  stakes: string
  button: string
  smallBlind: string
  bigBlind: string
  hero: string
  heroPosition: string
  heroHoleCards: Card[]
  players: HandPlayer[]
  streets: Street[]
  result: number
  summary?: string
}

export interface HandPlayer {
  name: string
  stack: number // En centavos
  position: string
  isHero: boolean
}

export interface Street {
  name: 'preflop' | 'flop' | 'turn' | 'river'
  boardCards?: Card[]
  actions: HandAction[]
}

export interface HandAction {
  player: string
  action: 'fold' | 'check' | 'call' | 'bet' | 'raise' | 'all-in'
  amount: number // En centavos
  timestamp?: number
}

export interface Card {
  rank: string // 'A' | 'K' | 'Q' | 'J' | 'T' | '9' | ... | '2'
  suit: string // 'h' | 'd' | 'c' | 's'
}

// ============================================================================
// CÁLCULO DE EQUIDAD
// ============================================================================

export interface EquityCalculationRequest {
  heroRange: string // Ej: "AA,KK,AKs,AQs"
  villainRange: string // Ej: "QQ+,AJs"
  boardCards?: Card[]
  runouts?: number // Simulaciones Monte Carlo
}

export interface EquityCalculationResponse {
  heroEquity: number // 0-1
  villainEquity: number // 0-1
  tieEquity: number // 0-1
  simulationCount: number
  executionTimeMs: number
}

export interface EquityCalculationMultiwayRequest {
  ranges: Range[]
  boardCards?: Card[]
  runouts?: number
}

export interface Range {
  name: string
  range: string
}

export interface EquityCalculationMultiwayResponse {
  equities: EquityData[]
  simulationCount: number
  executionTimeMs: number
}

export interface EquityData {
  name: string
  equity: number // 0-1
}

// ============================================================================
// RESPUESTAS DE ENDPOINTS
// ============================================================================

export interface GetPlayerStatsResponse extends ApiSuccessResponse<PlayerStats> {}

export interface GetRecentHandsResponse
  extends ApiSuccessResponse<{ hands: HandSummary[]; limit: number; total: number }> {}

export interface GetHandResponse extends ApiSuccessResponse<Hand> {}

export interface PostEquityCalculateResponse extends ApiSuccessResponse<EquityCalculationResponse> {}

export interface PostEquityCalculateMultiwayResponse
  extends ApiSuccessResponse<EquityCalculationMultiwayResponse> {}

// ============================================================================
// WEBSOCKET MESSAGES
// ============================================================================

export type WebSocketMessageType = 'connection_ack' | 'heartbeat' | 'new_hand' | 'error'

export interface BaseWebSocketMessage {
  type: WebSocketMessageType
  timestamp: string
}

export interface ConnectionAckMessage extends BaseWebSocketMessage {
  type: 'connection_ack'
  client_id: string
}

export interface HeartbeatMessage extends BaseWebSocketMessage {
  type: 'heartbeat'
}

export interface NewHandMessage extends BaseWebSocketMessage {
  type: 'new_hand'
  hand_id: string
  hero_result: number | null
  hero_position: string
  game_type: string
  stakes: string
}

export interface ErrorMessage extends BaseWebSocketMessage {
  type: 'error'
  message: string
}

export type WebSocketMessage =
  | ConnectionAckMessage
  | HeartbeatMessage
  | NewHandMessage
  | ErrorMessage

export type ConnectionStatus = 'connecting' | 'connected' | 'disconnected' | 'reconnecting'

// ============================================================================
// TIPOS DE UTILIDAD
// ============================================================================

export type ApiResponse<T> = ApiSuccessResponse<T> | ApiErrorResponse

/**
 * Hook para determinar si una respuesta es un error
 */
export const isErrorResponse = (response: ApiResponse<any>): response is ApiErrorResponse => {
  return 'error' in response
}

/**
 * Hook para determinar si una respuesta es exitosa
 */
export const isSuccessResponse = <T,>(response: ApiResponse<T>): response is ApiSuccessResponse<T> => {
  return 'data' in response
}

