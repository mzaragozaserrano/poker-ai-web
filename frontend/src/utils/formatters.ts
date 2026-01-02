import type { AmountFormat } from '../hooks/useAmountFormat'

/**
 * Formatea una cantidad según el formato especificado
 * @param amount Cantidad en centavos (100 = 1 EUR)
 * @param format Formato: 'bb' o 'eur'
 * @param bigBlind Big blind en centavos (ej: 200 para 2 EUR)
 * @returns String formateado
 */
export const formatAmount = (
  amount: number,
  format: AmountFormat,
  bigBlind: number
): string => {
  if (format === 'eur') {
    const eur = amount / 100
    return `${eur.toFixed(2)}€`
  }

  // Formato BB
  const bb = amount / bigBlind
  // Mostrar decimales solo si hay parte decimal significativa
  if (Math.abs(bb - Math.round(bb)) < 0.01) {
    return `${Math.round(bb)}bb`
  }
  return `${bb.toFixed(1)}bb`
}

/**
 * Convierte una cantidad de EUR a BB
 * @param eurAmount Cantidad en EUR
 * @param bigBlind Big blind en EUR
 * @returns Cantidad en BB
 */
export const eurToBB = (eurAmount: number, bigBlind: number): number => {
  return eurAmount / bigBlind
}

/**
 * Convierte una cantidad de BB a EUR
 * @param bbAmount Cantidad en BB
 * @param bigBlind Big blind en EUR
 * @returns Cantidad en EUR
 */
export const bbToEur = (bbAmount: number, bigBlind: number): number => {
  return bbAmount * bigBlind
}

/**
 * Formatea un stack/bote para visualización
 * Combina la lógica de formatAmount con validaciones
 */
export const formatStack = (
  amount: number,
  format: AmountFormat,
  bigBlind: number
): string => {
  if (amount === 0) return '0'
  return formatAmount(Math.round(amount), format, bigBlind)
}

