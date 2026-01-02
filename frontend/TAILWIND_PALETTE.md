# Paleta de Colores - Tailwind CSS Dark Mode

## Resumen Ejecutivo

La aplicación está configurada en **Dark Mode exclusivamente** utilizando Tailwind CSS con una paleta personalizada optimizada para análisis de poker. No hay opción de light mode.

**Referencia Principal:** `docs/project/ui-foundations.md`

---

## Paleta Base (Slate)

### Background Principal
```
--color-slate-950: #0F172A
bg-slate-950
```
Color de fondo profundo para la interfaz principal. Proporciona contraste óptimo y reduce la fatiga visual durante sesiones prolongadas.

### Surface / Cards
```
--color-slate-800: #1E293B
bg-slate-800
```
Fondo para tarjetas, paneles, modales y cualquier superficie secundaria.

### Borders / Divisores
```
--color-slate-700: #334155
border-slate-700
```
Color para bordes sutiles que organizan la información visual.

---

## Colores de Acciones de Poker

Estos colores tienen significado específico en el contexto de análisis de manos.

### Raise (Agresividad)
```
--color-poker-raise: #EF4444
bg-poker-raise
text-poker-raise
.badge-raise
```
**Rojo** - Indica acciones agresivas (raise, all-in). Usado en:
- Botones de acción
- Badges de manos jugadas agresivamente
- Indicadores en la matriz de rangos

### Call (Pasividad)
```
--color-poker-call: #3B82F6
bg-poker-call
text-poker-call
.badge-call
```
**Azul** - Indica acciones pasivas (call, check). Usado en:
- Botones de acción
- Badges de manos jugadas pasivamente
- Indicadores en rangos

### Fold (Descarte)
```
--color-poker-fold: #64748B
bg-poker-fold
text-poker-fold
.badge-fold
```
**Gris** - Indica fold o descarte. Usado en:
- Manos descartadas (opacidad al 20% en grids)
- Posiciones foldeadas
- Estados inactivos

### Equity High (Probabilidad Alta)
```
--color-poker-equity-high: #10B981
bg-poker-equity-high
text-poker-equity-high
.badge-equity
```
**Verde** - Indica alta probabilidad de victoria. Usado en:
- Indicadores de equity
- Gráficos de ganancia esperada
- Señales positivas

---

## Acento (Hero / Primario)

```
--color-accent-violet: #8B5CF6
bg-accent-violet
text-accent-violet
```
**Violeta** - Color para acciones primarias y elementos del Hero. Usado en:
- Botones primarios
- Links
- Highlights del jugador (thesmoy)
- Elementos de enfoque

---

## Variables CSS Disponibles

Ubicación: `frontend/src/styles/variables.css`

### Formato de Referencia

Todas las variables están disponibles en CSS como:

```css
/* En archivos CSS */
background-color: var(--color-poker-raise);
color: var(--color-text-primary);
border-color: var(--color-border);
```

### Variables Principales

| Variable | Color | Valor |
|----------|-------|-------|
| `--color-slate-950` | Slate | #0F172A |
| `--color-slate-800` | Slate | #1E293B |
| `--color-slate-700` | Slate | #334155 |
| `--color-poker-raise` | Rojo | #EF4444 |
| `--color-poker-call` | Azul | #3B82F6 |
| `--color-poker-fold` | Gris | #64748B |
| `--color-poker-equity-high` | Verde | #10B981 |
| `--color-accent-violet` | Violeta | #8B5CF6 |

---

## Clases Tailwind Personalizadas

Ubicación: `frontend/src/index.css`

### Componentes Base

```html
<!-- Botón primario -->
<button class="btn btn-primary">Acción</button>

<!-- Tarjeta -->
<div class="card">Contenido</div>

<!-- Badges de acciones -->
<span class="badge-raise">RAISE</span>
<span class="badge-call">CALL</span>
<span class="badge-fold">FOLD</span>
<span class="badge-equity">EQUITY HIGH</span>
```

### Colores Tailwind Nativos

Todos los colores están disponibles como clases Tailwind:

