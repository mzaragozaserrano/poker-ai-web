# Especificación de UX: Flujos, Layouts e Interactividad

Este documento detalla la experiencia de usuario y el flujo de producto para la plataforma Winamax Analyzer, asegurando una transición fluida desde la ingesta de datos hasta el análisis profundo.

---

## 1. Flujos de Usuario Críticos

### 1.1 Primera Ejecución y Configuración (Onboarding)
1.  **Detección Automática:** Al arrancar, el sistema intenta localizar la carpeta de `thesmoy` en la ruta estándar de AppData.
2.  **Validación de Ruta:** Si no se encuentra, se muestra una pantalla de "Carpeta no encontrada" con un selector de archivos manual.
3.  **Carga Inicial Masiva:** Una vez confirmada la ruta, se inicia la ingesta masiva (Fase 1 del Roadmap).
    - **Visualización:** Barra de progreso con conteo de archivos detectados, manos procesadas por segundo (aprovechando los 16 hilos del Ryzen) y tiempo estimado.
4.  **Finalización:** Al terminar, el sistema redirige automáticamente al Dashboard Principal.

### 1.2 Flujo de Análisis Post-Juego
1.  **Procesamiento en Background:** El `File Watcher` (Rust) detecta automáticamente nuevos archivos de historial mientras el usuario juega, procesándolos en segundo plano.
2.  **Análisis de Sesión:** Tras finalizar la sesión de juego, el usuario accede al Dashboard donde puede ver las manos procesadas.
3.  **Revisión Detallada:** El usuario accede a la "Vista de Sesión" donde todas las manos están disponibles para análisis detallado mediante el Hand Replayer.

---

## 2. Layouts de Pantallas (Modo Oscuro)

### 2.1 Dashboard Principal (Vista General)
- **Header:** Resumen rápido del Hero `thesmoy` (Profit total, bb/100, manos totales).
- **Sidebar:** Navegación entre Dashboard, Sesiones, Torneos, Análisis de Rangos y Ajustes.
- **Centro:** Gráfico de beneficios (Bankroll) usando Recharts con líneas de Net Won y All-in EV.
- **KPI Cards:** Rejilla de tarjetas con VPIP, PFR, 3Bet y WTSD, destacadas con el color de acento púrpura para el Hero.

### 2.2 Vista de Sesión / Detalle de Mano
- **Panel Izquierdo:** Lista filtrable de manos históricas con icono de "ganada/perdida" y tamaño del bote.
- **Panel Central:** **Hand Replayer (Canvas)** a 60 FPS con controles de timeline (Play, Pause, Speed x2/x5/x10) para revisar y analizar manos ya jugadas.
- **Panel Derecho:** Log de acciones detallado y probabilidades de equidad calculadas por el motor SIMD para cada acción de la mano, permitiendo análisis retrospectivo de decisiones.

---

## 3. Interactividad y Componentes Especializados

### 3.1 Matriz de Rangos 13x13
- **Drag-to-select:** Permite seleccionar bloques de manos arrastrando el ratón.
- **Mapas de Calor:** Las celdas se colorean dinámicamente según la frecuencia (ej. 3-bet frequency).
- **Presets:** Botones rápidos para cargar rangos estándar (ej. "Rango Abierto en BTN").
- **Guardado:** Capacidad de guardar rangos personalizados en DuckDB para comparativas futuras.

### 3.2 Visualización de Bounties y ROI
- **ROI Detallado:** Desglose visual entre premios por posición y beneficios por eliminaciones (Bounties).

---

## 4. Estados de la Aplicación y Mensajes

### 4.1 Estados Vacíos (Empty States)
- **Sin Datos:** Ilustración minimalista en escala de grises: "Esperando historiales de Winamax... Juega unas manos para empezar".
- **Filtros sin resultados:** "No se encontraron manos para el stake NL10 en este rango de fechas".

### 4.2 Mensajes de Error y Progreso
- **Error de Ingesta:** Notificación tipo Toast: "Error 102: Formato de archivo irreconocible en Nice 09.txt".
- **Parsing en Curso:** Indicador circular de actividad en la barra de estado con el mensaje: "Optimizando base de datos in-memory (64GB)...".

---

## 5. Distinción Hero vs. Oponentes

Para garantizar claridad analítica inmediata:
- **Hero (`thesmoy`):** Siempre utiliza el color de acento púrpura (`#7C3AED`) en gráficos y filas de tablas.
- **Oponentes:** Colores neutros (Slate/Gray) para evitar distracciones visuales.
- **Filtro "Solo Hero":** Interruptor global en el Dashboard para ocultar estadísticas de oponentes y centrarse en el estudio de leaks propios.