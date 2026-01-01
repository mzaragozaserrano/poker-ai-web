<#
.SYNOPSIS
    Crea issues en GitHub en lote basándose en una lista definida.
    Este script es modificado automáticamente por el Agente de Cursor antes de su ejecución.
    
.NOTES
    IMPORTANTE:
    - Usa Here-Strings (@"..."@) para Títulos y Bodies.
    - Las tildes y caracteres especiales (ñ, á, é) se manejan nativamente gracias a la configuración del entorno.
#>

# Configurar salida de consola a UTF-8 por si acaso
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# --- ZONA EDITABLE POR EL AGENTE ---
# FASE 3: Interfaz de Usuario y Visualización
$issues = @(
    @{ 
        Title = @"
3.1.1 Configurar proyecto React con Vite + TypeScript
"@
        Body = @"
Inicializar el proyecto frontend con React 18, Vite y TypeScript.

## Contexto
- Fase 3.1: Base de la SPA (React)
- Stack: React 18 + Vite + TypeScript
- Directorio: frontend/

## Tareas
- [ ] Crear proyecto con Vite (template react-ts)
- [ ] Configurar TypeScript strict mode
- [ ] Configurar ESLint + Prettier
- [ ] Configurar paths aliases (@/components, @/features, etc.)
- [ ] Crear estructura de directorios base (components/, features/, hooks/, utils/, types/)
- [ ] Verificar que dev server inicia correctamente

## Criterios de Aceptación
- npm run dev inicia servidor en < 500ms
- TypeScript strict mode activo
- Estructura de directorios creada
- Build de producción funciona sin errores

## Dependencias Clave
- react: ^18.2.0
- react-dom: ^18.2.0
- typescript: ^5.x
- vite: ^5.x
"@
        Labels = "enhancement,frontend,fase-3" 
    },
    @{ 
        Title = @"
3.1.2 Configurar Tailwind CSS con paleta Dark Mode
"@
        Body = @"
Configurar Tailwind CSS con la paleta de colores específica para el modo oscuro de poker.

## Contexto
- Fase 3.1: Base de la SPA (React)
- Dark Mode ONLY (no light mode)
- Paleta definida en docs/project/ui-foundations.md

## Tareas
- [ ] Instalar y configurar Tailwind CSS
- [ ] Configurar paleta de colores base (Slate-950/800/700)
- [ ] Añadir colores de acciones de poker (raise, call, fold, equity)
- [ ] Configurar color de acento (violet-500 para Hero)
- [ ] Crear archivo de variables CSS para colores custom
- [ ] Configurar dark mode como default (no toggle)
- [ ] Documentar paleta en comentarios de tailwind.config.js

## Criterios de Aceptación
- Colores de poker disponibles como clases (bg-poker-raise, etc.)
- Background slate-950 por defecto
- No hay opción de light mode
- Paleta consistente con ui-foundations.md

## Paleta de Colores
- bg-slate-950 (#0F172A) - Background principal
- bg-slate-800 (#1E293B) - Surface/Cards
- poker-raise (#EF4444) - Rojo
- poker-call (#3B82F6) - Azul
- poker-fold (#64748B) - Gris
- accent-violet (#8B5CF6) - Hero
"@
        Labels = "enhancement,frontend,fase-3,ui" 
    },
    @{ 
        Title = @"
3.1.3 Crear componentes base del sistema de diseño
"@
        Body = @"
Desarrollar los componentes reutilizables base del sistema de diseño.

## Contexto
- Fase 3.1: Base de la SPA (React)
- Componentes en src/components/
- Dark Mode only

## Tareas
- [ ] Crear Button.tsx con variantes (primary, secondary, ghost, destructive)
- [ ] Crear Card.tsx como contenedor base
- [ ] Crear Modal.tsx para diálogos
- [ ] Crear Navbar.tsx para navegación principal
- [ ] Crear Input.tsx para formularios
- [ ] Crear Badge.tsx para etiquetas
- [ ] Documentar props con TypeScript interfaces
- [ ] Crear storybook básico o página de componentes

## Criterios de Aceptación
- Todos los componentes tipados con TypeScript
- Estilos consistentes con paleta Dark Mode
- Focus rings visibles para accesibilidad
- Componentes exportados desde index.ts

## Componentes Mínimos
- Button (variantes: primary, secondary, ghost)
- Card (con header, body, footer opcionales)
- Modal (con overlay y animación)
- Navbar (logo, links, user area)
"@
        Labels = "enhancement,frontend,fase-3,ui" 
    },
    @{ 
        Title = @"
3.1.4 Configurar React Query y API client
"@
        Body = @"
Configurar React Query para estado del servidor y crear cliente HTTP para la API.

## Contexto
- Fase 3.1: Base de la SPA (React)
- Backend API en http://127.0.0.1:8000/api/v1
- Usar @tanstack/react-query

## Tareas
- [ ] Instalar @tanstack/react-query
- [ ] Configurar QueryClient con defaults
- [ ] Crear api-client.ts con fetch wrapper
- [ ] Configurar base URL y headers
- [ ] Crear hooks para endpoints principales:
  - [ ] usePlayerStats(playerName)
  - [ ] useRecentHands(limit)
  - [ ] useHand(handId)
  - [ ] useEquityCalculation()
- [ ] Configurar error handling global
- [ ] Añadir tipos de respuesta (types/api.ts)

## Criterios de Aceptación
- Hooks funcionan con API real
- Cache configurado correctamente
- Error handling consistente
- Tipos TypeScript para todas las respuestas

## Endpoints a Integrar
- GET /api/v1/stats/player/{name}
- GET /api/v1/hands/recent
- GET /api/v1/hands/{hand_id}
- POST /api/v1/equity/calculate
"@
        Labels = "enhancement,frontend,fase-3,api" 
    },
    @{ 
        Title = @"
3.1.5 Crear hook useWebSocket para conexión con backend
"@
        Body = @"
Desarrollar hook personalizado para gestionar la conexión WebSocket con el backend.

## Contexto
- Fase 3.1: Base de la SPA (React)
- WebSocket endpoint: ws://127.0.0.1:8000/ws
- Debe manejar reconexión automática

## Tareas
- [ ] Crear useWebSocket.ts en src/hooks/
- [ ] Implementar conexión automática al montar
- [ ] Manejar mensajes de heartbeat
- [ ] Implementar reconexión automática con backoff exponencial
- [ ] Crear estado de conexión (connected, disconnected, reconnecting)
- [ ] Exponer callback para new_hand messages
- [ ] Manejar cleanup en unmount
- [ ] Añadir tipos para mensajes WebSocket

## Criterios de Aceptación
- Conexión automática al cargar la app
- Reconexión automática funciona
- Estado de conexión visible en UI
- No hay memory leaks en unmount
- Heartbeat mantiene conexión viva

## Formato de Mensajes
- connection_ack: { type, client_id, timestamp }
- heartbeat: { type, timestamp }
- new_hand: { type, hand_id, timestamp, hero_result, ... }
"@
        Labels = "enhancement,frontend,fase-3,websocket" 
    },
    @{ 
        Title = @"
3.1.6 Configurar React Router y layout principal
"@
        Body = @"
Configurar el sistema de routing y el layout principal con sidebar.

## Contexto
- Fase 3.1: Base de la SPA (React)
- Layout con sidebar fija y contenido principal

## Tareas
- [ ] Instalar react-router-dom
- [ ] Configurar BrowserRouter en main.tsx
- [ ] Crear layout principal con sidebar + main content
- [ ] Definir rutas principales:
  - [ ] / (Dashboard)
  - [ ] /sessions (Lista de sesiones)
  - [ ] /hands/:handId (Replayer de mano)
  - [ ] /stats (Estadísticas)
  - [ ] /settings (Configuración)
- [ ] Crear componente Sidebar con navegación
- [ ] Implementar indicador de ruta activa
- [ ] Configurar 404 page

## Criterios de Aceptación
- Navegación funciona sin recargar página
- Sidebar indica ruta activa
- Layout responsive (sidebar colapsable en mobile)
- Rutas definidas y funcionando

## Estructura de Rutas
/ - Dashboard principal
/sessions - Lista de sesiones de juego
/hands/:id - Hand Replayer
/stats - Estadísticas del jugador
/settings - Configuración
"@
        Labels = "enhancement,frontend,fase-3,routing" 
    },
    @{ 
        Title = @"
3.2.1 Implementar canvas de mesa de poker 6-max
"@
        Body = @"
Desarrollar el componente Canvas para renderizar la mesa de poker 6-max con React-Konva.

## Contexto
- Fase 3.2: Hand Replayer (HTML5 Canvas)
- Usar React-Konva para renderizado por GPU
- Objetivo: 60 FPS

## Tareas
- [ ] Instalar react-konva y konva
- [ ] Crear componente PokerTable.tsx en features/replayer/
- [ ] Renderizar mesa oval con felt texture
- [ ] Posicionar 6 seats en posiciones correctas (BTN, SB, BB, UTG, MP, CO)
- [ ] Crear componente PlayerSeat con avatar, nombre, stack
- [ ] Renderizar dealer button
- [ ] Renderizar pot en el centro
- [ ] Crear utilidades de posicionamiento en lib/canvas/

## Criterios de Aceptación
- Mesa renderiza correctamente en canvas
- 6 posiciones claramente identificables
- Performance > 60 FPS
- Responsive a diferentes tamaños de pantalla

## Posiciones 6-max (sentido horario desde BTN)
- BTN (Button) - posición 0
- SB (Small Blind) - posición 1
- BB (Big Blind) - posición 2
- UTG (Under the Gun) - posición 3
- MP (Middle Position) - posición 4
- CO (Cutoff) - posición 5
"@
        Labels = "enhancement,frontend,fase-3,canvas,replayer" 
    },
    @{ 
        Title = @"
3.2.2 Implementar sistema de renderizado de cartas
"@
        Body = @"
Desarrollar el sistema de renderizado de cartas para el Hand Replayer.

## Contexto
- Fase 3.2: Hand Replayer (HTML5 Canvas)
- Cartas deben ser claras y legibles
- Soporte para cartas boca abajo (oponentes)

## Tareas
- [ ] Crear componente Card.tsx para canvas
- [ ] Implementar renderizado de palos (hearts, diamonds, clubs, spades)
- [ ] Implementar renderizado de valores (2-10, J, Q, K, A)
- [ ] Crear sprites o shapes para cartas
- [ ] Implementar carta boca abajo (back)
- [ ] Crear animación de repartir cartas
- [ ] Renderizar community cards (flop, turn, river)
- [ ] Crear utilidades en lib/canvas/cards.ts

## Criterios de Aceptación
- Cartas legibles a diferentes tamaños
- Colores claros (rojo para hearts/diamonds, negro para clubs/spades)
- Animaciones fluidas a 60 FPS
- Cartas de oponentes muestran back

## Notación de Cartas
- Formato: valor + palo (Ah = As de corazones, Kd = Rey de diamantes)
- Valores: 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A
- Palos: h (hearts), d (diamonds), c (clubs), s (spades)
"@
        Labels = "enhancement,frontend,fase-3,canvas,replayer" 
    },
    @{ 
        Title = @"
3.2.3 Implementar controles de reproducción del Hand Replayer
"@
        Body = @"
Desarrollar los controles de reproducción para el Hand Replayer.

## Contexto
- Fase 3.2: Hand Replayer (HTML5 Canvas)
- Controles: Play, Pause, Step, Speed
- Timeline visual de la mano

## Tareas
- [ ] Crear componente ReplayerControls.tsx
- [ ] Implementar botones Play/Pause
- [ ] Implementar botón Step Forward/Backward
- [ ] Implementar selector de velocidad (x1, x2, x5, x10)
- [ ] Crear timeline visual con acciones
- [ ] Implementar click en timeline para saltar a acción
- [ ] Crear máquina de estados para reproducción
- [ ] Sincronizar controles con canvas

## Criterios de Aceptación
- Controles responden correctamente
- Timeline muestra todas las acciones
- Velocidad ajustable funciona
- Step permite ir acción por acción

## Estados de Reproducción
- idle: Mano cargada, no reproduciendo
- playing: Reproducción automática
- paused: Pausado en una acción
- finished: Mano completada
"@
        Labels = "enhancement,frontend,fase-3,replayer" 
    },
    @{ 
        Title = @"
3.2.4 Implementar toggle de formato de cantidades (BB vs Monedas)
"@
        Body = @"
Desarrollar el toggle para alternar entre formato de Big Blinds y moneda real.

## Contexto
- Fase 3.2: Hand Replayer (HTML5 Canvas)
- Definido en docs/specs/ux-spec.md
- Preferencia persistente en localStorage

## Tareas
- [ ] Crear componente AmountFormatToggle.tsx
- [ ] Implementar lógica de conversión BB <-> EUR
- [ ] Aplicar formato a:
  - [ ] Apuestas en canvas
  - [ ] Pot en centro de mesa
  - [ ] Stacks de jugadores
  - [ ] Log de acciones
- [ ] Persistir preferencia en localStorage
- [ ] Crear hook useAmountFormat()
- [ ] Añadir a barra de controles del replayer

## Criterios de Aceptación
- Toggle cambia formato instantáneamente
- Preferencia se mantiene entre sesiones
- Formato aplicado consistentemente en toda la UI
- BB muestra decimales cuando es necesario (2.5bb)

## Formatos
- Big Blinds: "2.5bb", "100bb"
- Moneda: "0.05€", "2.00€"
"@
        Labels = "enhancement,frontend,fase-3,replayer,ux" 
    },
    @{ 
        Title = @"
3.3.1 Implementar Dashboard principal con KPIs
"@
        Body = @"
Desarrollar el Dashboard principal con tarjetas de KPIs y resumen del Hero.

## Contexto
- Fase 3.3: Feature Stats - Dashboard
- Hero: thesmoy
- KPIs: VPIP, PFR, 3Bet, WTSD, bb/100

## Tareas
- [ ] Crear página Dashboard.tsx en features/dashboard/
- [ ] Implementar header con resumen (profit total, manos totales)
- [ ] Crear componente StatCard.tsx para KPIs
- [ ] Implementar grid de KPIs (VPIP, PFR, 3Bet, WTSD)
- [ ] Añadir indicadores de tendencia (up/down arrows)
- [ ] Integrar con usePlayerStats hook
- [ ] Añadir filtros de fecha/stake
- [ ] Implementar skeleton loading

## Criterios de Aceptación
- Dashboard muestra datos reales del API
- KPIs se actualizan con filtros
- Loading state visible
- Colores indican rendimiento (verde/rojo)

## KPIs Principales
- VPIP (Voluntarily Put In Pot)
- PFR (Pre-Flop Raise)
- 3Bet percentage
- WTSD (Went To ShowDown)
- bb/100 (winrate)
"@
        Labels = "enhancement,frontend,fase-3,dashboard" 
    },
    @{ 
        Title = @"
3.3.2 Implementar gráfico de beneficios con Recharts
"@
        Body = @"
Desarrollar el gráfico de evolución de beneficios (bankroll) con Recharts.

## Contexto
- Fase 3.3: Feature Stats - Dashboard
- Mostrar Net Won y All-in EV
- Filtrable por rango de fechas

## Tareas
- [ ] Instalar recharts
- [ ] Crear componente ProfitChart.tsx
- [ ] Implementar gráfico de líneas con eje temporal
- [ ] Añadir línea de Net Won (beneficio real)
- [ ] Añadir línea de All-in EV (beneficio esperado)
- [ ] Implementar tooltip con detalles
- [ ] Añadir leyenda
- [ ] Configurar colores Dark Mode
- [ ] Implementar zoom/pan en rango de fechas

## Criterios de Aceptación
- Gráfico muestra datos históricos
- Dos líneas diferenciadas (Net Won vs EV)
- Tooltip muestra valores exactos
- Responsive a diferentes tamaños

## Colores
- Net Won: accent-violet (#8B5CF6)
- All-in EV: slate-400 (#94A3B8)
- Background: slate-800
"@
        Labels = "enhancement,frontend,fase-3,dashboard,charts" 
    },
    @{ 
        Title = @"
3.3.3 Implementar lista de manos recientes
"@
        Body = @"
Desarrollar la lista de manos recientes con filtros y navegación al Replayer.

## Contexto
- Fase 3.3: Feature Stats - Dashboard/Sessions
- Mostrar últimas N manos con resultado
- Click navega al Hand Replayer

## Tareas
- [ ] Crear componente HandsList.tsx
- [ ] Implementar tabla/lista con columnas (fecha, stake, resultado, posición)
- [ ] Añadir indicador de ganada/perdida (color)
- [ ] Implementar paginación o infinite scroll
- [ ] Añadir filtros (stake, posición, resultado)
- [ ] Implementar click para navegar a /hands/:id
- [ ] Integrar con useRecentHands hook
- [ ] Añadir búsqueda por hand_id

## Criterios de Aceptación
- Lista carga manos del API
- Filtros funcionan correctamente
- Click navega al Replayer
- Loading y empty states visibles

## Columnas
- Fecha/Hora
- Stake (ej: NL10)
- Posición (BTN, SB, etc.)
- Resultado (+5.50€ o -2.00€)
- Acción principal (ej: 3bet, call, fold)
"@
        Labels = "enhancement,frontend,fase-3,dashboard" 
    },
    @{ 
        Title = @"
3.3.4 Implementar matriz de rangos 13x13
"@
        Body = @"
Desarrollar el componente de matriz de rangos 13x13 con mapas de calor.

## Contexto
- Fase 3.3: Feature Stats - Análisis de Rangos
- Matriz clásica de starting hands
- Drag-to-select para edición

## Tareas
- [ ] Crear componente RangeMatrix.tsx
- [ ] Renderizar grid 13x13 con CSS Grid
- [ ] Implementar colores de calor según frecuencia
- [ ] Añadir etiquetas de filas/columnas (A, K, Q, ..., 2)
- [ ] Diferenciar suited (arriba diagonal) vs offsuit (abajo diagonal)
- [ ] Implementar drag-to-select para selección múltiple
- [ ] Añadir hover con tooltip de frecuencia
- [ ] Crear presets de rangos (EP Open, BTN Open, etc.)

## Criterios de Aceptación
- Matriz renderiza correctamente 169 celdas
- Colores de calor visibles
- Drag-to-select funciona
- Performance sin re-renders excesivos

## Layout de Matriz
- Diagonal: Pocket pairs (AA, KK, QQ, ...)
- Arriba diagonal: Suited combos (AKs, AQs, ...)
- Abajo diagonal: Offsuit combos (AKo, AQo, ...)
"@
        Labels = "enhancement,frontend,fase-3,stats,ranges" 
    }
)
# -----------------------------------

Write-Host "`n[Iniciando creación de lote de issues...]" -ForegroundColor Cyan

if ($issues.Count -eq 0) {
    Write-Warning "La lista de issues está vacía."
    exit
}

foreach ($issue in $issues) {
    Write-Host "Creando: $($issue.Title)..." -NoNewline
    
    # Extraer valores
    $title = $issue.Title
    $body = $issue.Body
    $labels = $issue.Labels
    
    # Ejecutar gh issue create usando las variables directamente
    # PowerShell pasa el contenido de las variables (UTF-8) correctamente a gh cli
    $result = gh issue create --title "$title" --body "$body" --label "$labels" 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host " OK" -ForegroundColor Green
    } else {
        Write-Host " ERROR" -ForegroundColor Red
        Write-Host $result -ForegroundColor Yellow
    }
    
    Start-Sleep -Milliseconds 500 
}

Write-Host "`n[Proceso finalizado.]" -ForegroundColor Cyan
