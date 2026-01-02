// Geometry utilities for positioning elements on the poker table

import type { Point, CanvasDimensions, ScaleConfig } from './types'
import type { SeatPosition } from './constants'
import { 
  SEAT_ANGLES, 
  TABLE_DIMENSIONS, 
  SEAT_DIMENSIONS,
  DEALER_BUTTON_DIMENSIONS 
} from './constants'

/**
 * Convierte grados a radianes
 */
export function degreesToRadians(degrees: number): number {
  return (degrees * Math.PI) / 180
}

/**
 * Calcula la posicion de un asiento en la mesa oval
 * @param position - Posicion del asiento (BTN, SB, etc.)
 * @param center - Centro de la mesa
 * @param radiusX - Radio X de la elipse
 * @param radiusY - Radio Y de la elipse
 * @param distanceFactor - Factor de distancia desde el centro (0-1)
 */
export function getSeatPosition(
  position: SeatPosition,
  center: Point,
  radiusX: number = TABLE_DIMENSIONS.radiusX,
  radiusY: number = TABLE_DIMENSIONS.radiusY,
  distanceFactor: number = SEAT_DIMENSIONS.distanceFromCenter
): Point {
  const angleDeg = SEAT_ANGLES[position]
  const angleRad = degreesToRadians(angleDeg)
  
  // Posicion en la elipse con factor de distancia
  const effectiveRadiusX = radiusX * distanceFactor
  const effectiveRadiusY = radiusY * distanceFactor
  
  return {
    x: center.x + effectiveRadiusX * Math.cos(angleRad),
    y: center.y + effectiveRadiusY * Math.sin(angleRad),
  }
}

/**
 * Calcula las posiciones de todos los asientos
 */
export function getAllSeatPositions(
  center: Point,
  radiusX: number = TABLE_DIMENSIONS.radiusX,
  radiusY: number = TABLE_DIMENSIONS.radiusY
): Record<SeatPosition, Point> {
  const positions: Partial<Record<SeatPosition, Point>> = {}
  
  for (const pos of Object.keys(SEAT_ANGLES) as SeatPosition[]) {
    positions[pos] = getSeatPosition(pos, center, radiusX, radiusY)
  }
  
  return positions as Record<SeatPosition, Point>
}

/**
 * Calcula la posicion del dealer button relativa al asiento
 */
export function getDealerButtonPosition(
  seatPosition: Point,
  seatAngle: number
): Point {
  const angleRad = degreesToRadians(seatAngle)
  const offset = SEAT_DIMENSIONS.avatarRadius + DEALER_BUTTON_DIMENSIONS.radius + 8
  
  // Posiciona el dealer button hacia el centro de la mesa
  return {
    x: seatPosition.x - offset * Math.cos(angleRad) * 0.5,
    y: seatPosition.y - offset * Math.sin(angleRad) * 0.5,
  }
}

/**
 * Calcula la escala y offset para hacer el canvas responsive
 */
export function calculateResponsiveScale(
  containerDimensions: CanvasDimensions,
  baseDimensions: CanvasDimensions = { 
    width: TABLE_DIMENSIONS.radiusX * 2 + 200, 
    height: TABLE_DIMENSIONS.radiusY * 2 + 200 
  }
): ScaleConfig {
  const scaleX = containerDimensions.width / baseDimensions.width
  const scaleY = containerDimensions.height / baseDimensions.height
  const scale = Math.min(scaleX, scaleY, 1.5) // Cap max scale at 1.5
  
  const scaledWidth = baseDimensions.width * scale
  const scaledHeight = baseDimensions.height * scale
  
  return {
    scale,
    offsetX: (containerDimensions.width - scaledWidth) / 2,
    offsetY: (containerDimensions.height - scaledHeight) / 2,
  }
}

/**
 * Obtiene el centro de la mesa basado en las dimensiones del canvas
 */
export function getTableCenter(dimensions: CanvasDimensions): Point {
  return {
    x: dimensions.width / 2,
    y: dimensions.height / 2,
  }
}

/**
 * Formatea un valor de stack para mostrar (ej: 1500 -> "1.5k", 100 -> "100")
 */
export function formatStack(amount: number): string {
  if (amount >= 1000000) {
    return `${(amount / 1000000).toFixed(1)}M`
  }
  if (amount >= 10000) {
    return `${(amount / 1000).toFixed(0)}k`
  }
  if (amount >= 1000) {
    return `${(amount / 1000).toFixed(1)}k`
  }
  return amount.toString()
}

/**
 * Formatea un valor de pot para mostrar
 */
export function formatPot(amount: number): string {
  if (amount === 0) return ''
  return `Pot: ${formatStack(amount)}`
}

