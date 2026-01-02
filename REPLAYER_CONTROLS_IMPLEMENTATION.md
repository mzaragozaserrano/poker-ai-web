# Controles de Reproducción del Hand Replayer - Issue #44

## Resumen

Se ha implementado un sistema completo de controles de reproducción para el Hand Replayer con una máquina de estados robusta, componentes interactivos y timeline visual de acciones.

## Arquitectura

### 1. Máquina de Estados (`useReplayerState.ts`)

Hook personalizado que maneja la lógica de reproducción con estados y transiciones:

**Estados:**
- `idle`: Mano cargada, no reproduciendo
- `playing`: Reproducción automática en curso
- `paused`: Pausado en una acción específica
- `finished`: Mano completada

**Acciones Disponibles:**
- `PLAY`: Iniciar reproducción
- `PAUSE`: Pausar reproducción
- `STOP`: Detener y volver al inicio
- `STEP_FORWARD`: Avanzar una acción
- `STEP_BACKWARD`: Retroceder una acción
- `SET_SPEED`: Cambiar velocidad de reproducción (x1, x2, x5, x10)
- `JUMP_TO_ACTION`: Saltar a una acción específica
- `FINISH`: Marcar como completada

### 2. Componentes

#### ReplayerControls.tsx

Barra de controles interactiva con:

**Elementos:**
- **Indicador de progreso**: Muestra acción actual y porcentaje completado
- **Botón Stop**: Detiene y vuelve al inicio
- **Botón Step Backward**: Retrocede una acción (deshabilitado si está en inicio)
- **Botón Play/Pause**: Alterna entre reproducción y pausa
- **Botón Step Forward**: Avanza una acción (deshabilitado si está al final)
- **Selector de velocidad**: Dropdown con opciones x1, x2, x5, x10
- **Información de estado**: Muestra estado actual y velocidad

**Estilos:**
- Dark mode con paleta de colores del proyecto
- Botones responsivos con hover states
- Indicador de progreso con animación suave
- Estados deshabilitados (disabled) cuando corresponde

#### ReplayerTimeline.tsx

Timeline visual con todas las acciones agrupadas por calle:

**Características:**
- Agrupa acciones por `preflop`, `flop`, `turn`, `river`
- Código de colores por tipo de acción:
  - **Fold** (gris): Acciones de descarte
  - **Check/Call** (azul): Acciones pasivas
  - **Bet** (ámbar): Apuestas
  - **Raise** (rojo): Subidas
  - **All-in** (rojo oscuro): Apuestas todo
- Click en acción salta a ese punto
- Resalte de acción actual (con ring violeta)
- Opacidad visual para acciones pasadas/futuras
- Leyenda de colores
- Scroll si hay muchas acciones

### 3. Hook useReplayerState

```typescript
interface UseReplayerStateOptions {
  totalActions: number
  initialSpeed?: PlaybackSpeed
}

const replayer = useReplayerState({
  totalActions: DEMO_ACTIONS.length,
  initialSpeed: 1,
})
```

**Retorna:**
```typescript
{
  state: ReplayerState,
  play: () => void,
  pause: () => void,
  stop: () => void,
  stepForward: () => void,
  stepBackward: () => void,
  setSpeed: (speed: PlaybackSpeed) => void,
  jumpToAction: (index: number) => void,
  finish: () => void,
}
```

## Integración en HandReplayer

### Flujo de Reproducción Automática

1. **Detección de Estado Playing**: Cuando estado es `playing`, se activa un timer
2. **Cálculo de Delay**: `baseDelay / playbackSpeed` (800ms / velocidad)
3. **Auto-step**: Se llama `stepForward()` después del delay
4. **Finalización**: Al llegar al final, se cambia automáticamente a `finished`

### Sincronización Automática

La página automáticamente:
- Actualiza la calle (`currentStreet`) basada en `currentAction.street`
- Muestra descripción de la acción actual
- Renderiza el canvas con las cartas comunitarias correctas

## Datos de Demostración

Se incluye `DEMO_ACTIONS` con 11 acciones de ejemplo que recorren las 4 calles:

```typescript
DEMO_ACTIONS: ReplayerActionStep[] = [
  { index: 0, street: 'preflop', player: 'UTG', action: 'raise', amount: 300, description: 'UTG abre a 3x' },
  { index: 1, street: 'preflop', player: 'thesmoy', action: 'raise', amount: 900, description: '3-bet a 9x' },
  // ... más acciones ...
]
```

## Criterios de Aceptación - Cumplidos

✅ **Controles responden correctamente**
- Todos los botones funcionan inmediatamente
- Los estados se actualizan correctamente
- Las transiciones son válidas

✅ **Timeline muestra todas las acciones**
- Agrupa por calle
- Código de colores por tipo
- Click funcional para saltar

✅ **Velocidad ajustable funciona**
- Selector con opciones x1, x2, x5, x10
- El delay se recalcula dinámicamente
- Se puede cambiar velocidad durante reproducción

✅ **Step permite ir acción por acción**
- Botones Forward/Backward funcionan
- Se respetan los límites (inicio/final)
- Pausa automáticamente cuando se usa step

## Tipos Agregados

```typescript
// poker.ts
export type ReplayerPlaybackState = 'idle' | 'playing' | 'paused' | 'finished'
export type PlaybackSpeed = 1 | 2 | 5 | 10

export interface ReplayerActionStep {
  index: number
  street: 'preflop' | 'flop' | 'turn' | 'river'
  player: string
  action: 'fold' | 'check' | 'call' | 'bet' | 'raise' | 'all-in'
  amount: number
  description: string
}

export interface ReplayerState {
  state: ReplayerPlaybackState
  currentActionIndex: number
  totalActions: number
  playbackSpeed: PlaybackSpeed
  isPaused: boolean
}

export interface ReplayerAction {
  type: 'PLAY' | 'PAUSE' | 'STOP' | 'STEP_FORWARD' | 'STEP_BACKWARD' | 'SET_SPEED' | 'JUMP_TO_ACTION' | 'FINISH'
  payload?: unknown
}
```

## Archivos Modificados

- ✨ `frontend/src/types/poker.ts` - Tipos de máquina de estados
- ✨ `frontend/src/hooks/useReplayerState.ts` - Hook de reproducción (NUEVO)
- ✨ `frontend/src/features/replayer/components/ReplayerControls.tsx` - Controles (NUEVO)
- ✨ `frontend/src/features/replayer/components/ReplayerTimeline.tsx` - Timeline (NUEVO)
- 📝 `frontend/src/features/replayer/components/index.ts` - Exports actualizados
- 🔄 `frontend/src/hooks/index.ts` - Exports actualizados
- 🔄 `frontend/src/pages/HandReplayer.tsx` - Integración de controles

## Testing

Para validar:

1. **Botones de Control:**
   - Click en Play/Pause alterna estado
   - Step Forward/Backward funcionan
   - Stop vuelve al inicio
   - Speed selector cambia velocidad

2. **Timeline:**
   - Muestra todas las acciones en colores correctos
   - Click en acción salta a ese punto
   - Resalte visual sigue acción actual

3. **Sincronización:**
   - La calle cambia automáticamente
   - Las cartas se actualizan correctamente
   - El timer se ajusta con cambios de velocidad

4. **Estados Límite:**
   - En inicio: Step Backward deshabilitado
   - En final: Step Forward deshabilitado
   - Stop funciona en cualquier momento

## Próximos Pasos

- Integración con datos reales del backend
- Animaciones de acciones en el canvas
- Historial de cambios de pila
- Overlay de información del rango
- Análisis de equidad en tiempo real

