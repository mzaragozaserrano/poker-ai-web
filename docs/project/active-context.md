# TAREA ACTIVA: ISSUE #4

## Título
feat(1.2.1): Desarrollo de Máquina de Estados Finitos (FSM) para Winamax

## Descripción y Requisitos
Implementar el parser FSM para interpretar el formato de texto de Winamax sin usar Regex costosas. El parser debe ser capaz de procesar historiales de Cash Games 6-max, manejar múltiples idiomas (Inglés/Francés/Español) y casos límite específicos de Winamax.

## Estado: COMPLETADO

## Tareas Completadas
- [x] Analizar formato de historiales Winamax según winamax-spec.md
- [x] Diseñar estados del FSM (Initial, Header, Seats, Blinds, Preflop, Flop, Turn, River, Showdown, Summary)
- [x] Implementar FSM en backend/parsers/src/fsm.rs
- [x] Implementar tipos de datos en backend/parsers/src/types.rs
- [x] Manejar casos límite (side pots, showdown, múltiples ganadores)
- [x] Soporte multi-idioma (Inglés/Francés: folds/passe, calls/suit, raises/relance, bets/mise)

## Criterios de Aceptación - TODOS SATISFECHOS
- [x] El FSM parsea correctamente historiales de Cash Game 5-max y 6-max
- [x] No se usan Regex en loops críticos de rendimiento (usa string slicing y prefijos)
- [x] Se manejan correctamente todos los casos límite documentados
- [x] El parser extrae todas las acciones y metadatos necesarios
- [x] Tests unitarios pasan (4 tests: parse_amount, parse_simple_hand, parse_showdown_hand, card_parse)

## Archivos Creados/Modificados
- `backend/parsers/src/types.rs` - Tipos de datos (ParserState, GameType, Street, ActionType, Position, Card, Player, Action, PotInfo, ParsedHand, ParseResult)
- `backend/parsers/src/fsm.rs` - Implementación del parser FSM (WinamaxParser)
- `backend/parsers/src/lib.rs` - Módulo principal con re-exports

## Rama
feat/issue-4-fsm-winamax-parser

## PR
#15