# Toggle de Formato de Cantidades - Guía de Integración

## Resumen
Implementación completada del toggle para alternar entre formato de Big Blinds (BB) y moneda real (EUR) en el Hand Replayer.

## Componentes Creados

### 1. Hook `useAmountFormat()` (frontend/src/hooks/useAmountFormat.ts)
Gestiona:
- Estado del formato (BB o EUR)
- Persistencia en localStorage bajo la clave `poker-amount-format`
- Toggle entre formatos
- Validación de valores guardados

**Uso:**
```tsx
const { format, setFormat, toggleFormat, isLoaded } = useAmountFormat()
```

### 2. Utilidades `formatAmount()` (frontend/src/utils/formatters.ts)
Funciones de formateo:
- `formatAmount(amount, format, bigBlind)` - Formatea una cantidad
- `eurToBB(eurAmount, bigBlind)` - Convierte EUR a BB
- `bbToEur(bbAmount, bigBlind)` - Convierte BB a EUR
- `formatStack(amount, format, bigBlind)` - Formatea stacks/botes

**Ejemplos:**
```tsx
formatAmount(50000, 'bb', 200)  // "250bb"
formatAmount(50000, 'eur', 200) // "500.00€"
formatStack(0, 'eur', 200)      // "0"
```

### 3. Componente `AmountFormatToggle` (frontend/src/features/replayer/components/AmountFormatToggle.tsx)
Botón toggle visual que muestra:
- "BB" cuando el formato es Big Blinds
- "€" cuando el formato es EUR

### 4. Integración en `ReplayerControls`
- Nuevo prop `amountFormat: AmountFormat`
- Nuevo prop `onToggleAmountFormat?: () => void`
- Toggle renderizado en barra de controles después de selector de velocidad
- Separador visual (línea) entre controles principales y toggle

## Cómo Usar en Componentes

### En componentes React que necesiten el formato:
```tsx
import { useAmountFormat } from '../hooks/useAmountFormat'
import { formatAmount } from '../utils/formatters'

export const MyComponent = () => {
  const { format } = useAmountFormat()
  
  return (
    <div>
      {formatAmount(15000, format, 200)} {/* Cantidad formateada */}
    </div>
  )
}
```

### En Canvas (Konva):
```tsx
import { formatStack } from '../utils/formatters'

// En PokerTable o similar:
const potDisplay = formatStack(totalPot, amountFormat, bigBlind)
```

### En ReplayerTimeline o logs de acciones:
```tsx
// Para mostrar apuestas en el log
const actionAmount = formatAmount(action.amount, format, bigBlind)
```

## Especificación de Formato

### Big Blinds (BB)
- Formato: "2.5bb", "100bb", "15bb"
- Decimales: Solo se muestran cuando hay parte decimal significativa
- Ejemplo: 250bb, 2.5bb, 0.5bb

### Moneda (EUR)
- Formato: "0.05€", "2.00€", "100.00€"
- Decimales: Siempre 2 posiciones decimales
- Símbolo: Siempre al final

## Persistencia
- La preferencia se guarda automáticamente en localStorage
- Se restaura al recargar la página
- Por defecto comienza en formato "bb"

## Próximos Pasos de Integración (Para Completar)

1. **PokerTable**: Pasar `amountFormat` prop y usarlo en:
   - Renderizado del pot central (línea 114)
   - Información de stacks en PlayerSeat

2. **PlayerSeat**: Integrar formato en:
   - Visualización del stack
   - Apuestas actuales

3. **ReplayerTimeline**: Mostrar cantidades formateadas en el log de acciones

4. **HandReplayer**: Pasar `amountFormat` a todos los componentes que lo necesiten

5. **Tests**: Crear tests unitarios para `formatAmount()` y `useAmountFormat()`

