// Canvas constants for the Poker Table renderer

/** Posiciones 6-max en sentido horario desde BTN */
export type SeatPosition = 'BTN' | 'SB' | 'BB' | 'UTG' | 'MP' | 'CO'

export const SEAT_POSITIONS: SeatPosition[] = ['BTN', 'SB', 'BB', 'UTG', 'MP', 'CO']

/** Colores del tapete y mesa */
export const TABLE_COLORS = {
  felt: '#1a472a',          // Verde oscuro bosque
  feltGradientStart: '#1a472a',
  feltGradientEnd: '#0d2818',
  border: '#3d2314',        // Madera oscura
  borderHighlight: '#5c3a2e',
  rail: '#2a1810',
  potArea: 'rgba(255, 255, 255, 0.05)',
} as const

/** Colores de jugador */
export const PLAYER_COLORS = {
  avatarDefault: '#475569',     // slate-600
  avatarHero: '#8B5CF6',        // violet-500
  nameText: '#F8FAFC',          // slate-50
  stackText: '#CBD5E1',         // slate-300
  positionBadge: '#1E293B',     // slate-800
  positionText: '#94A3B8',      // slate-400
  activeBorder: '#10B981',      // emerald-500
  foldedOverlay: 'rgba(0, 0, 0, 0.5)',
} as const

/** Colores del dealer button */
export const DEALER_BUTTON_COLORS = {
  background: '#FBBF24',        // amber-400
  text: '#1E293B',              // slate-800
  border: '#F59E0B',            // amber-500
} as const

/** Dimensiones base (escalables) */
export const TABLE_DIMENSIONS = {
  /** Radio X de la elipse de la mesa */
  radiusX: 320,
  /** Radio Y de la elipse de la mesa */
  radiusY: 180,
  /** Grosor del borde de madera */
  borderWidth: 20,
  /** Radio del rail interior */
  railWidth: 8,
} as const

export const SEAT_DIMENSIONS = {
  /** Radio del avatar */
  avatarRadius: 32,
  /** Ancho del area del asiento */
  width: 100,
  /** Alto del area del asiento */
  height: 80,
  /** Distancia desde el centro de la mesa al asiento */
  distanceFromCenter: 0.85, // porcentaje del radio
} as const

export const DEALER_BUTTON_DIMENSIONS = {
  radius: 16,
  fontSize: 12,
} as const

export const POT_DIMENSIONS = {
  fontSize: 18,
  chipRadius: 12,
} as const

/** Angulos de posicion para cada asiento (en grados, 0 = derecha, sentido horario) */
export const SEAT_ANGLES: Record<SeatPosition, number> = {
  BTN: 270,   // Abajo centro (posicion del heroe)
  SB: 210,    // Abajo izquierda
  BB: 150,    // Arriba izquierda
  UTG: 90,    // Arriba centro
  MP: 30,     // Arriba derecha
  CO: 330,    // Abajo derecha
}

