import { Stage, Layer, Ellipse, Text, Rect, Group } from 'react-konva'
import type { PokerTableProps } from '../../../lib/canvas'
import { 
  TABLE_COLORS, 
  TABLE_DIMENSIONS,
  SEAT_POSITIONS,
  POT_DIMENSIONS 
} from '../../../lib/canvas'
import { 
  getAllSeatPositions, 
  getTableCenter, 
  formatPot,
  calculateResponsiveScale 
} from '../../../lib/canvas'
import { PlayerSeat } from './PlayerSeat'

/**
 * Componente PokerTable - Mesa de poker 6-max renderizada en Canvas
 * Incluye mesa oval, 6 asientos, pot y community cards area
 */
export function PokerTable({ 
  tableState, 
  width = 800, 
  height = 500,
  onPlayerClick 
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
              text={formatPot(totalPot)}
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

        {/* Community cards placeholder */}
        <Group x={center.x} y={center.y + 50 * scale}>
          {tableState.communityCards.length > 0 ? (
            // Renderizar cartas comunitarias
            tableState.communityCards.map((card, index) => (
              <Group key={card} x={(index - 2) * 45 * scale}>
                <Rect
                  x={-18 * scale}
                  y={-25 * scale}
                  width={36 * scale}
                  height={50 * scale}
                  fill="white"
                  cornerRadius={4}
                  shadowColor="black"
                  shadowBlur={4}
                  shadowOpacity={0.3}
                />
                <Text
                  text={card}
                  fontSize={14 * scale}
                  fontFamily="JetBrains Mono, monospace"
                  fontStyle="bold"
                  fill={card.includes('h') || card.includes('d') ? '#EF4444' : '#1E293B'}
                  align="center"
                  width={36 * scale}
                  x={-18 * scale}
                  y={-5 * scale}
                />
              </Group>
            ))
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

