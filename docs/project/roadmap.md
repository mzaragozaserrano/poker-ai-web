# Roadmap Detallado de Implementación: Winamax Analyzer

Este roadmap desglosa las fases de desarrollo integrando los requisitos de hardware (Ryzen 3800X / 64GB RAM) y el stack tecnológico híbrido.

---

## Fase 1: El Núcleo e Infraestructura de Datos ✓ COMPLETADA
El objetivo es establecer la tubería de ingesta de datos de Winamax y la persistencia analítica.

### 1.1 Configuración del Entorno Multilingüe
- [x] Inicialización de workspace de **Rust** (Cargo) para el núcleo de procesamiento.
- [ ] Configuración de entorno **Python** (Poetry/Conda) para FastAPI y `PyO3` (Fase 2).
- [ ] Setup de **React + Vite + TypeScript** para el frontend (Fase 3).

### 1.2 Parser de Historiales Winamax (Rust) ✓
- [x] Desarrollo de la **Máquina de Estados Finitos (FSM)** para el formato de texto de Winamax.
- [x] Implementación de lectura optimizada mediante **string slicing** y prefijos para evitar Regex costosas.
- [x] Integración de **Rayon** para paralelizar la ingesta masiva en los 16 hilos del Ryzen 3800X.
- [x] Desarrollo del sistema de detección de archivos con la crate `notify` para procesamiento en background.

**Resultados:**
- 145 manos reales parseadas sin errores
- File watcher con deduplicación MD5 y retry logic
- Benchmarks de rendimiento implementados
- 3 ejemplos ejecutables disponibles

### 1.3 Almacenamiento Analítico (DuckDB) ✓
- [x] Diseño del **Esquema Star** en DuckDB (tablas `hands_actions` y `hands_metadata`).
- [x] Implementación de la capa de persistencia en formato **Parquet**.
- [x] Configuración de la base de datos para operar íntegramente **in-memory** aprovechando los 64GB de RAM.

**Resultados:**
- Persistencia Parquet con compresión ZSTD (1000 manos < 100KB)
- Particionamiento automático por fecha (year=/month=/day=/)
- 60+ tests pasando (48 unitarios + 12 integración)
- Schema init < 5 segundos

---

## Fase 2: Motor Matemático y Capa de Servicio ✓ COMPLETADA
Desarrollo de la lógica de negocio crítica y la comunicación entre lenguajes.

### 2.1 Motor de Evaluación de Manos (Rust) ✓
- [x] Implementación del algoritmo de evaluación (Cactus Kev híbrido).
- [x] Pre-calculado de la **Perfect Hash Table** de 7 cartas en RAM (133M combinaciones) para búsquedas $O(1)$.
- [x] Desarrollo del simulador **Monte Carlo** utilizando intrínsecos **SIMD AVX2** del procesador Ryzen.

**Resultados:**
- Algoritmo Cactus Kev híbrido con evaluador de 5, 6 y 7 cartas
- Perfect Hash Table generada en 24 segundos con Rayon (267MB en disco)
- Búsquedas O(1) en 19.4ns, evaluaciones < 100ns
- Monte Carlo con AVX2 y early stopping (convergencia < 0.1%)

### 2.2 Orquestación y API (FastAPI + PyO3) ✓
- [x] Creación del puente **FFI** para exponer las funciones de Rust a Python sin sobrecarga de serialización.
- [x] Desarrollo de endpoints REST de FastAPI para consultas de estadísticas (`VPIP`, `PFR`, `3Bet`) y datos históricos.
- [x] Sistema WebSocket para push de nuevas manos en tiempo real.

**Resultados:**
- Entorno Python con Poetry configurado (FastAPI, Uvicorn, PyO3/maturin)
- Crate `poker-ffi` con PyO3 (overhead < 1ms)
- Endpoints REST: /stats/player, /hands/recent, /hands/{id}, /equity/calculate
- WebSocket /ws con heartbeat automático y notificación < 500ms
- File Watcher integrado: Rust (notify) -> Python FFI -> WebSocket -> Clientes
- Tests de integración completos (pytest)

---

## Fase 3: Interfaz de Usuario y Visualización (Semanas 9-12)
Construcción de la experiencia visual en **Modo Oscuro** y el reproductor de alto rendimiento.

### 3.1 Base de la SPA (React)
- [ ] Configuración del proyecto React con Vite + TypeScript.
- [ ] Implementación del sistema de diseño en **Modo Oscuro** (Slate-950/800).
- [ ] Creación de componentes base (Button, Card, Modal, Navbar).
- [ ] Configuración de Tailwind CSS con paleta de colores de poker.
- [ ] Integración de React Query para estado del servidor.
- [ ] Configuración de WebSocket hook para conexión con backend.
- [ ] Creación de dashboards para estadísticas agregadas y gráficas de beneficios con Recharts.

### 3.2 Hand Replayer (HTML5 Canvas) - Análisis Post-Juego
- [ ] Desarrollo del reproductor de manos históricas utilizando **React-Konva** para renderizado por GPU.
- [ ] Implementación de la máquina de estados para animaciones fluidas a **60 FPS**.
- [ ] Renderizado de mesa de poker 6-max con posiciones correctas.
- [ ] Sistema de renderizado de cartas (sprites o canvas).
- [ ] Controles de reproducción (Play, Pause, Step-by-step, velocidad ajustable) para análisis detallado de decisiones pasadas.
- [ ] Toggle de formato de cantidades (Big Blinds vs Monedas).
- [ ] Visualización de **Matrices de Rangos 13x13** con mapas de calor dinámicos basados en la posición para contexto analítico.

### 3.3 Feature Stats - Estadísticas y Análisis
- [ ] Vista de estadísticas por jugador con filtros de posición y stake.
- [ ] Gráficos de evolución de bankroll con ECharts/Recharts.
- [ ] Comparación de acciones reales vs rangos GTO.
- [ ] Detección visual de leaks (acciones con frecuencia 0.0 en rangos).

---

## Fase 4: Optimización, Seguridad y Lanzamiento (Semanas 13-16)
Refinamiento final y despliegue local.

### 4.1 Rendimiento y Escalabilidad
- [ ] Pruebas de carga masiva con bases de datos de **10 millones de manos**.
- [ ] Configuración del SO para el uso de **Huge Pages** y optimización del swapping de RAM.
- [ ] Tuning de DuckDB para consultas vectorizadas masivas.

### 4.2 Cumplimiento y Seguridad
- [ ] Verificación de seguridad: Configurar la API para escuchar exclusivamente en `localhost` (127.0.0.1).
- [ ] Implementación de auditoría de logs para asegurar que no hay interacción prohibida con el proceso de Winamax.
- [ ] Empaquetado de la aplicación en un ejecutable local simplificado.