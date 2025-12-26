# Winamax Analyzer - Plataforma de Análisis de Póker Local-First

Una plataforma web de análisis de póker de alto rendimiento diseñada específicamente para el ecosistema de Winamax. Optimizada para hardware de nivel entusiasta (Ryzen 7 3800X / 64GB RAM) para ofrecer latencia cero y simulaciones de equidad masivas.

La aplicación opera íntegramente de forma local para garantizar la privacidad de los datos y el cumplimiento estricto de los ToS de Winamax, funcionando como una herramienta de análisis pasivo sin asistencia prohibida (RTA).

## Características Principales

- **Ingesta Ultra-Rápida**: Parser basado en FSM (Máquina de Estados Finitos) y multihilo que aprovecha los 16 hilos del Ryzen 3800X mediante Rayon
- **Motor Analítico SIMD**: Cálculos de equidad Monte Carlo optimizados con instrucciones AVX2 para procesar múltiples manos por ciclo de reloj
- **Almacenamiento In-Memory**: DuckDB configurado para operar 100% en RAM (64GB), eliminando cuellos de botella de disco
- **Visualización de Alto Rendimiento**: Interfaz React con Hand Replayer renderizado en HTML5 Canvas (Konva) a 60 FPS
- **Análisis Post-Juego**: Estadísticas agregadas (VPIP, PFR, 3Bet) y visualización de manos históricas para análisis detallado
- **Análisis de Rangos**: Comparación de acciones reales con rangos estratégicos GTO definidos en formato HandRangeDSL
- **Modo Oscuro**: Interfaz diseñada específicamente para reducir la fatiga visual durante sesiones prolongadas

## Stack Tecnológico

### Backend Core
- **Rust**: Núcleo de procesamiento de alto rendimiento
  - Parsing de historiales Winamax mediante FSM
  - Evaluación de manos con Perfect Hash Table
  - Simulaciones Monte Carlo con SIMD AVX2
  - File watching con crate `notify`
  - Paralelización con Rayon

### Base de Datos Analítica
- **DuckDB**: Base de datos analítica columnar in-process
- **Apache Parquet**: Almacenamiento inmutable y comprimido
- **Estrategia In-Memory**: Optimizada para 64GB RAM

### API y Orquestación
- **Python 3.11+**: FastAPI para endpoints REST
- **PyO3/FFI**: Integración sin sobrecarga entre Rust y Python
- **Apache Arrow**: Intercambio de datos masivos sin penalización de serialización

### Frontend
- **React 18**: Framework UI con Vite
- **TypeScript**: Tipado estático
- **Tailwind CSS**: Estilos con modo oscuro
- **React-Konva**: Renderizado Canvas para Hand Replayer

## Arquitectura

```
/
├── core-backend/             # Núcleo en Rust (Parser, Math Engine, FFI)
│   ├── src/parsers/          # FSM para historiales de Winamax
│   ├── src/math/             # Evaluadores SIMD y Monte Carlo
│   └── src/db/               # Integración con DuckDB
├── server-api/               # FastAPI (Orquestador Python)
│   ├── api/                  # Endpoints REST
│   └── bridge/               # PyO3 / FFI para llamar a Rust
├── frontend/                 # React + TypeScript + Vite
│   ├── src/features/         # Replayer, Stats, Dashboard
│   └── src/lib/canvas/       # Renderizado de mesa con Konva
└── data/                     # Persistencia Parquet y metadatos
```

## Especialización y Alcance

- **Formato de Juego**: Cash Games NLHE exclusivamente
- **Configuración de Mesa**: Optimizado para 6-max
- **Lógica Posicional**: En mesas de 5 jugadores, se omite EP (UTG). Posiciones: MP, CO, BTN, SB, BB
- **Hero (Usuario)**: `thesmoy` - identificador principal para análisis y estadísticas

## Requisitos del Sistema

- **OS**: Windows 10/11
- **CPU**: Ryzen 7 3800X (16 hilos) o equivalente
- **RAM**: 64GB (recomendado para operaciones in-memory)
- **Espacio en Disco**: Variable según volumen de historiales (Parquet comprimido)

## Instalación

### Prerrequisitos

- **Rust 1.70+** (última versión estable)
- Python 3.11+ (para Fase 2 - API)
- Node.js 18+ y npm/pnpm (para Fase 3 - Frontend)
- DuckDB (se instala automáticamente vía dependencias)

### Pasos de Instalación (Fase 1)

```powershell
# Clonar el repositorio
git clone https://github.com/mzaragozaserrano/poker-ai-web.git
cd poker-ai-web

# Configurar Rust backend
cd backend
cargo build --release

# Ejecutar tests para verificar instalación
cargo test --workspace

# Probar parser con archivo real
cargo run --example parse_real_file
```

> **Nota**: Python (server-api) y React (frontend) se configurarán en las Fases 2 y 3 respectivamente.

## Uso

### Probar el Parser (Fase 1)

```powershell
cd backend

# Ejecutar tests completos
cargo test --workspace

# Probar con archivo real de Winamax
cargo run --example parse_real_file

# Ejecutar benchmarks de rendimiento
cargo bench -p poker-parsers
```

### Configuración de Rutas

