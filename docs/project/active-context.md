# TAREA ACTIVA: ISSUE #5

## Título
feat(1.2.2): Implementación de lectura optimizada con string slicing

## Descripción y Requisitos
Optimizar la lectura de archivos usando técnicas de bajo nivel para máximo rendimiento. Implementar:
- Lectura eficiente con std::fs::read para archivos pequeños (< 10MB) y BufReader con buffer de 64KB para archivos grandes
- Detección de prefijos sin Regex usando bytes
- Extracción de valores numéricos con aritmética de enteros
- Benchmarks con criterion para validar mejoras vs Regex

## Estado: EN PROGRESO

## Tareas Completadas
- [ ] Implementar lectura eficiente de archivos (std::fs::read + BufReader)
- [ ] Implementar detección de prefijos sin Regex (starts_with_bytes, lookup tables)
- [ ] Implementar extracción de valores numéricos (parser de centavos, timestamps)
- [ ] Crear benchmarks con criterion

## Criterios de Aceptación
- [ ] La lectura de archivos es eficiente y no bloquea
- [ ] El parsing usa string slicing en lugar de Regex
- [ ] Los benchmarks muestran mejoras significativas vs Regex (objetivo: < 10ms por archivo de 1000 manos)
- [ ] El código maneja correctamente archivos grandes (> 100MB)

## Archivos a Crear/Modificar
- `backend/parsers/src/file_reader.rs` - Módulo de lectura eficiente
- `backend/parsers/src/bytes_parser.rs` - Parser optimizado usando bytes
- `backend/parsers/benches/benchmark.rs` - Benchmarks con criterion
- `backend/parsers/Cargo.toml` - Agregar criterion y dependencias necesarias

## Rama
feat/issue-5-optimized-file-reading

## PR
(Por crear)