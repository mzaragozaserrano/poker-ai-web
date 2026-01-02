# Frontend - Poker AI Analyzer

Plataforma de análisis de póker de alto rendimiento con React 18, Vite y TypeScript.

## Características

- React 18 + Vite (dev server < 500ms)
- TypeScript strict mode
- Tailwind CSS (Dark Mode)
- Path aliases configurados (@/components, @/features, etc.)
- ESLint + Prettier para calidad de código
- Estructura de directorios organizada

## Instalación

```bash
cd frontend
npm install
```

## Desarrollo

```bash
npm run dev
```

El servidor estará disponible en `http://localhost:5173`

## Build

```bash
npm run build
```

## Linting

```bash
npm run lint
```

## Estructura de Directorios

```
src/
├── components/       # Componentes reutilizables
├── features/         # Características del dominio
├── hooks/            # Custom React hooks
├── utils/            # Funciones de utilidad
├── types/            # Definiciones TypeScript
├── lib/
│   └── canvas/       # Utilidades de renderizado Konva
├── App.tsx
└── main.tsx
```

## Stack Técnico

- **Framework:** React 18
- **Build:** Vite 5
- **Lenguaje:** TypeScript 5
- **Estilos:** Tailwind CSS 3
- **Router:** React Router 6
- **Estado:** React Query 5
- **Canvas:** React Konva (Hand Replayer)
- **Gráficos:** Recharts 2

## API Backend

- Base URL: `http://127.0.0.1:8000/api/v1`
- WebSocket: `ws://127.0.0.1:8000/ws`

## Licencia

ISC
