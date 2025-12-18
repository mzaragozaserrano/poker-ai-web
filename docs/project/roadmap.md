# Roadmap Detallado de Implementación: Winamax Analyzer

Este roadmap desglosa las fases de desarrollo integrando los requisitos de hardware (Ryzen 3800X / 64GB RAM) y el stack tecnológico híbrido.

---

## Fase 1: El Núcleo e Infraestructura de Datos (Semanas 1-4)
El objetivo es establecer la tubería de ingesta de datos de Winamax y la persistencia analítica.

### 1.1 Configuración del Entorno Multilingüe
- [ ] Inicialización de workspace de **Rust** (Cargo) para el núcleo de procesamiento.
- [ ] Configuración de entorno **Python** (Poetry/Conda) para FastAPI y `PyO3`.
- [ ] Setup de **React + Vite + TypeScript** para el frontend.

### 1.2 Parser de Historiales Winamax (Rust)
- [ ] Desarrollo de la **Máquina de Estados Finitos (FSM)** para el formato de texto de Winamax.
- [ ] Implementación de lectura optimizada mediante **string slicing** y prefijos para evitar Regex costosas.
- [ ] Integración de **Rayon** para paralelizar la ingesta masiva en los 16 hilos del Ryzen 3800X.
- [ ] Desarrollo del sistema de detección de archivos con la crate `notify` para procesamiento en background.

### 1.3 Almacenamiento Analítico (DuckDB)
- [ ] Diseño del **Esquema Star** en DuckDB (tablas `hands_actions` y `hands_metadata`).
- [ ] Implementación de la capa de persistencia en formato **Parquet**.
- [ ] Configuración de la base de datos para operar íntegramente **in-memory** aprovechando los 64GB de RAM.

---

## Fase 2: Motor Matemático y Capa de Servicio (Semanas 5-8)
Desarrollo de la lógica de negocio crítica y la comunicación entre lenguajes.

### 2.1 Motor de Evaluación de Manos (Rust)
- [ ] Implementación del algoritmo de evaluación (Cactus Kev o variante OMPEval).
- [ ] Pre-calculado de la **Perfect Hash Table** de 7 cartas en RAM (aprox. 133M de combinaciones) para búsquedas $O(1)$.
- [ ] Desarrollo del simulador **Monte Carlo** utilizando intrínsecos **SIMD AVX2** del procesador Ryzen.

### 2.2 Orquestación y API (FastAPI + PyO3)
- [ ] Creación del puente **FFI** para exponer las funciones de Rust a Python sin sobrecarga de serialización.
- [ ] Desarrollo de endpoints REST de FastAPI para consultas de estadísticas (`VPIP`, `PFR`, `3Bet`) y datos históricos.

---

## Fase 3: Interfaz de Usuario y Visualización (Semanas 9-12)
Construcción de la experiencia visual en **Modo Oscuro** y el reproductor de alto rendimiento.

### 3.1 Base de la SPA (React)
- [ ] Implementación del sistema de diseño en **Modo Oscuro** (Slate-950/800).
- [ ] Creación de dashboards para estadísticas agregadas y gráficas de beneficios con Recharts.

### 3.2 Hand Replayer (HTML5 Canvas) - Análisis Post-Juego
- [ ] Desarrollo del reproductor de manos históricas utilizando **React-Konva** para renderizado por GPU.
- [ ] Implementación de la máquina de estados para animaciones fluidas a **60 FPS**.
- [ ] Controles de reproducción (Play, Pause, Step-by-step, velocidad ajustable) para análisis detallado de decisiones pasadas.
- [ ] Visualización de **Matrices de Rangos 13x13** con mapas de calor dinámicos basados en la posición para contexto analítico.

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