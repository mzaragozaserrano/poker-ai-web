/**
 * features/stats/utils/rangeUtils.ts
 * Utilidades para manejo de rangos y colores
 */

import type { RangeAction, Frequency, HandNotation, RangeData } from '../../../types/ranges'
import { ACTION_COLORS } from '../../../types/ranges'

// ============================================================================
// INTERPOLACIÓN DE COLORES
// ============================================================================

/**
 * Convierte un color hex a RGB
 */
function hexToRgb(hex: string): { r: number; g: number; b: number } {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16)
      }
    : { r: 0, g: 0, b: 0 }
}

/**
 * Convierte RGB a string rgba
 */
function rgbToRgba(r: number, g: number, b: number, alpha: number): string {
  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

/**
 * Interpola entre dos colores basado en un factor (0.0 - 1.0)
 */
function interpolateColor(color1: string, color2: string, factor: number): string {
  const c1 = hexToRgb(color1)
  const c2 = hexToRgb(color2)

  const r = Math.round(c1.r + (c2.r - c1.r) * factor)
  const g = Math.round(c1.g + (c2.g - c1.g) * factor)
  const b = Math.round(c1.b + (c2.b - c1.b) * factor)

  return `rgb(${r}, ${g}, ${b})`
}

// ============================================================================
// MAPAS DE CALOR
// ============================================================================

/**
 * Genera color de mapa de calor basado en acción y frecuencia
 * Usa opacidad para representar la frecuencia (0.0 = transparent, 1.0 = opaco)
 */
export function getHeatmapColor(action: RangeAction, frequency: Frequency): string {
  const baseColor = ACTION_COLORS[action]
  const rgb = hexToRgb(baseColor)

  // Mínimo 10% de opacidad para que sea visible
  const alpha = Math.max(0.1, Math.min(1.0, frequency))

  return rgbToRgba(rgb.r, rgb.g, rgb.b, alpha)
}

/**
 * Genera color de mapa de calor con interpolación de colores
 * Útil para gradientes más complejos
 */
export function getHeatmapColorGradient(
  minColor: string,
  maxColor: string,
  frequency: Frequency
): string {
  return interpolateColor(minColor, maxColor, frequency)
}

/**
 * Obtiene el color de texto apropiado según el fondo
 * Para mantener buen contraste
 */
export function getTextColor(backgroundColor: string, frequency: Frequency): string {
  // Si la frecuencia es alta (>0.5), usar texto blanco
  // Si es baja, usar texto gris claro
  return frequency > 0.5 ? '#FFFFFF' : '#CBD5E1' // white : slate-300
}

// ============================================================================
// ANÁLISIS DE RANGOS
// ============================================================================

/**
 * Calcula la frecuencia total de una mano (suma de todas las acciones)
 */
export function getTotalFrequency(entries: Array<{ frequency: Frequency }>): Frequency {
  return entries.reduce((sum, entry) => sum + entry.frequency, 0)
}

/**
 * Obtiene la acción principal (la de mayor frecuencia)
 */
export function getPrimaryAction<T extends { action: RangeAction; frequency: Frequency }>(
  entries: T[]
): T | null {
  if (entries.length === 0) return null
  return entries.reduce((max, entry) => (entry.frequency > max.frequency ? entry : max), entries[0])
}

/**
 * Calcula el porcentaje de fold implícito
 * fold% = 100% - suma(frecuencias de acciones)
 */
export function getImplicitFoldFrequency(entries: Array<{ frequency: Frequency }>): Frequency {
  const totalFrequency = getTotalFrequency(entries)
  return Math.max(0, 1.0 - totalFrequency)
}

/**
 * Verifica si una mano está en el rango (tiene al menos una acción con frecuencia > 0)
 */
export function isHandInRange(range: RangeData, hand: HandNotation): boolean {
  const entries = range[hand]
  return entries ? getTotalFrequency(entries) > 0 : false
}

/**
 * Cuenta cuántas manos están en el rango
 */
export function countHandsInRange(range: RangeData): number {
  return Object.keys(range).filter(hand => isHandInRange(range, hand)).length
}

/**
 * Calcula el porcentaje del rango (0-100%)
 * Total de 169 combinaciones posibles
 */
export function getRangePercentage(range: RangeData): number {
  const handsInRange = countHandsInRange(range)
  return (handsInRange / 169) * 100
}

// ============================================================================
// FORMATEO Y DISPLAY
// ============================================================================

/**
 * Formatea una frecuencia como porcentaje
 */
export function formatFrequency(frequency: Frequency): string {
  return `${(frequency * 100).toFixed(1)}%`
}

/**
 * Formatea múltiples acciones para tooltip
 * Ejemplo: "RAISE: 70%, CALL: 20%, FOLD: 10%"
 */
export function formatActionBreakdown(
  entries: Array<{ action: RangeAction; frequency: Frequency }>
): string {
  const lines = entries.map(entry => `${entry.action}: ${formatFrequency(entry.frequency)}`)
  const foldFreq = getImplicitFoldFrequency(entries)

  if (foldFreq > 0) {
    lines.push(`FOLD: ${formatFrequency(foldFreq)}`)
  }

  return lines.join('\n')
}

/**
 * Genera descripción legible de una estrategia mixta
 */
export function describeStrategy(
  entries: Array<{ action: RangeAction; frequency: Frequency }>
): string {
  if (entries.length === 0) return 'Fold 100%'
  if (entries.length === 1 && entries[0].frequency === 1.0) {
    return `${entries[0].action} 100%`
  }

  const primary = getPrimaryAction(entries)
  if (!primary) return 'Fold 100%'

  const foldFreq = getImplicitFoldFrequency(entries)

  if (entries.length === 1 && foldFreq > 0) {
    return `${primary.action} ${formatFrequency(primary.frequency)}, Fold ${formatFrequency(foldFreq)}`
  }

  return `Mixed: ${entries.map(e => `${e.action} ${formatFrequency(e.frequency)}`).join(', ')}`
}

// ============================================================================
// VALIDACIÓN
// ============================================================================

/**
 * Valida que la suma de frecuencias no exceda 1.0
 */
export function validateFrequencies(entries: Array<{ frequency: Frequency }>): boolean {
  const total = getTotalFrequency(entries)
  return total <= 1.0
}

/**
 * Valida que todas las frecuencias estén en el rango [0.0, 1.0]
 */
export function validateFrequencyRange(frequency: Frequency): boolean {
  return frequency >= 0.0 && frequency <= 1.0
}

/**
 * Valida un rango completo
 */
export function validateRange(range: RangeData): { valid: boolean; errors: string[] } {
  const errors: string[] = []

  for (const [hand, entries] of Object.entries(range)) {
    // Validar frecuencias individuales
    for (const entry of entries) {
      if (!validateFrequencyRange(entry.frequency)) {
        errors.push(`${hand}: Frecuencia fuera de rango (${entry.frequency})`)
      }
    }

    // Validar suma de frecuencias
    if (!validateFrequencies(entries)) {
      const total = getTotalFrequency(entries)
      errors.push(`${hand}: Suma de frecuencias excede 1.0 (${total.toFixed(2)})`)
    }
  }

  return {
    valid: errors.length === 0,
    errors
  }
}

