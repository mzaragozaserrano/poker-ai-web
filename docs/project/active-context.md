# TAREA ACTIVA: ISSUE #4

## Título
feat(1.2.1): Desarrollo de Máquina de Estados Finitos (FSM) para Winamax

## Descripción y Requisitos
Implementar el parser FSM para interpretar el formato de texto de Winamax sin usar Regex costosas. El parser debe ser capaz de procesar historiales de Cash Games 6-max, manejar múltiples idiomas (Inglés/Francés/Español) y casos límite específicos de Winamax.

## Estado: EN PROGRESO

## Tareas Pendientes
- [ ] Analizar formato de historiales Winamax según winamax-spec.md
- [ ] Diseñar estados del FSM (Initial, Header, Preflop, Flop, Turn, River, Summary)
- [ ] Implementar FSM en backend/parsers/src/lib.rs
- [ ] Manejar casos límite (cambios de mesa, sit-out, side pots)

## Criterios de Aceptación
- [ ] El FSM parsea correctamente historiales de Cash Game 6-max
- [ ] No se usan Regex en loops críticos de rendimiento
- [ ] Se manejan correctamente todos los casos límite documentados
- [ ] El parser extrae todas las acciones y metadatos necesarios

## Rama
feat/issue-4-fsm-winamax-parser