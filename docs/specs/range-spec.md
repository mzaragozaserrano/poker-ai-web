# Guía de Formato para Archivos de Rangos

Este documento explica el formato estándar para definir situaciones de póker y rangos en archivos Markdown.

## Estructura General del Archivo

Cada archivo `.md` representa una **situación específica** de póker (ej: SB Open Raise, BTN vs CO 3bet, etc.).

```markdown
---
title: "Título descriptivo de la situación"
category: "Categoría principal"
position: "Posición del jugador"
situationId: "identificador_unico"
benchmark_time: 45
tags: ["tag1", "tag2"]
---

## Sección de Acción 1

### Rango de acción específica

**`AA:1,KK:1,AKs:0.8`**

### Rango de acción marginal

**`QQ:0.5,JJ:0.3`**

## Notas Estratégicas (opcional)

Explicaciones adicionales...
```

## Frontmatter (Metadatos YAML)

El frontmatter debe estar al inicio del archivo, delimitado por `---`.

### Campos Obligatorios

- **`title`**: Título descriptivo de la situación
  - Ejemplo: `"SB Open Raise 100bb"`
  
- **`category`**: Categoría de la situación
  - Valores comunes: `"RFI"` (Raise First In), `"3Bet"`, `"4Bet"`, `"Blind Defense"`, `"Squeeze"`
  
- **`situationId`**: Identificador único (slug)
  - Formato: snake_case
  - Ejemplo: `"SB_Open_Raise"`, `"BTN_vs_CO_3bet"`
  
- **`benchmark_time`**: Tiempo de referencia en segundos
  - Usado para calcular el multiplicador de velocidad en el score
  - Ejemplo: `30`, `45`, `60`

### Campos Opcionales

- **`position`**: Posición del jugador
  - Valores: `"SB"`, `"BB"`, `"BTN"`, `"CO"`, `"MP"`, `"UTG"`
  
- **`tags`**: Array de etiquetas
  - Ejemplo: `["6-max", "GTO", "Cash"]`

## Formato de Rangos (Notación Compacta)

### Sintaxis Básica

```
**`mano:frecuencia,mano:frecuencia,...`**
```

- **Rodeado por negritas y backticks**: `**`...`**`
- **Sin espacios**: `AA:1,KK:0.9` (NO `AA: 1, KK: 0.9`)
- **Separado por comas**: `,` entre cada combo

### Formato de Manos

| Tipo | Formato | Ejemplos |
|------|---------|----------|
| **Parejas** | `XX` | `AA`, `KK`, `22` |
| **Suited** | `XYs` | `AKs`, `T9s`, `76s` |
| **Offsuit** | `XYo` | `AKo`, `KQo`, `J8o` |

**Orden de cartas:** A > K > Q > J > T > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2

### Formato de Frecuencias

Las frecuencias son números decimales entre **0.0** y **1.0**:

| Valor | Significado |
|-------|-------------|
| `1` o `1.0` | 100% (acción siempre realizada) |
| `0.8` | 80% (acción realizada 80% de las veces) |
| `0.5` | 50% (acción realizada 50% de las veces) |
| `0.28` | 28% (acción realizada 28% de las veces) |
| `0` o `0.0` | 0% (acción nunca realizada, equivale a no listar la mano) |

## Secciones y Mapeo de Acciones

Cada sección del markdown representa un tipo de acción. El parser identifica la acción basándose en el texto del header.

### Mapeo de Headers a Acciones

| Header (case-insensitive) | ActionType | Descripción |
|---------------------------|------------|-------------|
| "open raise", "open", "rfi" | `RAISE` | Apertura del bote con raise |
| "3bet", "resubir", "reraise" | `RAISE` | 3bet o reraise |
| "4bet" | `RAISE` | 4bet |
| "squeeze" | `RAISE` | Squeeze (3bet sobre open + call) |
| "call", "flat", "pagar" | `CALL` | Call simple |
| "all-in", "shove", "push" | `ALL_IN` | All-in preflop |
| **"marginal call"** | `MARGINAL` | Call marginal (GTO perfecto) |
| **"marginal all-in"**, **"marginal shove"** | `MARGINAL` | All-in marginal (GTO perfecto) |
| **"marginal open raise"**, **"marginal open"** | `MARGINAL` | Open raise marginal (GTO perfecto) |
| **"marginal"** (solo) | `MARGINAL` | Acción marginal genérica |

