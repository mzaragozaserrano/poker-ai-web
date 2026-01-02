# React Router & Layout Configuration (Issue #41)

## Resumen
Configuración completa de React Router v6 con layout principal, sidebar colapsable e indicador de ruta activa.

## Estructura de Rutas

```
/                    → Dashboard (Página principal)
/sessions            → Lista de sesiones de juego
/hands/:handId       → Hand Replayer (Análisis de mano individual)
/stats               → Estadísticas del jugador
/settings            → Configuración de la aplicación
*                    → Página 404 (Not Found)
```

## Componentes Principales

### 1. MainLayout (`src/layouts/MainLayout.tsx`)
Layout contenedor que:
- Renderiza el Sidebar fijo
- Gestiona el estado de colapso del sidebar
- Proporciona área de contenido responsive
- Usa `<Outlet />` para renderizar rutas anidadas

**Props:**
- Estado interno: `isSidebarCollapsed` (boolean)
- Callback: `onToggleCollapse` para alternar estado

### 2. Sidebar (`src/components/Sidebar.tsx`)
Navegación lateral con:
- 5 items de navegación con iconos emoji
- Indicador visual de ruta activa (fondo violeta)
- Botón de colapso/expansión
- Footer con información del usuario ("thesmoy")
- Ancho dinámico: 256px (expandido) / 80px (colapsado)

**Props:**
```typescript
interface SidebarProps {
  isCollapsed: boolean
  onToggleCollapse: () => void
}
```

**Características:**
- Transiciones suaves con `transition-all duration-300`
- Uso de `clsx` para aplicar clases condicionales
- Detección de ruta activa con `useLocation()`
- Posicionamiento fijo (`fixed`) y z-index alto (`z-40`)

### 3. Páginas (Placeholder)
- **Dashboard**: Tarjetas con estadísticas clave (Manos, Ganancia, VPIP, PFR)
- **Sessions**: Tabla de sesiones con fecha, duración, manos y ganancia
- **HandReplayer**: Área preparada para Canvas con React-Konva
- **Stats**: Layout con gráfico de evolución y resumen de métricas
- **Settings**: Formulario de preferencias y configuración
- **NotFound**: Página 404 con link de regreso al dashboard

## Configuración de Routing

### main.tsx
```typescript
<BrowserRouter>
  <QueryClientProvider client={queryClient}>
    <App />
  </QueryClientProvider>
</BrowserRouter>
```

### App.tsx
```typescript
const router = useRoutes(routes)
return router
```

### routes.tsx
Define estructura de rutas con `MainLayout` como componente padre y rutas anidadas en `children`.

## Indicador de Ruta Activa

La lógica en Sidebar utiliza `useLocation()` y `pathname` para detectar la ruta activa:

```typescript
const isActiveRoute = (href: string) => {
  if (href === '/' && location.pathname === '/') return true
  if (href !== '/' && location.pathname.startsWith(href.split(':')[0])) return true
  return false
}
```

Esto permite:
- Marcar exactamente la ruta "/" como activa solo en homepage
- Marcar rutas como "/sessions", "/stats", etc. como activas cuando se navega a ellas
- Manejo especial para rutas parametrizadas como "/hands/:handId"

## Paleta de Colores

### Sidebar
- Fondo: `bg-slate-800` (#1E293B)
- Borde: `border-slate-700` (#334155)
- Texto inactivo: `text-slate-300` (#CBD5E1)
- Hover: `hover:bg-slate-700` (#334155)
- Activo: `bg-violet-500` (#A78BFA)
- Logo: `text-violet-400` (#A78BFA)

### Main Content
- Fondo: `bg-slate-950` (#0F172A)
- Padding: `p-8`

## Responsive Design

El layout es responsive pero sin comportamiento mobile específico aún. En futuras iteraciones:
- Sidebar podría ocultarse completamente en mobile
- Se podría agregar un hamburger menu
- El contenido ocuparía el ancho completo

## Testing

El proyecto compila sin errores de TypeScript y se puede iniciar con:

```bash
npm run dev
```

La navegación funciona sin recargar página gracias a React Router, y el sidebar mantiene su estado durante la navegación.

## Próximas Tareas

- Integración de estadísticas reales desde la API en Dashboard y Stats
- Implementación del Hand Replayer con React-Konva (Issue #42)
- Mejoras de responsividad para mobile
- Agregar animaciones de transición de contenido

