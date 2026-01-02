# Guía de Testing Manual - Hand Replayer Controls

## Preparación

1. Iniciar el servidor frontend:
   ```bash
   cd frontend
   npm run dev
   ```

2. Navegar a: `http://localhost:5173/hand-replayer`

## Pruebas Funcionales

### 1. Controles Básicos

#### Play/Pause
- [ ] Click en "Reproducir" - la reproducción debe iniciarse
- [ ] El botón cambia a "Pausa"
- [ ] El estado en la esquina superior derecha debe mostrar "playing"
- [ ] El indicador de progreso debe avanzar automáticamente
- [ ] Click en "Pausa" - la reproducción se detiene
- [ ] El botón cambia a "Reproducir"
- [ ] El estado cambia a "paused"

#### Stop
- [ ] Click en "Stop" (ícono cuadrado) desde cualquier posición
- [ ] Debe volver al inicio (acción 1)
- [ ] Estado debe cambiar a "idle"
- [ ] Indicador de progreso debe estar en 0%

#### Step Forward
- [ ] Desde pausado, click en botón flecha derecha
- [ ] Debe avanzar a la siguiente acción
- [ ] Estado debe ser "paused"
- [ ] La calle debe actualizarse automáticamente si es necesaria
- [ ] Cuando está en la última acción, el botón debe estar deshabilitado (gris)

#### Step Backward
- [ ] Desde pausado con índice > 0, click en botón flecha izquierda
- [ ] Debe retroceder a la acción anterior
- [ ] Estado debe ser "paused"
- [ ] Cuando está en la acción 1, el botón debe estar deshabilitado (gris)

### 2. Selector de Velocidad

- [ ] Click en selector (muestra "1x")
- [ ] Dropdown abre mostrando opciones: 1x, 2x, 5x, 10x
- [ ] Click en cada opción:
  - [ ] "1x" - reproducción a velocidad normal
  - [ ] "2x" - reproducción 2 veces más rápida
  - [ ] "5x" - reproducción 5 veces más rápida
  - [ ] "10x" - reproducción 10 veces más rápida
- [ ] Indicador en botón se actualiza
- [ ] Dropdown se cierra después de seleccionar
- [ ] El timer entre acciones se ajusta correctamente

### 3. Timeline Visual

- [ ] Timeline debe mostrar:
  - [ ] Sección "PREFLOP" con acciones
  - [ ] Sección "FLOP" con acciones (cuando se alcanza)
  - [ ] Sección "TURN" con acciones (cuando se alcanza)
  - [ ] Sección "RIVER" con acciones (cuando se alcanza)

#### Colores de Acciones
- [ ] **Fold** aparece en gris oscuro
- [ ] **Check** aparece en azul
- [ ] **Call** aparece en azul más oscuro
- [ ] **Bet** aparece en ámbar
- [ ] **Raise** aparece en rojo
- [ ] **All-in** aparece en rojo oscuro

#### Interactividad
- [ ] La acción actual tiene un anillo violeta alrededor y está más grande (scale)
- [ ] Las acciones pasadas están semi-transparentes (opacidad menor)
- [ ] Las acciones futuras están más transparentes
- [ ] Click en cualquier acción:
  - [ ] Salta a esa acción
  - [ ] El indicador se actualiza
  - [ ] El estado cambia a "paused"
  - [ ] La calle se actualiza si es necesaria
  - [ ] Las tarjetas comunitarias cambian si es necesaria

### 4. Sincronización Automática

#### Calle Actual
- [ ] Al iniciar, debe mostrar "preflop"
- [ ] Al llegar a la primera acción de flop, cambia a "flop"
- [ ] Al llegar a la primera acción de turn, cambia a "turn"
- [ ] Al llegar a la primera acción de river, cambia a "river"
- [ ] La descripción de la acción se actualiza en el header

#### Cartas Comunitarias
- [ ] En preflop: 0 cartas (vacío)
- [ ] En flop: 3 cartas (Ah, Kd, 7c)
- [ ] En turn: 4 cartas (añade 2s)
- [ ] En river: 5 cartas (añade Qh)

#### Pot
- [ ] Preflop: 150
- [ ] Flop: 450
- [ ] Turn: 850
- [ ] River: 1650

### 5. Indicador de Progreso

- [ ] Barra azul muestra progreso visual
- [ ] Texto "Acción X de Y" es correcto
- [ ] Porcentaje es correcto (calculado como: (index / (total-1)) * 100)
- [ ] Se anima suavemente al cambiar

