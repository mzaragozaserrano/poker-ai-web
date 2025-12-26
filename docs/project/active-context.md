# FASE 2 EN PROGRESO - Motor Matemático

## Estado General
La Fase 2 (Motor Matemático y Capa de Servicio) ha comenzado. Actualmente trabajando en el evaluador de manos.

## Tarea Actual: ISSUE #22
2.1.1 Implementar algoritmo de evaluación de manos en Rust

## Estado: EN PROGRESO

## Tareas
- [ ] Investigar e implementar algoritmo Cactus Kev o OMPEval
- [ ] Crear módulo hand_evaluator en el workspace de Rust
- [ ] Implementar función para evaluar fuerza de mano de 5-7 cartas
- [ ] Crear tests unitarios con casos conocidos (Royal Flush, Straight, etc.)
- [ ] Benchmarks de rendimiento (objetivo: < 100ns por evaluación)

## Criterios de Aceptación
- El evaluador retorna correctamente el ranking de cualquier combinación de 5-7 cartas
- Tests pasan al 100%
- Performance < 100ns por evaluación en hardware objetivo

## Decisiones de Diseño

### Algoritmo Seleccionado: Two Plus Two / Cactus Kev Híbrido
- **Razón**: Evaluación O(1) mediante lookup tables pre-calculadas
- **Representación de Cartas**: 32-bit integer con bits para rank, suit y prime
- **Lookup Table**: ~32KB para flush detection + rankings
- **7-Card Evaluation**: Iteración sobre 21 combinaciones de 5 cartas

### Estructura de Módulos
```
backend/math/src/
├── lib.rs              # Exports públicos
├── hand_evaluator/
│   ├── mod.rs          # Módulo principal
│   ├── cards.rs        # Representación de cartas y barajas
│   ├── lookup.rs       # Lookup tables pre-calculadas
│   ├── evaluator.rs    # Lógica de evaluación
│   └── hand_rank.rs    # Tipos de ranking
```

## Rama
feat/issue-22-hand-evaluator

## Referencias
- Cactus Kev: https://suffe.cool/poker/evaluator.html
- OMPEval: https://github.com/zekyll/OMPEval

---

## Fase 1 Completada (Resumen)

### Componentes Implementados
- Parser Winamax (FSM, File Watcher, Rayon)
- Base de Datos Analítica (DuckDB, Parquet)
- 60+ tests pasando
