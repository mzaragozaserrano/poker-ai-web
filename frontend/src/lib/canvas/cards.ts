// Utilities for card parsing and rendering

/** Valores de cartas validos */
export type CardValue = '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A'

/** Palos de cartas validos */
export type CardSuit = 'h' | 'd' | 'c' | 's'

/** Card notation completa (ej: "Ah", "Kd", "Ts") */
export type CardNotation = `${CardValue}${CardSuit}`

/** Parsed card object */
export interface ParsedCard {
  value: CardValue
  suit: CardSuit
  display: string
  color: 'red' | 'black'
  suitSymbol: string
}

/** Simbolos Unicode para cada palo */
export const SUIT_SYMBOLS: Record<CardSuit, string> = {
  h: '♥',  // Hearts
  d: '♦',  // Diamonds
  c: '♣',  // Clubs
  s: '♠',  // Spades
}

/** Nombres completos de palos */
export const SUIT_NAMES: Record<CardSuit, string> = {
  h: 'hearts',
  d: 'diamonds',
  c: 'clubs',
  s: 'spades',
}

/** Colores por palo */
export const SUIT_COLORS: Record<CardSuit, 'red' | 'black'> = {
  h: 'red',
  d: 'red',
  c: 'black',
  s: 'black',
}

/** Display values (T -> 10) */
export const VALUE_DISPLAY: Record<CardValue, string> = {
  '2': '2',
  '3': '3',
  '4': '4',
  '5': '5',
  '6': '6',
  '7': '7',
  '8': '8',
  '9': '9',
  'T': '10',
  'J': 'J',
  'Q': 'Q',
  'K': 'K',
  'A': 'A',
}

/** Colores de cartas para renderizado */
export const CARD_COLORS = {
  red: '#EF4444',      // red-500
  black: '#1E293B',    // slate-800
  background: '#FFFFFF',
  border: '#E2E8F0',   // slate-200
  backPattern: '#1E293B',
  backBackground: '#64748B', // slate-500
} as const

/** Dimensiones de carta (escalables) */
export const CARD_DIMENSIONS = {
  width: 40,
  height: 56,
  cornerRadius: 4,
  borderWidth: 1,
  /** Font size para valor en esquinas */
  cornerFontSize: 14,
  /** Font size para simbolo central */
  suitFontSize: 24,
  /** Padding desde bordes */
  padding: 4,
} as const

/**
 * Valida si una string es una notacion de carta valida
 * @example isValidCard('Ah') => true
 * @example isValidCard('XX') => false
 */
export function isValidCard(notation: string): notation is CardNotation {
  if (notation.length !== 2) return false
  
  const value = notation[0].toUpperCase()
  const suit = notation[1].toLowerCase()
  
  const validValues: string[] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
  const validSuits: string[] = ['h', 'd', 'c', 's']
  
  return validValues.includes(value) && validSuits.includes(suit)
}

/**
 * Parsea una notacion de carta a un objeto estructurado
 * @throws Error si la notacion es invalida
 */
export function parseCard(notation: string): ParsedCard {
  const normalized = notation.charAt(0).toUpperCase() + notation.charAt(1).toLowerCase()
  
  if (!isValidCard(normalized)) {
    throw new Error(`Invalid card notation: ${notation}`)
  }
  
  const value = normalized[0] as CardValue
  const suit = normalized[1] as CardSuit
  
  return {
    value,
    suit,
    display: VALUE_DISPLAY[value],
    color: SUIT_COLORS[suit],
    suitSymbol: SUIT_SYMBOLS[suit],
  }
}

/**
 * Parsea multiples cartas
 * @returns Array de cartas parseadas (ignora cartas invalidas)
 */
export function parseCards(notations: string[]): ParsedCard[] {
  return notations
    .filter(isValidCard)
    .map(parseCard)
}

/**
 * Obtiene el color del texto para una carta
 */
export function getCardColor(suit: CardSuit): string {
  return SUIT_COLORS[suit] === 'red' ? CARD_COLORS.red : CARD_COLORS.black
}

/**
 * Formatea una carta para display (con simbolo)
 * @example formatCardDisplay('Ah') => 'A♥'
 */
export function formatCardDisplay(notation: string): string {
  if (!isValidCard(notation)) return notation
  
  const card = parseCard(notation)
  return `${card.display}${card.suitSymbol}`
}

/**
 * Obtiene el path SVG para renderizar un palo como shape
 * (Alternativa a usar simbolos Unicode)
 */
export function getSuitPath(suit: CardSuit): string {
  // Paths simplificados para cada palo (centrados en 0,0 con size ~20)
  switch (suit) {
    case 'h': // Heart
      return 'M0,-8 C-5,-13 -10,-13 -10,-6 C-10,-2 -5,2 0,8 C5,2 10,-2 10,-6 C10,-13 5,-13 0,-8Z'
    case 'd': // Diamond
      return 'M0,-10 L8,0 L0,10 L-8,0 Z'
    case 'c': // Club
      return 'M0,-8 C-3,-8 -5,-6 -5,-3 C-5,-1 -4,1 -2,2 L-3,8 L3,8 L2,2 C4,1 5,-1 5,-3 C5,-6 3,-8 0,-8 M-6,-2 C-6,-4 -5,-6 -3,-6 C-1,-6 0,-4 0,-2 C0,0 -1,2 -3,2 C-5,2 -6,0 -6,-2 M6,-2 C6,-4 5,-6 3,-6 C1,-6 0,-4 0,-2 C0,0 1,2 3,2 C5,2 6,0 6,-2Z'
    case 's': // Spade
      return 'M0,-10 C-5,-10 -8,-6 -8,-2 C-8,1 -5,3 -2,3 L-3,8 L3,8 L2,3 C5,3 8,1 8,-2 C8,-6 5,-10 0,-10Z'
  }
}

