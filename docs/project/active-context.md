# FASE 3 PENDIENTE - Interfaz de Usuario y Visualización

## Estado General
La Fase 2 (Motor Matemático y Capa de Servicio) ha sido completada. Se inicia la Fase 3: Frontend React.

## Resumen de Fase 2 Completada

### 2.1 Motor de Evaluación de Manos (Rust)
- Algoritmo Cactus Kev híbrido (5, 6 y 7 cartas)
- Perfect Hash Table: 133M combinaciones, generación en 24s, búsquedas en 19.4ns
- Monte Carlo con SIMD AVX2 + Rayon (16 threads), early stopping < 0.1%

### 2.2 Orquestación y API (FastAPI + PyO3)
- Entorno Python con Poetry en server-api/
- Crate poker-ffi con PyO3 (overhead < 1ms)
- Endpoints REST completos:
  - GET /health
  - GET /api/v1/stats/player/{name}
  - GET /api/v1/hands/recent
  - GET /api/v1/hands/{hand_id}
  - POST /api/v1/equity/calculate
  - POST /api/v1/equity/calculate/multiway
- WebSocket /ws con heartbeat automático
- File Watcher Service integrado (Rust -> Python -> WebSocket)
- Tests de integración completos

---

## Próxima Tarea: FASE 3.1 - Base de la SPA

### Contexto (Issue #37)
- Fase 3: Interfaz de Usuario y Visualización
- Stack: React 18 + Vite + TypeScript + Tailwind CSS
- Objetivo: Configurar Tailwind CSS con paleta Dark Mode específica para poker
- Status: En progreso - Configurando Tailwind CSS y paleta de colores

### Tareas Planificadas (Fase 3.1)
- [x] Configurar proyecto React con Vite + TypeScript
- [x] Configurar Tailwind CSS con paleta de colores de poker (Issue #37)
- [ ] Crear componentes base (Button, Card, Modal, Navbar)
- [ ] Implementar layout principal con sidebar
- [ ] Integrar React Query para estado del servidor
- [ ] Crear hook useWebSocket para conexión con backend
- [ ] Configurar routing con React Router

### Tareas Planificadas (Fase 3.2)
- [ ] Implementar Hand Replayer con React-Konva
- [ ] Renderizado de mesa 6-max a 60 FPS
- [ ] Sistema de cartas y animaciones
- [ ] Controles de reproducción

### Tareas Planificadas (Fase 3.3)
- [ ] Dashboard principal con estadísticas
- [ ] Gráficos de beneficios con Recharts/ECharts
- [ ] Matriz de rangos 13x13
- [ ] Vista de análisis de leaks

---

## Arquitectura Frontend Objetivo

```
frontend/
├── src/
│   ├── components/       # Componentes reutilizables
│   │   ├── Button.tsx
│   │   ├── Card.tsx
│   │   ├── Modal.tsx
│   │   └── Navbar.tsx
│   ├── features/         # Características del dominio
│   │   ├── replayer/     # Hand Replayer (Canvas)
│   │   ├── stats/        # Estadísticas y análisis
│   │   └── dashboard/    # Dashboard principal
│   ├── lib/
│   │   └── canvas/       # Utilidades de renderizado Konva
│   ├── hooks/            # Custom React hooks
│   │   ├── usePlayerStats.ts
│   │   ├── useHandHistory.ts
│   │   └── useWebSocket.ts
│   ├── utils/            # Funciones de utilidad
│   │   ├── formatters.ts
│   │   └── api-client.ts
│   ├── types/            # Definiciones TypeScript
│   │   ├── poker.ts
│   │   └── api.ts
│   ├── App.tsx
│   └── main.tsx
├── public/
├── index.html
├── package.json
├── tailwind.config.js
├── tsconfig.json
└── vite.config.ts
```

---

## Paleta de Colores (Dark Mode)

### Base (Slate)
- bg-slate-950 (#0F172A) - Background principal
- bg-slate-800 (#1E293B) - Surface/Cards
- bg-slate-700 (#334155) - Borders

### Poker Actions
- poker-raise (#EF4444) - Agresivo (rojo)
- poker-call (#3B82F6) - Pasivo (azul)
- poker-fold (#64748B) - Descartado (gris)
- poker-equity-high (#10B981) - Probabilidad alta (verde)

### Accent
- accent-violet (#8B5CF6) - Acciones primarias / Hero

---

## Dependencias Frontend Clave
- react: ^18.2.0
- react-dom: ^18.2.0
- react-router-dom: ^6.x
- @tanstack/react-query: ^5.x
- react-konva: ^18.x (Canvas)
- konva: ^9.x
- recharts: ^2.x (Gráficos)
- tailwindcss: ^3.x
- typescript: ^5.x
- vite: ^5.x

---

## Conexión con Backend

### API REST
- Base URL: http://127.0.0.1:8000/api/v1
- Documentación: http://127.0.0.1:8000/docs

### WebSocket
- URL: ws://127.0.0.1:8000/ws
- Mensajes:
  - connection_ack: Confirmación de conexión
  - heartbeat: Keepalive cada 30s
  - new_hand: Notificación de nueva mano detectada
  - error: Mensajes de error
