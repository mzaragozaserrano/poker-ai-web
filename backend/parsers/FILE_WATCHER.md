# File Watcher - Sistema de Detección Automática de Historiales

Sistema de file watching para detectar automáticamente nuevos archivos de historial de Winamax usando el crate `notify`.

## Características

- **File Watching Automático**: Usa `notify::RecommendedWatcher` para compatibilidad multiplataforma (optimizado para Windows)
- **Filtrado Inteligente**: Solo procesa archivos `.txt` de historiales
- **Deduplicación**: Hash MD5 para evitar procesamiento duplicado de archivos
- **Cola de Procesamiento**: `mpsc::channel` para manejo asíncrono de archivos detectados
- **Retry Logic**: Manejo de archivos bloqueados con backoff exponencial (máximo 3 reintentos)
- **Integración con Rayon**: Se conecta directamente con `ParallelProcessor` para procesamiento multihilo

## Ruta Monitoreada

```
C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history
```

## Uso Básico

### 1. Con Callback Personalizado

```rust
use poker_parsers::{FileWatcherBuilder, WinamaxParser};
use std::path::PathBuf;

let watcher = FileWatcherBuilder::new()
    .watch_path(PathBuf::from(r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history"))
    .max_retries(3)
    .retry_delay_ms(100)
    .build();

watcher.start(|file_path| {
    println!("Nuevo archivo: {:?}", file_path);
    
    // Procesar manualmente
    let content = std::fs::read_to_string(&file_path).unwrap();
    let mut parser = WinamaxParser::new();
    let result = parser.parse(&content);
    
    println!("Manos parseadas: {}", result.hands.len());
}).unwrap();
```

### 2. Con Integración Automática a ParallelProcessor

```rust
use poker_parsers::{FileWatcherBuilder, ParallelProcessor, ProcessingConfig};
use std::path::PathBuf;

// Configurar procesador paralelo (16 hilos)
let processor = ParallelProcessor::new(ProcessingConfig::default());

// Configurar watcher
let watcher = FileWatcherBuilder::new()
    .watch_path(PathBuf::from(r"C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history"))
    .build();

// Iniciar con integración automática
watcher.start_with_processor(processor).unwrap();
```

## Configuración

### WatcherConfig

```rust
pub struct WatcherConfig {
    /// Ruta del directorio a monitorear
    pub watch_path: PathBuf,
    
    /// Número máximo de reintentos para archivos bloqueados (default: 3)
    pub max_retries: u32,
    
    /// Delay inicial para retry en milisegundos (default: 100ms)
    pub retry_delay_ms: u64,
    
    /// Usar backoff exponencial para retries (default: true)
    pub use_exponential_backoff: bool,
}
```

### Builder Pattern

```rust
let watcher = FileWatcherBuilder::new()
    .watch_path(PathBuf::from("/custom/path"))
    .max_retries(5)
    .retry_delay_ms(200)
    .use_exponential_backoff(false)
    .build();
```

## Arquitectura

### Flujo de Detección

1. **notify::Watcher** detecta eventos `Create` y `Modify` en el directorio
2. **Filtrado**: Solo procesa archivos con extensión `.txt`
3. **Retry Logic**: Intenta leer el archivo con backoff exponencial si está bloqueado
4. **Hash MD5**: Calcula hash del contenido para deduplicación
5. **Verificación**: Comprueba si el hash ya fue procesado
6. **Cola**: Envía el archivo a la cola de procesamiento via `mpsc::channel`
7. **Callback**: Ejecuta el callback del usuario con la ruta del archivo

### Threads

- **Thread 1**: Escucha eventos de `notify` y filtra archivos
- **Thread 2**: Procesa archivos de la cola y ejecuta callbacks

### Deduplicación

El sistema mantiene un `HashSet<String>` con los hashes MD5 de archivos ya procesados:

```rust
processed_hashes: Arc<Mutex<HashSet<String>>>
```

Esto previene:
- Procesamiento duplicado del mismo archivo
- Procesamiento de archivos modificados sin cambios reales
- Race conditions en detección de eventos múltiples

### Retry Logic

Para manejar archivos que Winamax está escribiendo:

```rust
Intento 1: Delay 100ms
Intento 2: Delay 200ms (exponencial)
Intento 3: Delay 400ms (exponencial)
Error: FileLocked después de 3 intentos
```

## Ejemplos

### Ejecutar Demo con Procesador Integrado

```bash
cd backend
cargo run --example file_watcher_demo
```

### Ejecutar Demo Simple con Callback

```bash
cd backend
cargo run --example file_watcher_simple
```

## Tests

```bash
cd backend
cargo test --package poker-parsers --lib file_watcher
```

### Tests Disponibles

- `test_is_txt_file`: Verifica filtrado de extensiones
- `test_read_and_hash_file`: Verifica cálculo de MD5
- `test_read_and_hash_same_content_produces_same_hash`: Verifica consistencia de hash
- `test_watcher_config_default`: Verifica configuración por defecto
- `test_builder_pattern`: Verifica el builder pattern

## Dependencias

```toml
[dependencies]
notify = "6.1"  # File watching
md5 = "0.7"     # Hash para deduplicación
rayon = "1.7"   # Paralelización (integración)
```

## Limitaciones Conocidas

1. **Windows Only (optimizado)**: Aunque `notify` es multiplataforma, la ruta por defecto es específica de Windows
2. **Blocking**: El método `start()` bloquea el hilo actual (usar en thread separado si es necesario)
3. **Sin Persistencia**: Los hashes se pierden al reiniciar (no se persisten en disco)

## Roadmap Futuro

- [ ] Persistencia de hashes en archivo local (evitar reprocesamiento en reinicio)
- [ ] Soporte para múltiples directorios simultáneos
- [ ] Métricas de rendimiento (archivos/segundo, latencia de detección)
- [ ] Integración con WebSocket para notificar a la UI en tiempo real
- [ ] Modo "catch-up" para procesar archivos existentes al inicio

