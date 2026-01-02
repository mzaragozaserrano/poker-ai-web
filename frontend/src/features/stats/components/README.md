# Componentes de Análisis de Rangos

Este directorio contiene los componentes para visualización y análisis de rangos de starting hands en poker.

## Componentes

### RangeMatrix

Matriz 13x13 para visualización de rangos de starting hands con mapas de calor.

**Uso:**

```tsx
import { RangeMatrix } from '@/features/stats/components'
import type { RangeData } from '@/types/ranges'

function MyComponent() {
  const [range, setRange] = useState<RangeData>()
  
  return (
    <RangeMatrix
      range={range}
      onCellClick={(hand) => console.log('Clicked:', hand)}
      onSelectionChange={(hands) => console.log('Selected:', hands)}
    />
  )
}
```

**Props:**

- `range?: RangeData` - Rango a visualizar (opcional)
- `onCellClick?: (hand: HandNotation) => void` - Callback al hacer click en una celda
- `onSelectionChange?: (selectedHands: HandNotation[]) => void` - Callback cuando cambia la selección
- `className?: string` - Clases CSS adicionales

**Características:**

- Grid 13x13 con todas las starting hands (169 combinaciones)
- Layout estándar: diagonal (pairs), arriba (suited), abajo (offsuit)
- Colores de calor basados en frecuencia (0.0-1.0)
- Drag-to-select para selección múltiple
- Tooltips con desglose de acciones
- Performance optimizada con memoización

---

### RangePresets

Sidebar con presets de rangos GTO para 6-max cash games.

**Uso:**

```tsx
import { RangePresets } from '@/features/stats/components'
import type { RangePreset } from '@/types/ranges'

function MyComponent() {
  const [selectedId, setSelectedId] = useState<string>()
  
  const handleSelect = (preset: RangePreset) => {
    setSelectedId(preset.id)
    // Usar preset.range para actualizar la matriz
  }
  
  return (
    <RangePresets
      onPresetSelect={handleSelect}
      selectedPresetId={selectedId}
    />
  )
}
```

**Props:**

- `onPresetSelect: (preset: RangePreset) => void` - Callback al seleccionar un preset
- `selectedPresetId?: string` - ID del preset actualmente seleccionado
- `className?: string` - Clases CSS adicionales

**Presets Incluidos:**

#### RFI (Raise First In)
- **UTG Open** - Early Position (~15%)
- **MP Open** - Middle Position (~20%)
- **CO Open** - Cutoff (~35%)
- **BTN Open** - Button (~48%)
- **SB Open** - Small Blind (~45%)

#### 3Bet
- **BB vs BTN 3Bet** - Big Blind 3Bet vs Button (~12%)
- **SB vs BTN 3Bet** - Small Blind 3Bet vs Button (~10%)

#### Blind Defense
- **BB vs SB Call** - Big Blind Call vs Small Blind (~40%)

---

## Utilidades

### rangeUtils

Funciones de utilidad para análisis y visualización de rangos.

**Ubicación:** `features/stats/utils/rangeUtils.ts`

**Funciones Principales:**

```typescript
// Mapas de calor
getHeatmapColor(action: RangeAction, frequency: Frequency): string
getHeatmapColorGradient(minColor: string, maxColor: string, frequency: Frequency): string
getTextColor(backgroundColor: string, frequency: Frequency): string

// Análisis de rangos
getTotalFrequency(entries: RangeEntry[]): Frequency
getPrimaryAction(entries: RangeEntry[]): RangeEntry | null
getImplicitFoldFrequency(entries: RangeEntry[]): Frequency
isHandInRange(range: RangeData, hand: HandNotation): boolean
countHandsInRange(range: RangeData): number
getRangePercentage(range: RangeData): number

// Formateo
formatFrequency(frequency: Frequency): string
formatActionBreakdown(entries: RangeEntry[]): string
describeStrategy(entries: RangeEntry[]): string

// Validación
validateFrequencies(entries: RangeEntry[]): boolean
validateFrequencyRange(frequency: Frequency): boolean
validateRange(range: RangeData): { valid: boolean; errors: string[] }
```

---

## Tipos

### Tipos Principales

