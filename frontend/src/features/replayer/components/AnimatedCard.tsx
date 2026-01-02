import { useEffect, useRef } from 'react'
import { Group } from 'react-konva'
import Konva from 'konva'
import { Card, CardBack } from './Card'
import type { CardNotation } from '../../../lib/canvas'

/**
 * Tipos de animacion disponibles
 */
export type CardAnimationType = 'deal' | 'flip' | 'slide' | 'none'

/**
 * Props para AnimatedCard
 */
interface AnimatedCardProps {
  /** Notacion de la carta */
  notation: CardNotation | string
  /** Posicion X final */
  x: number
  /** Posicion Y final */
  y: number
  /** Escala */
  scale?: number
  /** Tipo de animacion al montar */
  animation?: CardAnimationType
  /** Duracion de la animacion en ms */
  duration?: number
  /** Delay antes de iniciar la animacion en ms */
  delay?: number
  /** Si la carta esta boca abajo */
  faceDown?: boolean
  /** Callback cuando termina la animacion */
  onAnimationComplete?: () => void
  /** Callback al hacer click */
  onClick?: () => void
  /** Si la carta esta resaltada */
  highlighted?: boolean
}

/**
 * AnimatedCard - Carta con animaciones de entrada
 * Soporta animaciones de deal (desde centro), flip y slide
 */
export function AnimatedCard({
  notation,
  x,
  y,
  scale = 1,
  animation = 'deal',
  duration = 300,
  delay = 0,
  faceDown = false,
  onAnimationComplete,
  onClick,
  highlighted = false
}: AnimatedCardProps) {
  const groupRef = useRef<Konva.Group>(null)

  useEffect(() => {
    const group = groupRef.current
    if (!group || animation === 'none') return

    // Configurar estado inicial segun tipo de animacion
    switch (animation) {
      case 'deal': {
        // Deal desde el centro de la pantalla
        const stage = group.getStage()
        const centerX = stage ? stage.width() / 2 : x
        const centerY = stage ? stage.height() / 2 : y
        
        group.x(centerX)
        group.y(centerY)
        group.opacity(0)
        group.scale({ x: 0.1, y: 0.1 })
        
        // Animar hacia posicion final
        setTimeout(() => {
          group.to({
            x,
            y,
            scaleX: 1,
            scaleY: 1,
            opacity: 1,
            duration: duration / 1000,
            easing: Konva.Easings.EaseOut,
            onFinish: onAnimationComplete,
          })
        }, delay)
        break
      }

      case 'flip': {
        // Flip horizontal (rotar sobre eje Y)
        group.x(x)
        group.y(y)
        group.scaleX(0)
        group.scaleY(1)
        group.opacity(1)
        
        setTimeout(() => {
          group.to({
            scaleX: 1,
            duration: duration / 1000,
            easing: Konva.Easings.EaseInOut,
            onFinish: onAnimationComplete,
          })
        }, delay)
        break
      }

      case 'slide': {
        // Slide desde arriba
        group.x(x)
        group.y(y - 100)
        group.opacity(0)
        
        setTimeout(() => {
          group.to({
            y,
            opacity: 1,
            duration: duration / 1000,
            easing: Konva.Easings.EaseOut,
            onFinish: onAnimationComplete,
          })
        }, delay)
        break
      }
    }
  }, [animation, x, y, duration, delay, onAnimationComplete])

  return (
    <Group ref={groupRef}>
      {faceDown ? (
        <CardBack scale={scale} onClick={onClick} />
      ) : (
        <Card 
          notation={notation} 
          scale={scale} 
          onClick={onClick}
          highlighted={highlighted}
        />
      )}
    </Group>
  )
}

/**
 * Props para AnimatedCardGroup
 */
interface AnimatedCardGroupProps {
  /** Cartas a renderizar */
  cards: Array<{
    notation: CardNotation | string
    x: number
    y: number
    faceDown?: boolean
  }>
  /** Escala para todas las cartas */
  scale?: number
  /** Tipo de animacion */
  animation?: CardAnimationType
  /** Duracion de cada animacion */
  duration?: number
  /** Delay entre cartas en ms */
  staggerDelay?: number
  /** Delay inicial antes de empezar */
  initialDelay?: number
  /** Callback cuando todas las animaciones terminan */
  onAllAnimationsComplete?: () => void
}

/**
 * AnimatedCardGroup - Grupo de cartas con animaciones escalonadas
 * Util para renderizar flop, turn, river con timing secuencial
 */
export function AnimatedCardGroup({
  cards,
  scale = 1,
  animation = 'deal',
  duration = 300,
  staggerDelay = 100,
  initialDelay = 0,
  onAllAnimationsComplete,
}: AnimatedCardGroupProps) {
  const completedCount = useRef(0)

  const handleCardComplete = () => {
    completedCount.current += 1
    if (completedCount.current === cards.length && onAllAnimationsComplete) {
      onAllAnimationsComplete()
    }
  }

  return (
    <Group>
      {cards.map((card, index) => (
        <AnimatedCard
          key={`${card.notation}-${index}`}
          notation={card.notation}
          x={card.x}
          y={card.y}
          scale={scale}
          animation={animation}
          duration={duration}
          delay={initialDelay + index * staggerDelay}
          faceDown={card.faceDown}
          onAnimationComplete={handleCardComplete}
        />
      ))}
    </Group>
  )
}