**IMPORTANTE - "Marginal" como Tipo de Acción:**

El término **"marginal"** es un **tipo de acción independiente** (`MARGINAL`), NO un modificador de frecuencia.

**Características:**
- Manos que **no generan mucho valor esperado** en general
- Se juegan para tener un **GTO perfecto** (evitar explotación, balancear rangos)
- Pueden tener **cualquier frecuencia** (0.0 a 1.0), incluso frecuencia 1.0 (100%)
- Se juegan "de vez en cuando" desde una perspectiva estratégica, pero esto no limita la frecuencia

**Ejemplos:**
- `"marginal call"` → Mapea a `MARGINAL` (no a `CALL`)
- `"marginal all-in"` → Mapea a `MARGINAL` (no a `ALL_IN`)
- `"marginal open raise"` → Mapea a `MARGINAL` (no a `RAISE`)

**Ejemplo de frecuencia:**
- `J3s:1` en "marginal open raise" → `{ hand: "J3s", action: "MARGINAL", frequency: 1.0 }`
- `Q8o:0.05` en "marginal call" → `{ hand: "Q8o", action: "MARGINAL", frequency: 0.05 }`

## Estrategias Mixtas

Una **estrategia mixta** ocurre cuando una mano aparece en múltiples secciones con diferentes acciones.

### Ejemplo

```markdown
### Rango de 3bet
**`AQo:0.28`**

### Rango de marginal call
**`AQo:0.02`**
```

**Interpretación:**
- `AQo` hace 3bet el 28% de las veces
- `AQo` hace call el 2% de las veces
- `AQo` hace fold el 70% de las veces (calculado implícitamente)

### Objeto SolutionRange Resultante

```typescript
{
  "AQo": [
    { hand: "AQo", action: "RAISE", frequency: 0.28 },
    { hand: "AQo", action: "MARGINAL", frequency: 0.02 },  // "marginal call" → MARGINAL
    { hand: "AQo", action: "FOLD", frequency: 0.70 }        // Calculado: 1 - 0.28 - 0.02
  ]
}
```

**Nota:** "marginal call" se mapea a `MARGINAL` (tipo de acción independiente), no a `CALL`. La frecuencia puede ser cualquier valor entre 0.0 y 1.0.

## Cálculo de Fold Implícito

Para cualquier mano, el fold se calcula automáticamente:

$$P(Fold) = 1 - \sum P_{acciones}$$

### Reglas

1. **Manos no listadas explícitamente**: Fold = 100% (1.0)
2. **Manos listadas parcialmente**: Fold = 1.0 - suma de frecuencias
3. **Manos con frecuencias que suman 1.0**: Fold = 0% (nunca fold)

### Ejemplos

| Manos Listadas | Suma Frecuencias | Fold Implícito |
|----------------|------------------|----------------|
| `AA:1` (open) | 1.0 | 0.0 (0%) |
| `KK:0.9` (3bet) | 0.9 | 0.1 (10%) |
| `QQ:0.5` (3bet), `QQ:0.3` (call) | 0.8 | 0.2 (20%) |
| `JJ:0.2` (marginal open) | 0.2 | 0.8 (80%) |
| `72o` (no listada) | 0.0 | 1.0 (100%) |

## Validaciones

El parser debe validar:

1. **Formato de mano**: Solo manos válidas (AA-22, AKs-23s, AKo-32o)
2. **Rango de frecuencias**: 0.0 ≤ frecuencia ≤ 1.0
3. **Suma de frecuencias**: Para cada mano, suma ≤ 1.0
4. **Formato de string**: Sin espacios, solo comas como separadores
5. **Presencia de backticks y negritas**: `**`...`**`

## Ejemplo Completo

Ver archivo: `example_SB_Open_Raise.md`

## Notas de Implementación

### Para el Parser (Fase 2, Tarea 2.2-2.3)

1. Usar `gray-matter` para extraer frontmatter
2. Buscar bloques con regex: `/\*\*`([^`]+)`\*\*/g`
3. Separar por comas y parsear `mano:frecuencia`
4. Mapear headers a `ActionType`
5. Construir `SolutionRange` con las 169 manos
6. Calcular fold implícito para cada mano

### Para la UI (Fase 3-4)

- Mostrar estrategias mixtas con indicadores visuales
- Color principal según la acción con mayor frecuencia
- Tooltip o overlay mostrando distribución completa
- En modo review, resaltar diferencias entre respuesta y solución