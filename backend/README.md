# Backend - Rust High-Performance Core

Núcleo de procesamiento de alto rendimiento para Winamax Analyzer, optimizado para Ryzen 7 3800X.

## Requisitos Previos

### Instalación de Rust

Este proyecto requiere **Rust 1.70 o superior** con soporte para SIMD AVX2.

#### En Windows (Recomendado)

1. Descargar e instalar desde [https://rustup.rs/](https://rustup.rs/)
   ```powershell
   # La instalación automática de rustup
   # Seleccionar opción 1 para instalación estándar
   ```

2. Actualizar Rust a la versión estable más reciente:
   ```powershell
   rustup update stable
   rustup default stable
   ```

3. Verificar instalación:
   ```powershell
   rustc --version
   cargo --version
   ```

## Estructura del Workspace

```
backend/
├── Cargo.toml              # Configuración del workspace
├── parsers/                # FSM para parsing de Winamax
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── fsm.rs         # Máquina de Estados Finitos
│       └── winamax.rs     # Parser específico de Winamax
├── math/                   # Motor matemático (SIMD + Monte Carlo)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── hand_evaluator.rs   # Evaluación de manos con AVX2
│       ├── equity.rs           # Cálculos de equidad
│       └── monte_carlo.rs      # Simulaciones paralelizadas
├── ranges/                 # Parser de rangos (HandRangeDSL)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── dsl_parser.rs       # Parser de rangos Markdown
│       ├── range_analyzer.rs   # Análisis de rangos
│       └── leak_detector.rs    # Detección de desviaciones (leaks)
└── db/                     # Base de datos (DuckDB + Parquet)
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── schema.rs       # Definición de tablas (Star Schema)
        ├── connection.rs   # Conexión DuckDB in-memory
        ├── parquet_io.rs   # I/O de archivos Parquet
        └── query.rs        # Construcción y ejecución de queries
```

## Dependencias Principales

### Paralelización & Performance
- **rayon**: Paralelización automática con 16 hilos (Ryzen 3800X)
- **packed_simd_2**: Intrínsecos AVX2 para montecarlo

### Storage & Datos
- **duckdb**: Base de datos analítica columnar in-process
- **arrow**: Intercambio de datos sin serialización (FFI)
- **parquet**: Persistencia columnar comprimida

### FFI & Integración
- **pyo3**: Bindings para Python (FastAPI)

### Parsing
- **nom**: Parser combinadores (historiales Winamax)
- **regex**: Expresiones regulares optimizadas

## Compilación

### Build Release Optimizado

```powershell
# Compilar con optimizaciones para Ryzen 3800X
cd backend
cargo build --release
```

**Perfiles de optimización configurados:**
- `opt-level = 3`: Máxima optimización
- `lto = true`: Link Time Optimization
- `codegen-units = 1`: Monolítico (mejor optimización)
- SIMD AVX2 habilitado por defecto

### Build Debug

```powershell
# Compilación rápida para desarrollo
cargo build
```

## Testing

```powershell
# Tests unitarios
cargo test

# Tests con output
cargo test -- --nocapture

# Tests específicos de un módulo
cargo test -p poker-parsers
```

## Benchmarking

```powershell
# Ejecutar benchmarks del módulo math
cargo bench -p poker-math
```

## Arquitectura de Perfiles

### Release (Producción)
- Optimización máxima (`opt-level = 3`)
- LTO habilitado
- Símbolos stripped
- Ideal para análisis de volúmenes masivos

### Dev (Desarrollo)
- Sin optimización (`opt-level = 0`)
- Compilación rápida
- Símbolos para debugging

## Próximos Pasos

1. Implementar **parsers/src/fsm.rs** - FSM para historiales Winamax
2. Implementar **math/src/hand_evaluator.rs** - Evaluador con AVX2
3. Implementar **db/src/connection.rs** - Conexión DuckDB
4. Integración con **pyo3** para exposición a Python/FastAPI

## Referencias

- Arquitectura: `docs/project/architecture.md`
- Especificaciones Rust: `docs/specs/ffi-contract.md`
- Winamax Format: `docs/winamax/winamax-spec.md`

