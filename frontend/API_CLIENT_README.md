## API Client & React Query - Documentación

### Resumen de Implementación (Issue #39)

Se ha configurado un sistema completo de gestión de estado del servidor usando React Query y un cliente HTTP tipado. Esto permite comunicación eficiente y confiable con la API backend en `http://127.0.0.1:8000/api/v1`.

---

## Estructura de Archivos

```
frontend/src/
├── types/
│   ├── api.ts              # Tipos de todas las respuestas API
│   ├── poker.ts            # Tipos de dominio de poker
│   └── index.ts            # Exportación centralizada de tipos
├── lib/
│   ├── query-client.ts     # Configuración de QueryClient
│   └── index.ts
├── utils/
│   ├── api-client.ts       # Cliente HTTP centralizado
│   └── index.ts
├── hooks/
│   ├── usePlayerStats.ts   # Hook para estadísticas de jugadores
│   ├── useRecentHands.ts   # Hook para historial de manos
│   ├── useHand.ts          # Hook para detalles de una mano
│   ├── useEquityCalculation.ts  # Hook para cálculo de equidad
│   └── index.ts
└── pages/
    └── ApiClientDemo.tsx   # Página de demostración
```

---

## Componentes Principales

### 1. Tipos API (`types/api.ts`)

Define interfaces tipadas para todas las respuestas de la API:

- **Respuestas Genéricas**: `ApiErrorResponse`, `ApiSuccessResponse<T>`
- **Estadísticas de Jugador**: `PlayerStats`, `PositionalStats`
- **Historial de Manos**: `Hand`, `HandSummary`, `Street`, `HandAction`
- **Equidad**: `EquityCalculationRequest`, `EquityCalculationResponse`

Todas las respuestas están tipadas con TypeScript stricto.

### 2. Cliente HTTP (`utils/api-client.ts`)

Cliente HTTP centralizado con:

```typescript
// Método genérico
fetchAPI<T>(endpoint: string, options?: FetchOptions): Promise<T>

// Métodos de utilidad
apiClient.get<T>(endpoint, options)
apiClient.post<T>(endpoint, body, options)
apiClient.put<T>(endpoint, body, options)
apiClient.delete<T>(endpoint, options)
```

**Características**:
- Manejo automático de errores con clase `ApiError`
- Timeout configurable (default: 30s)
- Reintentos automáticos
- Headers de `Content-Type: application/json` automáticos
- Tipado genérico completo

### 3. QueryClient (`lib/query-client.ts`)

Configuración centralizada con defaults optimizados para poker:

```typescript
// Estadísticas: Cache 5 min, GC 10 min
staleTime: 5 * 60 * 1000
gcTime: 10 * 60 * 1000

// Reintentos
retry: 2
retryDelay: exponencial con cap 30s
```

Presets disponibles:
- `frequentUpdateConfig`: Para queries que cambian frecuentemente (10s cache)
- `rareUpdateConfig`: Para queries estables (30 min cache)
- `noCacheConfig`: Sin cache (cálculos dinámicos)

### 4. Hooks Personalizados

#### `usePlayerStats(options: UsePlayerStatsOptions)`

```typescript
const { data, isPending, isError, error } = usePlayerStats({
  playerName: 'thesmoy',
  enabled: true  // Opcional, default true
})

if (data) {
  console.log(data.vpip)      // number
  console.log(data.pfr)       // number
  console.log(data.winrate)   // number (BB/100)
}
```

#### `useRecentHands(options?: UseRecentHandsOptions)`

```typescript
const { data } = useRecentHands({ limit: 20 })

if (data) {
  console.log(data.hands)    // HandSummary[]
  console.log(data.total)    // number
  console.log(data.limit)    // number
}
```

#### `useHand(options: UseHandOptions)`

```typescript
const { data } = useHand({
  handId: 'hand-123',
  enabled: handIdIsValid  // Opcional
})

if (data) {
  console.log(data.streets)      // Street[]
  console.log(data.heroHoleCards) // Card[]
  console.log(data.result)       // number (centavos)
}
```

