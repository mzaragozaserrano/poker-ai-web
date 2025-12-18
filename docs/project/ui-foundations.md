# Fundamentos de UI
La interfaz está diseñada específicamente en modo oscuro para reducir la fatiga visual durante sesiones prolongadas de estudio y análisis.

## Paleta Base
| Elemento | Color | Uso |
| :--- | :--- | :--- |
| **Background** | `#0F172A` (slate-950) | Fondo principal profundo para contraste óptimo. |
| **Surface** | `#1E293B` (slate-800) | Fondo para tarjetas, paneles y modales. |
| **Accent** | `#8B5CF6` (violet-500) | Color para acciones primarias y picos de éxito en gráficos. |
| **Border** | `#334155` (slate-700) | Divisores sutiles para organizar la información. |

## Colores de Poker (Tableros y Visualización)
| Acción | Color | Uso |
| :--- | :--- | :--- |
| **RAISE** | `#EF4444` (red-500) | Representación visual de agresividad. |
| **CALL** | `#3B82F6` (blue-500) | Representación visual de acciones pasivas. |
| **FOLD** | `#64748B` (slate-500) | Opacidad al 20% en grids para manos descartadas. |
| **EQUITY HIGH** | `#10B981` (emerald-500) | Indicador de alta probabilidad de ganar la mano. |

## Componentes Clave
* **Hand Replayer (Canvas)**: Herramienta de análisis post-juego que reproduce manos históricas con renderizado de alta fidelidad a 60 FPS utilizando **HTML5 Canvas** (Konva) para delegar el procesamiento gráfico a la GPU. Permite revisar, analizar y estudiar decisiones pasadas.
    * *Modo Oscuro*: Tapete verde oscuro bosque con cartas de alto contraste y animaciones fluidas.
    * *Propósito*: Análisis retrospectivo de manos ya jugadas para identificar leaks y mejorar el juego.
* **Matriz de Rangos 13x13**: Grid con `aspect-ratio: 1/1` que utiliza mapas de calor para mostrar tendencias según la posición.