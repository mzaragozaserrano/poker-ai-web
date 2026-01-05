# Poker Analyzer - Empaquetado y Distribución

Este directorio contiene todos los scripts necesarios para empaquetar Poker Analyzer como una aplicación autocontenida para Windows.

## Estructura del Paquete Final

```
poker-analyzer/
├── poker-analyzer.exe          # Launcher principal (ejecutable Rust)
├── config.toml                 # Configuración de la aplicación
├── backend/
│   ├── python311/              # Python embebido (3.11)
│   │   ├── python.exe
│   │   ├── python311.dll
│   │   └── Lib/
│   │       └── site-packages/  # Dependencias Python + poker-ffi
│   └── app/                    # FastAPI server-api
│       └── main.py
├── frontend/
│   └── dist/                   # Build estático de React
│       ├── index.html
│       └── assets/
├── data/                       # Base de datos DuckDB y archivos Parquet
├── logs/                       # Logs de auditoría
├── README.md
└── LICENSE
```

## Requisitos de Build

### Windows

- **Rust** 1.70+ (con cargo)
- **Node.js** 18+ y npm
- **Python** 3.11+ con pip
- **Maturin** (`pip install maturin`)
- **PowerShell** 5.1+ o PowerShell Core 7+

### Linux (Futuro)

- Rust, Node.js, Python (mismas versiones)
- Dependencias de sistema: `libssl-dev`, `pkg-config`

## Scripts de Build

### 1. Build Individual de Componentes

#### Backend Rust
```powershell
.\packaging\build-rust.ps1 [-OutputDir "dist/backend"] [-Verbose]
```

Compila todo el workspace Rust en modo release con optimizaciones específicas para Ryzen 3800X (AVX2, znver2).

#### FFI Wheel
```powershell
.\packaging\build-ffi.ps1 [-OutputDir "dist/wheels"] [-Verbose]
```

Compila poker-ffi como Python wheel (.whl) usando Maturin.

#### Frontend
```powershell
.\packaging\build-frontend.ps1 [-OutputDir "dist/frontend"] [-Verbose]
```

Compila el frontend React con Vite en modo producción.

#### Launcher
```powershell
.\packaging\build-launcher.ps1 [-OutputDir "dist/launcher"] [-Verbose]
```

Compila el launcher Rust (`poker-analyzer.exe`).

### 2. Empaquetado Completo

```powershell
.\packaging\package-windows.ps1 [-OutputDir "release/poker-analyzer"] [-Verbose]
```

Este script orquesta todo el proceso:

1. **Build de componentes**: Ejecuta todos los scripts de build individuales
2. **Python embebido**: Descarga e instala Python 3.11 embeddable
3. **Dependencias Python**: Instala poker-ffi wheel y dependencias de FastAPI
4. **Copia de artefactos**: Organiza todos los archivos en la estructura final
5. **Configuración**: Copia config.toml con valores por defecto
6. **Launcher**: Crea script launcher temporal

**Opciones:**
- `-OutputDir`: Directorio de salida (default: `release/poker-analyzer`)
- `-SkipBuild`: Salta la compilación y usa artefactos existentes
- `-Verbose`: Output detallado

### 3. Crear ZIP de Distribución

```powershell
.\packaging\create-zip.ps1 [-SourceDir "release/poker-analyzer"] [-OutputZip "release/poker-analyzer-windows-x64.zip"] [-Validate]
```

Crea un archivo ZIP autocontenido listo para distribución.

**Opciones:**
- `-SourceDir`: Directorio del paquete empaquetado
- `-OutputZip`: Nombre del archivo ZIP de salida
- `-Validate`: Validar tamaño y contenido del ZIP

## Workflow Completo

Para crear un paquete de distribución desde cero:

```powershell
# 1. Empaquetar aplicación completa
.\packaging\package-windows.ps1 -Verbose

# 2. Crear ZIP de distribución
.\packaging\create-zip.ps1 -Validate

# Resultado: release/poker-analyzer-windows-x64.zip
```

## Configuración

El archivo `config.template.toml` se copia como `config.toml` en el paquete final. Los usuarios pueden editarlo para:

- Ajustar threads de DuckDB según su CPU
- Configurar ruta de historiales de Winamax
- Cambiar puerto del servidor (si hay conflicto)
- Modificar límites de memoria
- Deshabilitar auto-apertura del navegador

**IMPORTANTE**: El host siempre debe ser `127.0.0.1` por seguridad (localhost only).

## Tamaños Objetivo

| Componente | Tamaño Estimado |
|------------|----------------|
| Python embebido + libs | ~80 MB |
| poker-ffi wheel | ~15 MB |
| FastAPI dependencies | ~30 MB |
| Frontend build | ~5 MB |
| Launcher | ~1 MB |
| **Total (sin compresión)** | **~130 MB** |
| **ZIP comprimido** | **~60-80 MB** |

El objetivo es mantener el paquete final **< 500 MB**.

## CI/CD con GitHub Actions

Ver `.github/workflows/release-package.yml` para builds automáticos en cada release tag.

```bash
git tag v0.1.0
git push origin v0.1.0
# GitHub Actions compila y crea release con artifacts
```

## Instalación para Usuario Final

1. Descargar `poker-analyzer-windows-x64.zip`
2. Extraer en cualquier directorio (ej: `C:\PokerAnalyzer\`)
3. Ejecutar `poker-analyzer.exe`
4. Configurar ruta de historiales en `config.toml`
5. El navegador se abrirá automáticamente en `http://127.0.0.1:8000`

**Tiempo estimado de instalación**: < 5 minutos

## Troubleshooting

### Error: Python no encontrado

Verificar que `backend/python311/python.exe` existe. Si no, volver a ejecutar:

```powershell
.\packaging\package-windows.ps1 -SkipBuild
```

### Error: poker-ffi no encontrado

Recompilar wheel:

```powershell
.\packaging\build-ffi.ps1
```

### Error: Frontend no carga

Verificar que `frontend/dist/index.html` existe:

```powershell
.\packaging\build-frontend.ps1
```

### Paquete excede 500MB

Optimizaciones posibles:
- Reducir Python embebido (eliminar bibliotecas no usadas)
- Comprimir assets del frontend
- Eliminar source maps en producción

## Desarrollo

Para probar el paquete localmente sin crear ZIP:

```powershell
.\packaging\package-windows.ps1
cd release\poker-analyzer
.\start-poker-analyzer.ps1
```

## Soporte

- Windows 10/11 x64
- Linux x64 (planned)
- macOS (not planned - use Docker)

## Licencia

MIT OR Apache-2.0 (ver LICENSE en raíz del proyecto)