#### `useEquityCalculation()`

```typescript
const mutation = useEquityCalculation()

mutation.mutate({
  heroRange: 'AA,KK,AKs',
  villainRange: 'QQ+,AJs',
  runouts: 1000
})

if (mutation.data) {
  console.log(mutation.data.heroEquity)      // 0-1
  console.log(mutation.data.villainEquity)   // 0-1
  console.log(mutation.data.executionTimeMs) // number
}
```

#### `useEquityCalculationMultiway()`

```typescript
const mutation = useEquityCalculationMultiway()

mutation.mutate({
  ranges: [
    { name: 'Button', range: 'AA,KK,AKs' },
    { name: 'SB', range: 'QQ+,AJs' },
    { name: 'BB', range: '22+,AJs+' }
  ],
  runouts: 1000
})

if (mutation.data) {
  console.log(mutation.data.equities) // EquityData[]
}
```

---

## Configuración en App

El `QueryClientProvider` se ha integrado automáticamente en `src/main.tsx`:

```typescript
import { QueryClientProvider } from '@tanstack/react-query'
import { queryClient } from './lib/query-client'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <QueryClientProvider client={queryClient}>
    <App />
  </QueryClientProvider>
)
```

**No necesitas hacer nada más** - solo usa los hooks en tus componentes.

---

## Manejo de Errores

Todos los hooks devuelven `ApiError` tipado:

```typescript
const { isError, error } = usePlayerStats({ playerName })

if (isError) {
  if (error instanceof ApiError) {
    console.log(error.status)     // number (e.g., 404, 500)
    console.log(error.statusText) // string (e.g., 'Not Found')
    console.log(error.detail)     // string | undefined
  }
}
```

---

## Ejemplo de Uso en Componente

```typescript
import { usePlayerStats, useRecentHands } from '../hooks'

export function Dashboard() {
  const { data: stats, isPending, isError } = usePlayerStats({
    playerName: 'thesmoy'
  })
  const { data: handsData } = useRecentHands({ limit: 10 })

  if (isPending) return <div>Cargando...</div>
  if (isError) return <div>Error al cargar datos</div>

  return (
    <div>
      <h1>{stats?.name}</h1>
      <p>Winrate: {stats?.winrate} BB/100</p>
      <p>Manos: {handsData?.hands.length}</p>
    </div>
  )
}
```

---

## Testing de la API

Se incluye una página de demostración en `pages/ApiClientDemo.tsx` que prueba todos los hooks:

1. **Estadísticas de Jugador**: Carga datos de `usePlayerStats`
2. **Manos Recientes**: Lista las últimas manos con `useRecentHands`
3. **Detalles de Mano**: Click en una mano para ver detalles con `useHand`
4. **Calculador de Equidad**: Calcula equidad heads-up con `useEquityCalculation`

Para ver la demo:
```bash
cd frontend
npm run dev
# Visita http://localhost:5173
```

---

## Criterios de Aceptación - COMPLETADOS ✅

- [x] Hooks funcionan con API real
- [x] Cache configurado correctamente (staleTime, gcTime)
- [x] Error handling consistente con tipos
- [x] TypeScript stricto en todos los archivos
- [x] Página de demostración funcional
- [x] ESLint sin errores
- [x] Compilación TypeScript exitosa

---

## Próximos Pasos

- Implementar `useWebSocket` para notificaciones en tiempo real (Issue #40)
- Configurar React Router para navegación (Issue #41)
- Crear layout principal con sidebar (Issue #42)
- Implementar Hand Replayer con Konva (Fase 3.2)

---

## Referencias

- React Query Docs: https://tanstack.com/query/latest
- TypeScript Handbook: https://www.typescriptlang.org/docs/
- API Endpoints: `docs/specs/api-spec.md`
- Esquema de Datos: `docs/specs/db-schema.md`

