import { Stage, Layer, Ellipse, Text, Rect, Group } from 'react-konva'
import type { PokerTableProps } from '../../../lib/canvas'
import { 
  TABLE_COLORS, 
  TABLE_DIMENSIONS,
  SEAT_POSITIONS,
  POT_DIMENSIONS,
  CARD_DIMENSIONS 
} from '../../../lib/canvas'
import { 
  getAllSeatPositions, 
  getTableCenter, 
  calculateResponsiveScale,
  isValidCard 
} from '../../../lib/canvas'
import { formatStack } from '../../../utils/formatters'
import { PlayerSeat } from './PlayerSeat'
import { Card } from './Card'

/**
 * Componente PokerTable - Mesa de poker 6-max renderizada en Canvas
 * Incluye mesa oval, 6 asientos, pot y community cards area
 */
export function PokerTable({ 
  tableState, 
  width = 800, 
  height = 500,
  onPlayerClick,
  amountFormat = 'bb',
  bigBlind = 200
}: PokerTableProps) {
  const { scale } = calculateResponsiveScale(
    { width, height },
    { width: 800, height: 500 }
  )

  const center = getTableCenter({ width, height })
  const seatPositions = getAllSeatPositions(
    center,
    TABLE_DIMENSIONS.radiusX * scale,
    TABLE_DIMENSIONS.radiusY * scale
  )

  const { radiusX, radiusY, borderWidth, railWidth } = TABLE_DIMENSIONS
  const scaledRadiusX = radiusX * scale
  const scaledRadiusY = radiusY * scale
  const scaledBorderWidth = borderWidth * scale
  const scaledRailWidth = railWidth * scale

  const totalPot = tableState.pot.main + (tableState.pot.side?.reduce((a, b) => a + b, 0) ?? 0)

  return (
    <Stage width={width} height={height}>
      <Layer>
        {/* Mesa exterior (borde de madera) */}
        <Ellipse
          x={center.x}
          y={center.y}
          radiusX={scaledRadiusX + scaledBorderWidth}
          radiusY={scaledRadiusY + scaledBorderWidth}
          fill={TABLE_COLORS.border}
          shadowColor="black"
          shadowBlur={20}
          shadowOpacity={0.5}
          shadowOffsetY={5}
        />

        {/* Rail interior */}
        <Ellipse
          x={center.x}
          y={center.y}
          radiusX={scaledRadiusX + scaledRailWidth}
          radiusY={scaledRadiusY + scaledRailWidth}
          fill={TABLE_COLORS.rail}
        />

        {/* Felt (tapete verde) */}
        <Ellipse
          x={center.x}
          y={center.y}
          radiusX={scaledRadiusX}
          radiusY={scaledRadiusY}
          fillLinearGradientStartPoint={{ x: 0, y: -scaledRadiusY }}
          fillLinearGradientEndPoint={{ x: 0, y: scaledRadiusY }}
          fillLinearGradientColorStops={[
            0, TABLE_COLORS.feltGradientStart,
            1, TABLE_COLORS.feltGradientEnd
          ]}
        />

        {/* Linea decorativa del felt */}
        <Ellipse
          x={center.x}
          y={center.y}
          radiusX={scaledRadiusX * 0.7}
          radiusY={scaledRadiusY * 0.7}
          stroke="rgba(255, 255, 255, 0.08)"
          strokeWidth={2}
        />

        {/* Pot area */}
        <Group x={center.x} y={center.y}>
          {/* Pot background */}
          <Rect
            x={-60 * scale}
            y={-20 * scale}
            width={120 * scale}
            height={40 * scale}
            fill={TABLE_COLORS.potArea}
            cornerRadius={8}
          />
          
          {/* Pot amount */}
          {totalPot > 0 && (
            <Text
              text={formatStack(totalPot, amountFormat, bigBlind)}
              fontSize={POT_DIMENSIONS.fontSize * scale}
              fontFamily="JetBrains Mono, monospace"
              fontStyle="bold"
              fill="#FBBF24"
              align="center"
              width={120 * scale}
              x={-60 * scale}
              y={-8 * scale}
            />
          )}
        </Group>

        {/* Community cards */}
        <Group x={center.x} y={center.y + 50 * scale}>
          {tableState.communityCards.length > 0 ? (
            // Renderizar cartas comunitarias usando componente Card
            tableState.communityCards.map((card, index) => {
              const cardSpacing = (CARD_DIMENSIONS.width + 5) * scale
              const totalWidth = tableState.communityCards.length * cardSpacing
              const startX = -totalWidth / 2 + (CARD_DIMENSIONS.width * scale) / 2
              
              return isValidCard(card) ? (
                <Card
                  key={`community-${card}-${index}`}
                  notation={card}
                  x={startX + index * cardSpacing}
                  y={-CARD_DIMENSIONS.height * scale / 2}
                  scale={scale}
                />
              ) : null
            })
          ) : (
            // Placeholder cuando no hay cartas
            <Text
              text={tableState.currentStreet === 'preflop' ? '' : 'No cards'}
              fontSize={12 * scale}
              fontFamily="Inter, system-ui, sans-serif"
              fill="rgba(255, 255, 255, 0.3)"
              align="center"
              width={200 * scale}
              x={-100 * scale}
            />
          )}
        </Group>

        {/* Street indicator */}
        <Text
          text={tableState.currentStreet.toUpperCase()}
          fontSize={10 * scale}
          fontFamily="Inter, system-ui, sans-serif"
          fontStyle="bold"
          fill="rgba(255, 255, 255, 0.4)"
          x={center.x - 30 * scale}
          y={center.y - 60 * scale}
          width={60 * scale}
          align="center"
        />

        {/* Renderizar asientos de jugadores */}
        {SEAT_POSITIONS.map((position) => {
          const player = tableState.players.find(p => p.position === position)
          if (!player) return null

          return (
            <PlayerSeat
              key={player.id}
              player={player}
              position={seatPositions[position]}
              isDealer={tableState.dealerPosition === position}
              scale={scale}
              onClick={() => onPlayerClick?.(player.id)}
            />
          )
        })}
      </Layer>
    </Stage>
  )
}

