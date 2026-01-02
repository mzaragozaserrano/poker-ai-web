# Arquitectura del Frontend

## Visión General

El frontend de Poker AI Web es una aplicación React moderna optimizada para análisis post-juego de manos de póker. Sigue una arquitectura modular con separación clara entre componentes, características y utilidades.

## Principios de Diseño

1. **Dark Mode First**: La aplicación está diseñada únicamente para modo oscuro (slate-950 background)
2. **Performance**: Vite para HMR rápido, code splitting automático, Canvas para visualizaciones
3. **Type Safety**: TypeScript strict mode en todos los archivos
4. **Modularidad**: Características independientes y componentes reutilizables
5. **Accesibilidad**: Focus rings, navegación por teclado, prefers-reduced-motion

## Estructura de Directorios

### `/src/components`
Componentes reutilizables y genéricos que pueden usarse en múltiples características.

Ejemplos esperados:
- `Button.tsx` - Botón base con variantes
- `Card.tsx` - Contenedor base
- `Modal.tsx` - Modal reutilizable
- `Navbar.tsx` - Barra de navegación

### `/src/features`
Características independientes del dominio, cada una es una "feature" completa.

#### `replayer/`
Hand Replayer - Análisis visual de manos ya jugadas.
- Renderizado con HTML5 Canvas + Konva
- Reproducción paso a paso
- 60 FPS target

#### `stats/`
Estadísticas y análisis agregados.
- Gráficos con ECharts
- Estadísticas por posición
- Desviaciones vs rangos GTO

#### `dashboard/`
Dashboard principal - Visión general de sesiones.
- Resumen de sesiones
- Tabla de leaderboard
- Quick stats

### `/src/lib`
Librerías y utilidades de nivel bajo.

#### `canvas/`
Utilidades específicas para renderizado con Konva.
- `table.ts` - Renderizado de mesa de póker
- `cards.ts` - Renderizado de cartas
- `animations.ts` - Animaciones

Ejemplos esperados:
- Card rendering (AS, KD, etc.)
- Table layout (6-max positioning)
- Action markers (check, bet, raise)

### `/src/hooks`
Custom React hooks reutilizables.

Ejemplos esperados:
- `usePlayerStats.ts` - Fetch datos de jugador
- `useHandHistory.ts` - Fetch manos
- `useWebSocket.ts` - Conexión WebSocket con backend

### `/src/utils`
Funciones de utilidad generales.

Ejemplos esperados:
- `formatters.ts` - Formateo de números, dinero
- `validators.ts` - Validación de entrada
- `api-client.ts` - Cliente HTTP

### `/src/types`
Definiciones de tipos TypeScript.

Esperados:
- `poker.ts` - Hand, Action, Position, etc.
- `api.ts` - Response types del backend
- `ui.ts` - Props types de componentes

## Flujo de Datos

```
Backend (FastAPI/Rust)
        ↓
   WebSocket / REST API
        ↓
  React Component State (Zustand)
        ↓
  React Query Cache
        ↓
  UI Components (Tailwind CSS)
```

## Paleta de Colores

### Base (Slate)
- `bg-slate-950` (#0F172A) - Background principal
- `bg-slate-800` (#1E293B) - Surface/Cards
- `bg-slate-700` (#334155) - Borders

### Poker Actions
- `bg-poker-raise` (#EF4444) - Agresivo (rojo)
- `bg-poker-call` (#3B82F6) - Pasivo (azul)
- `bg-poker-fold` (#64748B) - Descartado (gris)
- `bg-poker-equity-high` (#10B981) - Probabilidad alta (verde)

### Accent
- `bg-accent-violet` (#8B5CF6) - Acciones primarias

## Responsive Design

- **Mobile (<380px)**: Grid 4px gap, text-xs
- **Tablet (380px-768px)**: Grid 6px gap, text-sm/base
- **Desktop (>768px)**: Grid 8px gap, text-base/lg

## Performance Targets

- **Dev Server Startup**: <500ms
- **Page Load**: <2s
- **Canvas Rendering**: 60 FPS
- **Build Size**: <200KB (gzipped)

## Estado de Implementación

✓ **Fase 3.1**: Componentes base y configuración
  - React 18 + Vite + TypeScript
  - Tailwind CSS con modo oscuro
  - Button, Card, Navbar, Modal, Input, Badge
  - React Query y API client
  - WebSocket hook con reconexión automática
  - Layout principal con sidebar
  - React Router con 8 páginas

✓ **Fase 3.2**: Feature Replayer
  - Hand Replayer con React-Konva
  - Mesa 6-max con renderizado Canvas (60 FPS)
  - Sistema de cartas con sprites SVG
  - Controles de reproducción completos
  - Toggle de formato BB/Moneda

✓ **Fase 3.3**: Feature Stats y Dashboard
  - Dashboard principal con StatCards
  - Gráficos de beneficios con Recharts
  - Lista de manos recientes
  - Matriz de rangos 13x13
  - Presets de rangos GTO 6-max
  - Comparación con rangos estratégicos

