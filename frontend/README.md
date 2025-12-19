# Frontend - Poker AI Web

Plataforma de análisis de póker de alto rendimiento para Winamax, construida con React 18, Vite, TypeScript y Tailwind CSS.

## Requisitos

- Node.js 18+ 
- npm 9+

## Instalación

```bash
npm install
```

## Desarrollo

Para ejecutar el servidor de desarrollo local:

```bash
npm run dev
```

El servidor estará disponible en `http://localhost:5173/`

## Compilación

Para crear una compilación de producción:

```bash
npm run build
```

Los archivos compilados estarán en `dist/`

## Visualización de Compilación de Producción

Para probar la compilación de producción localmente:

```bash
npm run preview
```

## Estructura del Proyecto

```
src/
├── components/          # Componentes reutilizables
├── features/            # Características principales
│   ├── replayer/       # Hand Replayer para análisis post-juego
│   ├── stats/          # Estadísticas y análisis
│   └── dashboard/      # Dashboard principal
├── lib/                # Utilidades y librerías
│   └── canvas/        # Renderizado con Konva
├── hooks/              # Custom React hooks
├── utils/              # Funciones utilitarias
├── types/              # Definiciones de tipos TypeScript
├── App.tsx            # Componente raíz
├── main.tsx           # Punto de entrada
└── index.css          # Estilos globales (Tailwind)
```

## Stack Tecnológico

- **React 18**: Framework UI
- **Vite 7**: Build tool moderno y rápido
- **TypeScript**: Type safety
- **Tailwind CSS 4**: Utility-first CSS framework
- **React Router DOM**: Navegación
- **Konva & React-Konva**: Renderizado Canvas para Hand Replayer
- **ECharts**: Gráficos de estadísticas
- **React Query**: Gestión de estado asincrónico
- **Zustand**: State management

## Configuración

### TypeScript
- Strict mode activado
- Path aliases configurados (@/, @components/, @features/, etc.)

### Tailwind CSS
- Tema oscuro (slate-950/slate-800)
- Paleta de colores de póker personalizada (raise, call, fold, equity)
- Breakpoints responsivos

### Vite
- Optimizaciones de producción
- Code splitting automático
- Fast Refresh para HMR

## Linting y Formato

El proyecto incluye ESLint para mantener la calidad del código.

## Licencia

Parte del proyecto Poker AI Web.
