/**
 * types/ranges.ts
 * Tipos TypeScript para el sistema de rangos de poker
 */

// ============================================================================
// TIPOS BASE DE MANOS
// ============================================================================

/**
 * Tipo de mano de poker
 * - pair: Parejas (AA, KK, QQ, ..., 22)
 * - suited: Manos del mismo palo (AKs, KQs, T9s, ...)
 * - offsuit: Manos de diferente palo (AKo, KQo, J9o, ...)
 */
export type HandType = 'pair' | 'suited' | 'offsuit'

/**
 * Carta individual (rank)
 * Orden: A > K > Q > J > T > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2
 */
export type Rank = 'A' | 'K' | 'Q' | 'J' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2'

/**
 * Notación de una mano de poker
 * Ejemplos: "AA", "AKs", "AKo", "KQs", "72o"
 */
export type HandNotation = string

/**
 * Información completa de una mano
 */
export interface Hand {
  notation: HandNotation // "AA", "AKs", "AKo", etc.
  type: HandType // pair, suited, offsuit
  rank1: Rank // Primera carta (mayor)
  rank2: Rank // Segunda carta (menor o igual)
}

// ============================================================================
// RANGOS Y FRECUENCIAS
// ============================================================================

/**
 * Frecuencia de una acción (0.0 a 1.0)
 * 0.0 = nunca (0%)
 * 0.5 = mitad del tiempo (50%)
 * 1.0 = siempre (100%)
 */
export type Frequency = number

/**
 * Tipo de acción en el rango
 */
export type RangeAction = 'RAISE' | 'CALL' | 'FOLD' | 'ALL_IN' | 'MARGINAL'

/**
 * Entrada individual de una mano en un rango
 */
export interface RangeEntry {
  hand: HandNotation // "AKs", "QQ", etc.
  action: RangeAction // Tipo de acción
  frequency: Frequency // Frecuencia (0.0-1.0)
}

/**
 * Rango completo (169 manos)
 * Cada mano puede tener múltiples acciones (estrategia mixta)
 */
export type RangeData = {
  [hand: HandNotation]: RangeEntry[]
}

/**
 * Metadatos de un rango estratégico
 */
export interface RangeMetadata {
  id: string // "EP_Open_Raise", "BTN_vs_CO_3bet", etc.
  title: string // "EP Open Raise 100bb"
  category: string // "RFI", "3Bet", "4Bet", "Blind Defense", "Squeeze"
  position?: string // "SB", "BB", "BTN", "CO", "MP", "UTG"
  tags?: string[] // ["6-max", "GTO", "Cash"]
  benchmarkTime?: number // Tiempo de referencia en segundos
}

/**
 * Rango completo con metadatos
 */
export interface Range {
  metadata: RangeMetadata
  data: RangeData
}

// ============================================================================
// MATRIZ 13x13
// ============================================================================

/**
 * Posición en la matriz 13x13
 * row: 0 (A) -> 12 (2)
 * col: 0 (A) -> 12 (2)
 */
export interface MatrixPosition {
  row: number // 0-12 (A-2)
  col: number // 0-12 (A-2)
}

/**
 * Celda de la matriz con información de la mano
 */
export interface MatrixCell {
  position: MatrixPosition
  hand: Hand
  entries: RangeEntry[] // Acciones asociadas (puede haber múltiples)
  isSelected: boolean // Para drag-to-select
}

/**
 * Estado de la matriz completa (169 celdas)
 */
export type MatrixState = MatrixCell[]

// ============================================================================
// PRESETS DE RANGOS
// ============================================================================

/**
 * Preset de rango predefinido
 */
export interface RangePreset {
  id: string // "ep_open", "btn_open", "sb_open", etc.
  name: string // "EP Open"
  description: string // "Early Position Open Raise"
  position: string // "UTG", "MP", "CO", "BTN", "SB", "BB"
  range: RangeData
}

/**
 * Categorías de presets
 */
export type PresetCategory = 'RFI' | '3Bet' | '4Bet' | 'Blind Defense' | 'Squeeze' | 'Call'

// ============================================================================
// COLORES Y VISUALIZACIÓN
// ============================================================================

/**
 * Configuración de color para una acción
 */
export interface ActionColor {
  action: RangeAction
  color: string // Hex color
  label: string // "Raise", "Call", etc.
}

/**
 * Mapa de calor: frecuencia → color
 */
export interface HeatmapConfig {
  minColor: string // Color para frecuencia 0.0
  maxColor: string // Color para frecuencia 1.0
  steps?: number // Número de pasos en el gradiente (default: 10)
}

// ============================================================================
// UTILIDADES
// ============================================================================

/**
 * Todas las ranks en orden descendente
 */
export const RANKS: Rank[] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']

/**
 * Índice de rank (A=0, K=1, ..., 2=12)
 */
export const RANK_INDEX: Record<Rank, number> = {
  'A': 0, 'K': 1, 'Q': 2, 'J': 3, 'T': 4,
  '9': 5, '8': 6, '7': 7, '6': 8, '5': 9,
  '4': 10, '3': 11, '2': 12
}

/**
 * Colores por acción (usando paleta del proyecto)
 */
export const ACTION_COLORS: Record<RangeAction, string> = {
  RAISE: '#EF4444', // poker-raise (red-500)
  CALL: '#3B82F6', // poker-call (blue-500)
  FOLD: '#64748B', // poker-fold (slate-500)
  ALL_IN: '#F59E0B', // amber-500
  MARGINAL: '#8B5CF6' // accent-violet
}