```typescript
// Tipo de mano
type HandType = 'pair' | 'suited' | 'offsuit'

// Notación de mano
type HandNotation = string // "AA", "AKs", "AKo", etc.

// Frecuencia (0.0 - 1.0)
type Frequency = number

// Acción
type RangeAction = 'RAISE' | 'CALL' | 'FOLD' | 'ALL_IN' | 'MARGINAL'

// Entrada de rango
interface RangeEntry {
  hand: HandNotation
  action: RangeAction
  frequency: Frequency
}

// Rango completo (169 manos)
type RangeData = {
  [hand: HandNotation]: RangeEntry[]
}

// Preset de rango
interface RangePreset {
  id: string
  name: string
  description: string
  position: string
  range: RangeData
}
```

---

## Paleta de Colores

Los colores siguen la paleta del proyecto (Dark Mode):

| Acción   | Color       | Hex       | Uso                                    |
|----------|-------------|-----------|----------------------------------------|
| RAISE    | red-500     | `#EF4444` | Acciones agresivas (open, 3bet, 4bet)  |
| CALL     | blue-500    | `#3B82F6` | Acciones pasivas (call, flat)          |
| FOLD     | slate-500   | `#64748B` | Fold (calculado implícitamente)        |
| ALL_IN   | amber-500   | `#F59E0B` | All-in preflop                         |
| MARGINAL | violet-500  | `#8B5CF6` | Acciones marginales GTO                |

**Intensidad:** La opacidad representa la frecuencia (0% = transparente, 100% = opaco)

---

## Ejemplo Completo

```tsx
import { useState } from 'react'
import { RangeMatrix, RangePresets } from '@/features/stats/components'
import type { RangePreset, RangeData } from '@/types/ranges'

export function RangeAnalyzer() {
  const [selectedRange, setSelectedRange] = useState<RangeData>()
  const [selectedPresetId, setSelectedPresetId] = useState<string>()
  const [selectedHands, setSelectedHands] = useState<string[]>([])

  const handlePresetSelect = (preset: RangePreset) => {
    setSelectedRange(preset.range)
    setSelectedPresetId(preset.id)
  }

  return (
    <div className="grid grid-cols-4 gap-6">
      {/* Sidebar de Presets */}
      <div className="col-span-1">
        <RangePresets
          onPresetSelect={handlePresetSelect}
          selectedPresetId={selectedPresetId}
        />
      </div>

      {/* Matriz de Rangos */}
      <div className="col-span-3">
        <RangeMatrix
          range={selectedRange}
          onCellClick={(hand) => console.log('Clicked:', hand)}
          onSelectionChange={setSelectedHands}
        />
        
        {selectedHands.length > 0 && (
          <p>Seleccionadas: {selectedHands.length} manos</p>
        )}
      </div>
    </div>
  )
}
```

---

## Testing

### Manual Testing Checklist

- [ ] Matriz renderiza 169 celdas correctamente
- [ ] Layout: diagonal (pairs), arriba (suited), abajo (offsuit)
- [ ] Etiquetas A-2 visibles en filas y columnas
- [ ] Selección de presets actualiza la matriz
- [ ] Colores de calor visibles según frecuencia
- [ ] Click en celda selecciona individualmente
- [ ] Drag sobre celdas selecciona área rectangular
- [ ] Tooltips muestran desglose de acciones
- [ ] Performance sin lag durante drag-to-select

---

## Mejoras Futuras

### Fase 1: Edición de Rangos
- [ ] Click derecho para editar frecuencia
- [ ] Guardar rangos personalizados en localStorage
- [ ] Importar/exportar rangos en JSON

### Fase 2: Comparación
- [ ] Mostrar dos matrices lado a lado
- [ ] Resaltar diferencias entre rangos
- [ ] Calcular overlap entre rangos

### Fase 3: Análisis Avanzado
- [ ] Integrar con datos de manos jugadas
- [ ] Mostrar desviaciones del rango GTO
- [ ] Identificar leaks por posición

### Fase 4: UX Enhancements
- [ ] Animaciones de transición
- [ ] Navegación con teclado
- [ ] Screen reader support
- [ ] High contrast mode

---

## Referencias

- **Spec de Rangos:** `/docs/specs/range-spec.md`
- **Rangos GTO:** `/docs/ranges/preflop-ranges.md`
- **UI Foundations:** `/docs/project/ui-foundations.md`

