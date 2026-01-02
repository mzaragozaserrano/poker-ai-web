import { Group, Rect, Text } from 'react-konva'
import { parseCard, CARD_DIMENSIONS, CARD_COLORS, type CardNotation } from '../../../lib/canvas'

/**
 * Props para el componente Card
 */
interface CardProps {
  /** Notacion de la carta (ej: "Ah", "Kd") */
  notation: CardNotation | string
  /** Posicion X */
  x?: number
  /** Posicion Y */
  y?: number
  /** Escala de renderizado */
  scale?: number
  /** Callback al hacer click */
  onClick?: () => void
  /** Si la carta esta resaltada/seleccionada */
  highlighted?: boolean
}

/**
 * Componente Card - Renderiza una carta de poker en Canvas
 * Incluye valor, palo con colores correctos y sombras
 */
export function Card({ 
  notation, 
  x = 0, 
  y = 0, 
  scale = 1,
  onClick,
  highlighted = false
}: CardProps) {
  const card = parseCard(notation)
  const { width, height, cornerRadius, cornerFontSize, suitFontSize, padding } = CARD_DIMENSIONS
  
  const scaledWidth = width * scale
  const scaledHeight = height * scale
  const scaledPadding = padding * scale
  const scaledCornerFontSize = cornerFontSize * scale
  const scaledSuitFontSize = suitFontSize * scale

  const textColor = card.color === 'red' ? CARD_COLORS.red : CARD_COLORS.black

  return (
    <Group
      x={x}
      y={y}
      onClick={onClick}
      onTap={onClick}
    >
      {/* Card background */}
      <Rect
        width={scaledWidth}
        height={scaledHeight}
        fill={CARD_COLORS.background}
        cornerRadius={cornerRadius * scale}
        stroke={highlighted ? '#10B981' : CARD_COLORS.border}
        strokeWidth={highlighted ? 2 : 1}
        shadowColor="black"
        shadowBlur={6 * scale}
        shadowOpacity={0.25}
        shadowOffsetX={1 * scale}
        shadowOffsetY={2 * scale}
      />

      {/* Valor en esquina superior izquierda */}
      <Text
        text={card.display}
        x={scaledPadding}
        y={scaledPadding}
        fontSize={scaledCornerFontSize}
        fontFamily="JetBrains Mono, monospace"
        fontStyle="bold"
        fill={textColor}
        align="left"
      />

      {/* Palo pequeño en esquina superior izquierda (debajo del valor) */}
      <Text
        text={card.suitSymbol}
        x={scaledPadding}
        y={scaledPadding + scaledCornerFontSize + 1}
        fontSize={scaledCornerFontSize * 0.8}
        fontFamily="Arial, sans-serif"
        fill={textColor}
        align="left"
      />

      {/* Simbolo de palo central grande */}
      <Text
        text={card.suitSymbol}
        x={scaledWidth / 2}
        y={scaledHeight / 2}
        fontSize={scaledSuitFontSize}
        fontFamily="Arial, sans-serif"
        fill={textColor}
        align="center"
        verticalAlign="middle"
        offsetX={scaledSuitFontSize / 4}
        offsetY={scaledSuitFontSize / 2}
      />

      {/* Valor en esquina inferior derecha (invertido) */}
      <Text
        text={card.display}
        x={scaledWidth - scaledPadding}
        y={scaledHeight - scaledPadding}
        fontSize={scaledCornerFontSize}
        fontFamily="JetBrains Mono, monospace"
        fontStyle="bold"
        fill={textColor}
        align="right"
        verticalAlign="bottom"
        rotation={180}
        offsetX={scaledCornerFontSize}
        offsetY={scaledCornerFontSize}
      />

      {/* Palo pequeño en esquina inferior derecha (encima del valor, invertido) */}
      <Text
        text={card.suitSymbol}
        x={scaledWidth - scaledPadding}
        y={scaledHeight - scaledPadding - scaledCornerFontSize - 1}
        fontSize={scaledCornerFontSize * 0.8}
        fontFamily="Arial, sans-serif"
        fill={textColor}
        align="right"
        rotation={180}
        offsetX={scaledCornerFontSize * 0.8}
        offsetY={scaledCornerFontSize * 0.8}
      />
    </Group>
  )
}

/**
 * Props para CardBack
 */
interface CardBackProps {
  /** Posicion X */
  x?: number
  /** Posicion Y */
  y?: number
  /** Escala de renderizado */
  scale?: number
  /** Callback al hacer click */
  onClick?: () => void
}

/**
 * Componente CardBack - Renderiza el reverso de una carta
 * Usado para cartas de oponentes que no son visibles
 */
export function CardBack({ 
  x = 0, 
  y = 0, 
  scale = 1,
  onClick
}: CardBackProps) {
  const { width, height, cornerRadius } = CARD_DIMENSIONS
  
  const scaledWidth = width * scale
  const scaledHeight = height * scale

  return (
    <Group
      x={x}
      y={y}
      onClick={onClick}
      onTap={onClick}
    >
      {/* Card background border */}
      <Rect
        width={scaledWidth}
        height={scaledHeight}
        fill={CARD_COLORS.backBackground}
        cornerRadius={cornerRadius * scale}
        stroke={CARD_COLORS.border}
        strokeWidth={1}
        shadowColor="black"
        shadowBlur={6 * scale}
        shadowOpacity={0.25}
        shadowOffsetX={1 * scale}
        shadowOffsetY={2 * scale}
      />

      {/* Inner pattern border */}
      <Rect
        x={3 * scale}
        y={3 * scale}
        width={(width - 6) * scale}
        height={(height - 6) * scale}
        stroke={CARD_COLORS.backPattern}
        strokeWidth={1.5}
        cornerRadius={(cornerRadius - 1) * scale}
      />

      {/* Diagonal lines pattern */}
      <Rect
        x={5 * scale}
        y={5 * scale}
        width={(width - 10) * scale}
        height={(height - 10) * scale}
        fill={CARD_COLORS.backPattern}
        opacity={0.15}
        cornerRadius={(cornerRadius - 2) * scale}
      />

      {/* Center decoration */}
      <Rect
        x={(width / 2 - 6) * scale}
        y={(height / 2 - 8) * scale}
        width={12 * scale}
        height={16 * scale}
        fill={CARD_COLORS.backPattern}
        opacity={0.3}
        cornerRadius={2 * scale}
      />
    </Group>
  )
}

