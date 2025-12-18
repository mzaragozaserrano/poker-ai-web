# Configuración de Rust - Guía Rápida

## Requisitos Previos

Este proyecto utiliza un core en Rust para procesamiento de alto rendimiento. **Rust debe estar instalado en el sistema.**

### Instalar Rust en Windows

1. **Descargar rustup (instalador oficial)**
   - Ir a [https://rustup.rs/](https://rustup.rs/)
   - Ejecutar el instalador descargado

2. **Seguir las instrucciones de instalación**
   ```
   Seleccionar opción 1 (default installation)
   ```

3. **Verificar instalación**
   ```powershell
   rustc --version
   cargo --version
   ```

## Compilación del Backend

### Compilación Release (Optimizada para Ryzen 3800X)

```powershell
cd backend
cargo build --release
```

**Tiempo estimado:** 5-10 minutos (depende de las dependencias y sistema)

**Optimizaciones aplicadas:**
- `opt-level = 3`: Máxima optimización
- `lto = true`: Link Time Optimization
- `codegen-units = 1`: Monolítico
- AVX2 SIMD habilitado automáticamente

### Compilación Debug (Rápida para desarrollo)

```powershell
cd backend
cargo build
```

**Tiempo estimado:** 2-3 minutos

## Verificación

Después de la instalación, verificar que todo está correcto:

```powershell
cd backend
cargo check
```

Si no hay errores, el workspace está correctamente configurado.

## Troubleshooting

### Error: "cargo: command not found"
- Reiniciar PowerShell o terminal después de instalar Rust
- Verificar que `C:\Users\<usuario>\.cargo\bin` está en PATH

### Errores de dependencias
```powershell
# Actualizar Rust y dependencias
rustup update
cargo update
```

### Compilación lenta
- Los primeros builds toman más tiempo por la descarga de dependencias
- Los builds subsecuentes son más rápidos (caché de cargo)

## Próximos Pasos

Una vez compilado el backend Rust, el siguiente paso es:
1. Integrar el backend con FastAPI (PyO3 / FFI)
2. Compilar módulos nativos de Python
3. Ejecutar la plataforma completa (Rust + Python + React)

Ver `backend/README.md` para más detalles sobre la arquitectura del backend.

