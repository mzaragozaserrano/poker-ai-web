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

## Estado: EN PROGRESO

## Ruta de Monitoreo
`C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history`

## Tareas Pendientes
- [ ] Crear módulo file_watcher.rs con notify::Watcher
- [ ] Implementar detección de eventos Create/Modify
- [ ] Implementar sistema de deduplicación con MD5
- [ ] Crear cola de procesamiento con mpsc::channel
- [ ] Implementar retry logic para archivos bloqueados
- [ ] Integrar con ParallelProcessor de Rayon
- [ ] Crear tests unitarios del watcher
- [ ] Crear ejemplo de uso del watcher

## Criterios de Aceptación
- [ ] El sistema detecta automáticamente nuevos archivos de historial
- [ ] No se procesan archivos duplicados
- [ ] Se manejan correctamente archivos en escritura
- [ ] El file watcher funciona correctamente en Windows

## Rama
feat/issue-7-file-watcher

## PR
Pendiente de creación