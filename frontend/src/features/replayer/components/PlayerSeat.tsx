import { Group, Circle, Text, Rect } from 'react-konva'
import type { PlayerSeatProps } from '../../../lib/canvas'
import { 
  PLAYER_COLORS, 
  SEAT_DIMENSIONS, 
  DEALER_BUTTON_COLORS,
  DEALER_BUTTON_DIMENSIONS 
} from '../../../lib/canvas'
import { formatStack } from '../../../lib/canvas'

/**
 * Componente PlayerSeat - Renderiza un asiento de jugador en el canvas
 * Incluye avatar, nombre, stack y opcionalmente el dealer button
 */
export function PlayerSeat({ 
  player, 
  position, 
  isDealer,
  scale = 1,
  onClick 
}: PlayerSeatProps) {
  const { avatarRadius } = SEAT_DIMENSIONS
  const scaledRadius = avatarRadius * scale

  const avatarColor = player.isHero 
    ? PLAYER_COLORS.avatarHero 
    : PLAYER_COLORS.avatarDefault

  const opacity = player.isFolded ? 0.4 : 1

  return (
    <Group
      x={position.x}
      y={position.y}
      opacity={opacity}
      onClick={onClick}
      onTap={onClick}
    >
      {/* Avatar circle */}
      <Circle
        radius={scaledRadius}
        fill={avatarColor}
        stroke={player.isActive ? PLAYER_COLORS.activeBorder : 'transparent'}
        strokeWidth={player.isActive ? 3 : 0}
        shadowColor="black"
        shadowBlur={8}
        shadowOpacity={0.3}
      />

      {/* Player initial in avatar */}
      <Text
        text={player.name.charAt(0).toUpperCase()}
        fontSize={scaledRadius * 0.8}
        fontFamily="Inter, system-ui, sans-serif"
        fontStyle="bold"
        fill={PLAYER_COLORS.nameText}
        align="center"
        verticalAlign="middle"
        width={scaledRadius * 2}
        height={scaledRadius * 2}
        offsetX={scaledRadius}
        offsetY={scaledRadius}
      />

      {/* Name background */}
      <Rect
        x={-SEAT_DIMENSIONS.width / 2 * scale}
        y={scaledRadius + 4}
        width={SEAT_DIMENSIONS.width * scale}
        height={20 * scale}
        fill={PLAYER_COLORS.positionBadge}
        cornerRadius={4}
        opacity={0.9}
      />

      {/* Player name */}
      <Text
        text={player.name.length > 10 ? player.name.slice(0, 9) + '...' : player.name}
        fontSize={12 * scale}
        fontFamily="Inter, system-ui, sans-serif"
        fill={PLAYER_COLORS.nameText}
        align="center"
        width={SEAT_DIMENSIONS.width * scale}
        x={-SEAT_DIMENSIONS.width / 2 * scale}
        y={scaledRadius + 7}
      />

      {/* Stack display */}
      <Text
        text={formatStack(player.stack)}
        fontSize={11 * scale}
        fontFamily="JetBrains Mono, monospace"
        fill={PLAYER_COLORS.stackText}
        align="center"
        width={SEAT_DIMENSIONS.width * scale}
        x={-SEAT_DIMENSIONS.width / 2 * scale}
        y={scaledRadius + 26}
      />

      {/* Position badge */}
      <Rect
        x={scaledRadius * 0.5}
        y={-scaledRadius - 8}
        width={28 * scale}
        height={16 * scale}
        fill={PLAYER_COLORS.positionBadge}
        cornerRadius={3}
      />
      <Text
        text={player.position}
        fontSize={9 * scale}
        fontFamily="Inter, system-ui, sans-serif"
        fontStyle="bold"
        fill={PLAYER_COLORS.positionText}
        align="center"
        width={28 * scale}
        x={scaledRadius * 0.5}
        y={-scaledRadius - 5}
      />

      {/* Current bet (if any) */}
      {player.currentBet && player.currentBet > 0 && (
        <Group y={-scaledRadius - 30}>
          <Circle
            radius={10 * scale}
            fill="#EF4444"
            x={0}
          />
          <Text
            text={formatStack(player.currentBet)}
            fontSize={10 * scale}
            fontFamily="JetBrains Mono, monospace"
            fill="white"
            align="center"
            width={60 * scale}
            x={12 * scale}
            y={-5 * scale}
          />
        </Group>
      )}

      {/* Dealer button */}
      {isDealer && (
        <Group x={-scaledRadius - 10} y={-scaledRadius * 0.3}>
          <Circle
            radius={DEALER_BUTTON_DIMENSIONS.radius * scale}
            fill={DEALER_BUTTON_COLORS.background}
            stroke={DEALER_BUTTON_COLORS.border}
            strokeWidth={2}
            shadowColor="black"
            shadowBlur={4}
            shadowOpacity={0.3}
          />
          <Text
            text="D"
            fontSize={DEALER_BUTTON_DIMENSIONS.fontSize * scale}
            fontFamily="Inter, system-ui, sans-serif"
            fontStyle="bold"
            fill={DEALER_BUTTON_COLORS.text}
            align="center"
            verticalAlign="middle"
            width={DEALER_BUTTON_DIMENSIONS.radius * 2 * scale}
            height={DEALER_BUTTON_DIMENSIONS.radius * 2 * scale}
            offsetX={DEALER_BUTTON_DIMENSIONS.radius * scale}
            offsetY={DEALER_BUTTON_DIMENSIONS.radius * scale}
          />
        </Group>
      )}
    </Group>
  )
}

