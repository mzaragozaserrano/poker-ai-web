# Project Brief: Winamax Analyzer - Alto Rendimiento

## 1. Overview
Una plataforma web de análisis de póker local-first, diseñada específicamente para el ecosistema de Winamax, que aprovecha hardware de nivel entusiasta (Ryzen 7 3800X / 64GB RAM) para ofrecer latencia cero y simulaciones de equidad masivas.

La aplicación opera íntegramente de forma local para garantizar la privacidad de los datos y el cumplimiento estricto de los ToS de Winamax, funcionando como una herramienta de análisis pasivo sin asistencia prohibida (RTA).

Se compone de cuatro pilares técnicos:
1. **Ingesta (Rust Core):** Parser ultra-rápido basado en FSM y multihilo.
2. **Motor Analítico:** Cálculos de equidad Monte Carlo optimizados con instrucciones SIMD (AVX2).
3. **Almacenamiento (DuckDB):** Base de datos analítica columnar funcionando 100% in-memory.
4. **Visualización (React/Canvas):** Interfaz reactiva en modo oscuro con hand replayer de alto rendimiento para análisis post-juego de manos históricas.

## 2. Problem Statement
Las herramientas comerciales actuales (PokerTracker, Holdem Manager) presentan limitaciones críticas:
1. **Latencia de Datos:** Bases de datos orientadas a filas (PostgreSQL) que sufren con volúmenes masivos.
2. **Infrautilización de Hardware:** No aprovechan arquitecturas multinúcleo modernas ni instrucciones vectoriales.
3. **Privacidad:** Dependencia de nubes o procesos pesados que comprometen la soberanía de los datos.

## 3. Solution & Value Proposition

### Arquitectura Bare-Metal (Optimización Ryzen)
- **Aprovechamiento de 16 Hilos:** Uso de la arquitectura Zen 2 para paralelizar la ingesta de manos mediante un sistema Producer-Consumer en Rust.
- **Estrategia In-Memory (64GB):** DuckDB configurado para mantener toda la base de datos "caliente" en RAM, eliminando el cuello de botella del disco.
- **Cálculo SIMD:** Motor de evaluación de manos que procesa múltiples manos por ciclo de reloj usando intrínsecos de hardware (AVX2).

### Módulo de Análisis
- **Ingesta de Archivos:** Detección automática de nuevos historiales mediante file watching (crate `notify`) para procesamiento post-juego.
- **Análisis Post-Juego:** Visualización de estadísticas agregadas (VPIP, PFR, 3Bet) para análisis detallado después de las sesiones.
- **Reconciliación de Bounties:** Módulo inteligente para cruzar historiales de manos con resúmenes de torneos KO.

## 4. Especialización y Alcance
* **Formato de Juego**: Especializado exclusivamente en **Cash Games NLHE**.
* **Configuración de Mesa**: Optimizado para **6-max**.
* **Lógica Posicional**: En mesas de 5 jugadores, el sistema actuará como un 6-max pero omitiendo **EP (UTG)**. Solo se procesarán las posiciones: **MP, CO, BTN, SB y BB**.
* **Hero (Usuario)**: El jugador central a analizar es **thesmoy**.