# Sistema de Renderizado de Cartas - Hand Replayer

## Descripción General

Sistema completo de renderizado de cartas de poker en Canvas usando React-Konva. Incluye componentes base, animaciones y utilidades para parsing de notación de cartas.

## Componentes

### Card

Renderiza una carta individual de poker con valor, palo y colores correctos.

**Props:**
- `notation`: Notación de carta (ej: "Ah", "Kd", "Ts")
- `x`, `y`: Posición en el canvas
- `scale`: Escala de renderizado (default: 1)
- `onClick`: Callback al hacer click
- `highlighted`: Si la carta está resaltada

**Ejemplo:**
```tsx
<Card notation="Ah" x={100} y={100} scale={1} />
```

**Características:**
- Valor en esquinas superior izquierda e inferior derecha
- Símbolo de palo grande centrado
- Colores correctos: rojo para hearts/diamonds, negro para clubs/spades
- Sombras y bordes redondeados para profundidad visual

### CardBack

Renderiza el reverso de una carta (cartas boca abajo).

**Props:**
- `x`, `y`: Posición en el canvas
- `scale`: Escala de renderizado
- `onClick`: Callback al hacer click

**Uso:**
```tsx
<CardBack x={100} y={100} scale={1} />
```

Usado para cartas de oponentes que no son visibles.

### AnimatedCard

Carta con animaciones de entrada.

**Props:**
- `notation`: Notación de la carta
- `x`, `y`: Posición final después de la animación
- `scale`: Escala
- `animation`: Tipo de animación ('deal', 'flip', 'slide', 'none')
- `duration`: Duración en ms (default: 300)
- `delay`: Delay antes de iniciar en ms
- `faceDown`: Si la carta está boca abajo
- `onAnimationComplete`: Callback cuando termina la animación

**Tipos de Animación:**

1. **'deal'**: Carta aparece desde el centro con scale y fade
2. **'flip'**: Rotación horizontal (efecto de voltear)
3. **'slide'**: Desliza desde arriba con fade
4. **'none'**: Sin animación, aparece directamente

**Ejemplo:**
```tsx
<AnimatedCard 
  notation="Kh" 
  x={200} 
  y={150} 
  animation="deal"
  duration={300}
  delay={100}
/>
```

### AnimatedCardGroup

Grupo de cartas con animaciones escalonadas.

**Props:**
- `cards`: Array de objetos {notation, x, y, faceDown?}
- `scale`: Escala para todas las cartas
- `animation`: Tipo de animación
- `duration`: Duración de cada animación
- `staggerDelay`: Delay entre cartas (default: 100ms)
- `initialDelay`: Delay antes de empezar
- `onAllAnimationsComplete`: Callback cuando todas terminan

**Uso típico (renderizar flop):**
```tsx
<AnimatedCardGroup
  cards={[
    { notation: 'Ah', x: 100, y: 200 },
    { notation: 'Kd', x: 150, y: 200 },
    { notation: '7c', x: 200, y: 200 },
  ]}
  animation="deal"
  staggerDelay={100}
/>
```

## Utilidades (lib/canvas/cards.ts)

### Tipos

```typescript
type CardValue = '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A'
type CardSuit = 'h' | 'd' | 'c' | 's'
type CardNotation = `${CardValue}${CardSuit}`

interface ParsedCard {
  value: CardValue
  suit: CardSuit
  display: string        // "10" para T, resto igual
  color: 'red' | 'black'
  suitSymbol: string     // ♥ ♦ ♣ ♠
}
```

### Funciones

**`isValidCard(notation: string): boolean`**
Valida si una string es una notación de carta válida.

**`parseCard(notation: string): ParsedCard`**
Parsea una notación a objeto estructurado. Lanza error si es inválida.

**`parseCards(notations: string[]): ParsedCard[]`**
Parsea múltiples cartas, ignora inválidas.

**`getCardColor(suit: CardSuit): string`**
Obtiene el color hex para renderizado.

**`formatCardDisplay(notation: string): string`**
Formatea para display con símbolo (ej: "A♥").

### Constantes

```typescript
SUIT_SYMBOLS = { h: '♥', d: '♦', c: '♣', s: '♠' }
SUIT_COLORS = { h: 'red', d: 'red', c: 'black', s: 'black' }
CARD_DIMENSIONS = {
  width: 40,
  height: 56,
  cornerRadius: 4,
  cornerFontSize: 14,
  suitFontSize: 24,
  padding: 4,
}
CARD_COLORS = {
  red: '#EF4444',
  black: '#1E293B',
  background: '#FFFFFF',
  border: '#E2E8F0',
  backPattern: '#1E293B',
  backBackground: '#64748B',
}
```

## Integración en PokerTable

### Community Cards

Las community cards se renderizan automáticamente desde `tableState.communityCards`:

```tsx
<PokerTable
  tableState={{
    communityCards: ['Ah', 'Kd', '7c'],
    // ... otros campos
  }}
/>
```

Las cartas aparecen centradas en la mesa, espaciadas correctamente.

### Hole Cards de Jugadores

Los jugadores pueden tener cartas visibles en `PlayerState.cards`:

```tsx
{
  id: '1',
  name: 'thesmoy',
  position: 'BTN',
  cards: ['As', 'Kh'],  // Cartas visibles (hero)
  // ... otros campos
}

{
  id: '2',
  name: 'Villain',
  position: 'SB',
  cards: ['??', '??'],  // Cartas no visibles (renderiza CardBack)
}
```

- Cartas válidas (ej: "Ah") → renderiza carta completa
- Notación inválida (ej: "??") → renderiza CardBack
- Sin cards → no renderiza nada

## Notación de Cartas

### Formato

`{VALOR}{PALO}`

### Valores
- Números: `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`
- Ten: `T` (se muestra como "10")
- Figuras: `J`, `Q`, `K`, `A`

### Palos
- Hearts (corazones): `h` → ♥ (rojo)
- Diamonds (diamantes): `d` → ♦ (rojo)
- Clubs (tréboles): `c` → ♣ (negro)
- Spades (picas): `s` → ♠ (negro)

### Ejemplos
- `Ah` = As de corazones
- `Kd` = Rey de diamantes
- `Ts` = 10 de picas
- `7c` = 7 de tréboles

## Performance

### Optimizaciones Implementadas

1. **Símbolos Unicode**: Uso de caracteres Unicode (♥♦♣♠) en lugar de paths SVG complejos para mejor performance
2. **Componentes ligeros**: Uso de primitivas de Konva (Rect, Text) en lugar de Groups complejos
3. **Animaciones GPU**: Transformaciones CSS aceleradas por GPU
4. **Escalado responsive**: Todas las dimensiones se escalan proporcionalmente

### Target: 60 FPS

- Animaciones usan `Konva.Tween` con `requestAnimationFrame`
- Duración típica: 300ms por carta
- Stagger delay: 100ms entre cartas del flop
- Easing: `EaseOut` para naturalidad

### Testing

Probado con:
- 5 community cards
- 6 jugadores con hole cards simultáneas
- Animaciones escalonadas
- Responsive (400px - 1920px width)
- Escalas: 0.5x - 2x

## Próximas Mejoras

1. **Variantes de CardBack**: Diferentes diseños de reverso
2. **Highlights dinámicos**: Resaltar cartas según equity o outs
3. **Burn cards**: Renderizar cartas quemadas
4. **Transiciones de street**: Animaciones especiales al cambiar de calle
5. **Mobile touch**: Gestos de swipe para navegar manos

