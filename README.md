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

> **Nota**: El proyecto está en desarrollo activo. Consulta el [Roadmap](docs/project/roadmap.md) para el estado actual de implementación.

### Prerrequisitos

- Rust (última versión estable)
- Python 3.11+
- Node.js 18+ y npm/pnpm
- DuckDB (se instala automáticamente vía dependencias)

### Pasos de Instalación

```powershell
# Clonar el repositorio
git clone <repository-url>
cd poker-ai-web

# Configurar Rust (core-backend)
cd core-backend
cargo build --release

# Configurar Python (server-api)
cd ../server-api
python -m venv venv
.\venv\Scripts\Activate.ps1
pip install -r requirements.txt

# Configurar Frontend
cd ../frontend
npm install
```

## Uso

### Iniciar el Servidor API

```powershell
cd server-api
.\venv\Scripts\Activate.ps1
python app/main.py
```

La API estará disponible en `http://127.0.0.1:8000/api/v1`

### Iniciar el Frontend

```powershell
cd frontend
npm run dev
```

### Configuración de Rutas

El sistema detecta automáticamente los historiales de Winamax en:
```
C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history
```

## API Endpoints

### `GET /api/v1/stats/{player_name}`

Obtiene las métricas agregadas de un jugador.

**Query Parameters:**
- `start_date` (ISO-8601): Fecha inicio del filtrado
- `end_date` (ISO-8601): Fecha fin del filtrado
- `stake` (string): Nivel de ciegas (ej: 'NL2')
- `game_type` (enum): 'NLHE' o 'PLO'
- `min_hands` (int): Tamaño de muestra mínimo (default: 1)

**Ejemplo de Respuesta:**
```json
{
  "player": "thesmoy",
  "is_hero": true,
  "summary": {
    "hands": 1540,
    "vpip": 24.5,
    "pfr": 20.1,
    "three_bet": 8.2,
    "wtsd": 28.4,
    "af": 2.5,
    "net_won_cents": 4500,
    "bb_100": 15.2,
    "ev_bb_100": 12.8
  },
  "positional": {
    "BTN": { "vpip": 45.0, "pfr": 38.0, "hands": 250 },
    "SB": { "vpip": 32.0, "pfr": 28.0, "hands": 250 },
    "BB": { "vpip": 12.0, "pfr": 0.0, "hands": 250 },
    "MP": { "vpip": 18.0, "pfr": 15.0, "hands": 250 },
    "CO": { "vpip": 26.0, "pfr": 22.0, "hands": 250 }
  }
}
```

## Estructura de Datos

### Esquema Star Schema (DuckDB)

- **`hands_metadata`**: Tabla de dimensiones con información de cada mano
- **`hands_actions`**: Tabla de hechos con todas las acciones de los jugadores
- **`players`**: Identidad única de jugadores
- **`cash_sessions`**: Sesiones de cash con resultados y EV

Los datos se persisten en formato Parquet con particionamiento por fecha para optimizar consultas masivas.

## Documentación

La documentación completa del proyecto se encuentra en el directorio `docs/`:

- **[Arquitectura](docs/project/architecture.md)**: Especificaciones técnicas y estructura del proyecto
- **[Project Brief](docs/project/project-brief.md)**: Visión general y objetivos del proyecto
- **[Roadmap](docs/project/roadmap.md)**: Plan de implementación detallado
- **[UI Foundations](docs/project/ui-foundations.md)**: Guía de diseño y paleta de colores
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

Este proyecto está en desarrollo activo. Consulta el [Roadmap](docs/project/roadmap.md) para conocer el estado actual de las fases de implementación.

## Contribuciones

Este es un proyecto personal optimizado para un hardware específico. Las contribuciones son bienvenidas, pero ten en cuenta las restricciones de hardware y el enfoque local-first del proyecto.