```html
<!-- Background -->
<div class="bg-slate-950">Principal</div>
<div class="bg-slate-800">Surface</div>
<div class="bg-poker-raise">Raise</div>

<!-- Texto -->
<p class="text-slate-200">Texto primario</p>
<p class="text-poker-call">Texto en azul</p>

<!-- Bordes -->
<div class="border border-slate-700">Con borde</div>
```

---

## Ejemplos de Uso

### Componente Button (Raise)

```typescript
import React from 'react';

export const RaiseButton: React.FC = () => {
  return (
    <button className="btn bg-poker-raise text-white hover:opacity-90">
      RAISE
    </button>
  );
};
```

### Componente Card

```typescript
export const StatCard: React.FC<{ title: string; value: string }> = ({ title, value }) => {
  return (
    <div className="card">
      <h3 className="text-slate-200 font-semibold">{title}</h3>
      <p className="text-accent-violet text-lg font-bold">{value}</p>
    </div>
  );
};
```

### Matriz de Rangos (Grid 13x13)

```typescript
export const RangeMatrix: React.FC<{ data: number[][] }> = ({ data }) => {
  return (
    <div className="grid grid-cols-13 gap-1 bg-slate-800 p-4 rounded-lg">
      {data.map((row, i) =>
        row.map((value, j) => (
          <div
            key={`${i}-${j}`}
            className={`
              aspect-square rounded
              ${value > 0.7 ? 'bg-poker-equity-high' : ''}
              ${value > 0.4 ? 'bg-poker-call' : ''}
              ${value > 0 ? 'bg-poker-raise' : 'bg-poker-fold opacity-20'}
            `}
          />
        ))
      )}
    </div>
  );
};
```

---

## Modo Oscuro - Configuración

### Forzado a Nivel de Proyecto

El dark mode está **forzado como único modo** en:

1. **tailwind.config.js**
   ```javascript
   darkMode: 'class'
   ```

2. **index.css**
   ```css
   :root {
     color-scheme: dark;
   }
   ```

3. **No hay componentes de toggle** para cambiar a light mode.

### Por qué Dark Mode Only?

- **Fatiga visual:** Reduce la fatiga visual durante sesiones prolongadas de análisis
- **Contraste:** Mejor contraste para números y gráficos
- **Profesionalismo:** Estándar en herramientas de análisis profesionales
- **Privacidad visual:** Ideal para entornos oscuros (focus)

---

## Validación de Consistencia

Todos los colores en esta paleta están definidos en tres lugares:

1. ✓ `frontend/tailwind.config.js` - Configuración de Tailwind
2. ✓ `frontend/src/styles/variables.css` - Variables CSS
3. ✓ `frontend/src/index.css` - Componentes personalizados
4. ✓ `docs/project/ui-foundations.md` - Especificación de negocio

**Cambiar paleta:** Actualizar en los 3 archivos de código + documentación.

---

## Guía de Mantenimiento

### Agregar nuevo color

Si necesitas agregar un nuevo color:

1. Agregar en `tailwind.config.js` en la sección `colors`
2. Agregar variable CSS en `frontend/src/styles/variables.css`
3. Documentar en este archivo (TAILWIND_PALETTE.md)
4. Actualizar `docs/project/ui-foundations.md` si aplica

### Cambiar colores existentes

1. Cambiar valor en `tailwind.config.js`
2. Cambiar variable en `frontend/src/styles/variables.css`
3. Buscar usos en componentes: `grep -r "poker-raise" src/`
4. Probar visualmente en todos los componentes

---

## Herramientas Útiles

### Verificar disponibilidad de clases

```bash
# En cualquier componente TSX
className="bg-poker-raise"  // ✓ Válido
className="text-poker-call"  // ✓ Válido
className="border-poker-fold"  // ✓ Válido
```

### Convertir variables CSS a Tailwind

```css
/* Usa variable CSS */
background-color: var(--color-poker-raise);

/* O usa clase Tailwind directamente */
className="bg-poker-raise"
```

---

## Contacto & Cambios

Para cambios en la paleta de colores:
- Documentar en Issue de GitHub
- Actualizar todos los 3 archivos de código
- Notificar al equipo en la Pull Request

**Última actualización:** Issue #37 - Fase 3.1 Base de SPA