### 6. Información de Estado

- [ ] Sección inferior muestra:
  - [ ] Estado actual (idle/playing/paused/finished)
  - [ ] Velocidad actual (1x, 2x, 5x, 10x)

### 7. Casos de Uso Integrados

#### Reproducción Completa
1. [ ] Click "Reproducir"
2. [ ] Observar que avanza automáticamente a través de todas las acciones
3. [ ] La calle cambia automáticamente
4. [ ] Las cartas comunitarias aparecen correctamente
5. [ ] Al llegar al final, estado cambia a "finished"

#### Reproducción con Pausa Manual
1. [ ] Click "Reproducir"
2. [ ] Esperar 2-3 segundos
3. [ ] Click "Pausa"
4. [ ] Click "Reproducir" nuevamente
5. [ ] Debe continuar desde donde estaba

#### Cambio de Velocidad Dinámico
1. [ ] Click "Reproducir" a 1x
2. [ ] Esperar 2 acciones
3. [ ] Click en velocidad, seleccionar 5x
4. [ ] Las siguientes acciones deben ir más rápido

#### Navegación Manual Completa
1. [ ] Click "Stop" para ir a inicio
2. [ ] Click repetido "Step Forward" para recorrer todas las acciones
3. [ ] Verificar que timeline se actualiza
4. [ ] Verificar que calle y cartas se actualizan
5. [ ] Click "Step Backward" varias veces
6. [ ] Verificar que todo retrocede correctamente

#### Saltar con Timeline
1. [ ] Click en acción del flop desde preflop
2. [ ] Debe saltar a esa acción inmediatamente
3. [ ] La calle debe cambiar a "flop"
4. [ ] Las cartas comunitarias deben actualizarse
5. [ ] Click en acción del river
6. [ ] Todo debe actualizarse correctamente

### 8. Estados Límite

#### En Inicio (Acción 1)
- [ ] "Step Backward" está deshabilitado (gris)
- [ ] "Stop" está disponible pero sin efecto visible
- [ ] "Step Forward" está habilitado
- [ ] Click en primera acción del timeline no tiene efecto (ya está allí)

#### En Final (Última Acción)
- [ ] "Step Forward" está deshabilitado (gris)
- [ ] "Step Backward" está habilitado
- [ ] Estado muestra "finished"
- [ ] Progreso está en 100%

### 9. Renderizado

#### Responsividad
- [ ] Redimensionar ventana del navegador
- [ ] Los controles se adaptan correctamente
- [ ] El timeline se ajusta horizontalmente
- [ ] El canvas se redimensiona sin problemas
- [ ] Probar en tamaños: 1200px, 800px, 500px

#### Accesibilidad
- [ ] Todos los botones tienen tooltips (title)
- [ ] Los colores tienen suficiente contraste
- [ ] Los textos son legibles en modo oscuro

### 10. Performance

#### Fluidez
- [ ] La reproducción automática es suave (sin stuttering)
- [ ] Cambiar velocidad no causa lag
- [ ] Click en acciones es instantáneo
- [ ] Timeline scroll es fluido

#### A Altas Velocidades
- [ ] Con velocidad 10x, la reproducción es suave
- [ ] No hay saltos entre acciones
- [ ] El indicador de progreso se actualiza correctamente

## Casos Problemáticos Esperados

### Por Diseño (No son bugs)
- [ ] Después de "finished", "Play" reinicia desde el inicio (no es bug)
- [ ] Cambiar velocidad durante "idle" funciona pero no afecta nada hasta "playing"
- [ ] Stop siempre vuelve a index 0 (comportamiento esperado)

## Verificación Final

Después de todas las pruebas:

- [ ] Todos los botones funcionan como se espera
- [ ] La timeline es interactiva y responsive
- [ ] Los colores son correctos y legibles
- [ ] La sincronización automática es correcta
- [ ] No hay errores en la consola del navegador
- [ ] No hay memory leaks (revisar DevTools)

## Comandos Útiles para Testing

```bash
# Desarrollo
npm run dev

# Lint y type check
npm run lint
npm run typecheck

# Build para producción
npm run build

# Preview del build
npm run preview
```

## Notas de Testing

- La demo usa datos hardcodeados en `DEMO_ACTIONS`
- En producción, estos datos vendrán de la API
- El comportamiento debe ser idéntico con datos reales
- Verificar que el código es escalable para manos con 50+ acciones