El sistema detecta automáticamente los historiales de Winamax en:
```
C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history
```

> **Nota**: La API (FastAPI) y el Frontend (React) estarán disponibles en las Fases 2 y 3.

## Verificación de la Fase 1

### Tests Disponibles

```powershell
cd backend

# Tests del parser (FSM, file watcher, bytes parser)
cargo test -p poker-parsers --lib

# Tests de DuckDB (schema, conexión, in-memory)
cargo test -p poker-db --lib

# Tests de integración (Parquet, persistencia completa)
cargo test -p poker-db --test integration_tests

# Benchmark de rendimiento
cargo bench -p poker-parsers
```

### Ejemplos Ejecutables

```powershell
# Parser con archivo real (145 manos de Winamax)
cargo run --example parse_real_file

# File watcher simple con callback
cargo run --example file_watcher_simple

# File watcher con procesador paralelo integrado
cargo run --example file_watcher_demo
```

### Métricas de Éxito Fase 1

- ✓ **145 manos parseadas** sin errores desde archivo real
- ✓ **60+ tests pasando** (48 unitarios + 12 integración)
- ✓ **Compresión Parquet**: 1000 manos < 100KB
- ✓ **Schema init**: < 5 segundos
- ✓ **Particionamiento**: Estructura `year=YYYY/month=MM/day=DD/` operativa

> **Nota**: Los endpoints REST estarán disponibles en la Fase 2 (Motor Matemático y API).

## Estructura de Datos

### Esquema Star Schema (DuckDB)

- **`hands_metadata`**: Tabla de dimensiones con información de cada mano
- **`hands_actions`**: Tabla de hechos con todas las acciones de los jugadores
- **`players`**: Identidad única de jugadores
- **`cash_sessions`**: Sesiones de cash con resultados y EV

Los datos se persisten en formato Parquet con particionamiento por fecha para optimizar consultas masivas.

## Documentación

La documentación completa del proyecto se encuentra en el directorio `docs/`:

### Visión General y Diseño
- **[Project Brief](docs/project/project-brief.md)**: Visión general y objetivos del proyecto
- **[Roadmap](docs/project/roadmap.md)**: Plan de implementación detallado

### Arquitectura
- **[Arquitectura General](docs/project/architecture.md)**: Especificaciones técnicas y estructura del sistema
- **[Arquitectura Frontend](docs/project/frontend-architecture.md)**: Organización interna del módulo React

### Especificaciones de Diseño
- **[UI Foundations](docs/project/ui-foundations.md)**: Guía de diseño, paleta de colores y componentes visuales

### Especificaciones Técnicas
- **[API Specification](docs/specs/api-spec.md)**: Contratos de endpoints REST
- **[DB Schema](docs/specs/db-schema.md)**: Esquema completo de base de datos
- **[Winamax Spec](docs/winamax/winamax-spec.md)**: Especificación del formato de historiales Winamax
- **[Poker Logic](docs/specs/poker-logic.md)**: Lógica de negocio y cálculos analíticos
- **[Range Specification](docs/specs/range-spec.md)**: Formato y especificación de rangos estratégicos
- **[FFI Contract](docs/specs/ffi-contract.md)**: Contratos de comunicación Rust ↔ Python (PyO3/Apache Arrow)
- **[Preflop Ranges](docs/ranges/preflop-ranges.md)**: Rangos estratégicos GTO para situaciones preflop

## Privacidad y Seguridad

- **Local-First**: Todos los datos permanecen en localhost (127.0.0.1)
- **Sin Nube**: No hay transmisión de datos fuera del sistema local
- **Cumplimiento ToS**: Herramienta de análisis pasivo post-juego, sin asistencia en tiempo real (RTA)

## Optimizaciones de Rendimiento

- **Paralelización**: Uso completo de los 16 hilos del Ryzen 3800X mediante Rayon
- **SIMD AVX2**: Instrucciones vectoriales para cálculos masivos de equidad
- **In-Memory**: DuckDB configurado para mantener toda la base de datos en RAM
- **Perfect Hash**: Tabla pre-calculada de 133M combinaciones para evaluación O(1)
- **Canvas Rendering**: Hand Replayer renderizado por GPU a 60 FPS

## Licencia

Ver [LICENSE](LICENSE) para más detalles.

## Estado del Proyecto

**Fase 1 Completada** ✓ - El núcleo e infraestructura de datos está operativo.

### Componentes Implementados

- **Parser Winamax FSM**: Parsing completo de historiales con 145 manos reales procesadas sin errores
- **File Watcher**: Detección automática de nuevos archivos con deduplicación MD5
- **DuckDB In-Memory**: Base de datos analítica configurada para operaciones 100% en RAM
- **Persistencia Parquet**: Almacenamiento comprimido con particionamiento por fecha (1000 manos < 100KB)
- **Tests**: 60+ tests pasando (48 unitarios + 12 integración)

Consulta el [Roadmap](docs/project/roadmap.md) para conocer el estado actual de las fases de implementación.

## Contribuciones

Este es un proyecto personal optimizado para un hardware específico. Las contribuciones son bienvenidas, pero ten en cuenta las restricciones de hardware y el enfoque local-first del proyecto.

