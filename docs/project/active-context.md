# TAREA ACTIVA: ISSUE #7

## Título
1.2.4 Sistema de detección de archivos con crate notify

## Descripción y Requisitos
Implementar file watching para detectar nuevos historiales automáticamente. El sistema debe:
- Configurar crate notify para file watching en Windows
- Detectar eventos Create y Modify de archivos .txt
- Evitar procesamiento duplicado usando hash MD5
- Manejar archivos en escritura parcial
- Integrar con sistema de parsing paralelo de Rayon

## Estado: COMPLETADO

## Ruta de Monitoreo
`C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history`

## Tareas Completadas
- [x] Crear módulo file_watcher.rs con notify::Watcher
- [x] Implementar detección de eventos Create/Modify
- [x] Implementar sistema de deduplicación con MD5
- [x] Crear cola de procesamiento con mpsc::channel
- [x] Implementar retry logic para archivos bloqueados
- [x] Integrar con ParallelProcessor de Rayon
- [x] Crear tests unitarios del watcher (5 tests)
- [x] Crear ejemplos de uso (file_watcher_demo y file_watcher_simple)

## Criterios de Aceptación - TODOS SATISFECHOS
- [x] El sistema detecta automáticamente nuevos archivos de historial
- [x] No se procesan archivos duplicados (deduplicación con MD5)
- [x] Se manejan correctamente archivos en escritura (retry con backoff exponencial)
- [x] El file watcher funciona correctamente en Windows (RecommendedWatcher)

## Arquitectura Implementada
- **Watcher**: notify::RecommendedWatcher con RecursiveMode::NonRecursive
- **Filtrado**: Solo archivos .txt mediante extensión
- **Deduplicación**: HashSet<String> con MD5 hashes en Arc<Mutex>
- **Cola**: mpsc::channel para comunicación entre threads
- **Retry**: Máximo 3 intentos con backoff exponencial (100ms, 200ms, 400ms)
- **Threads**: 2 threads (1 para eventos notify, 1 para procesamiento)

## Archivos Creados/Modificados
- `backend/parsers/src/file_watcher.rs` - Módulo principal (NUEVO)
- `backend/parsers/src/lib.rs` - Exportar nuevo módulo
- `backend/parsers/Cargo.toml` - Agregar dependencia md5
- `backend/parsers/examples/file_watcher_demo.rs` - Ejemplo con ParallelProcessor (NUEVO)
- `backend/parsers/examples/file_watcher_simple.rs` - Ejemplo con callback (NUEVO)
- `backend/parsers/FILE_WATCHER.md` - Documentación completa (NUEVO)

## API Principal
```rust
// Uso con callback personalizado
let watcher = FileWatcherBuilder::new()
    .watch_path(PathBuf::from(r"C:\Users\Miguel\..."))
    .max_retries(3)
    .retry_delay_ms(100)
    .build();

watcher.start(|file_path| {
    println!("Nuevo archivo: {:?}", file_path);
}).unwrap();

// Uso con ParallelProcessor integrado
let processor = ParallelProcessor::new(ProcessingConfig::default());
watcher.start_with_processor(processor).unwrap();
```

## Tests Ejecutados
- 30 tests unitarios pasados (5 nuevos del file_watcher)
- 17 doc tests pasados
- Ejemplos compilados exitosamente

## Rama
feat/issue-7-file-watcher

## PR
Pendiente de creación