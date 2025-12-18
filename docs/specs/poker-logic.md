# Especificación de Lógica de Póker y Cálculos Analíticos

Este documento define las fórmulas matemáticas y las reglas de negocio para la generación de estadísticas y el cálculo de resultados de Cash Games 6-max.

---

## 1. Definiciones Formales de Estadísticas (Métricas V1)

Para garantizar la precisión, todas las métricas se calculan sobre "oportunidades reales" de realizar la acción.

| Métrica | Definición Lógica | Spots Excluidos |
| :--- | :--- | :--- |
| **VPIP** | (Calls + Raises Voluntarios) / Total Manos. | No cuenta si el jugador foldea en la BB ante un "Walk" (nadie entra al pozo). |
| **PFR** | Cualquier subida pre-flop (Open Raise, 3-bet, etc.) / Total Manos. | No cuenta si no hay opción de subir (ej: all-in previo que no reabre la acción). |
| **3Bet** | Re-raise ante un Open Raise previo / Oportunidades de 3-bet. | Limp-raises no cuentan como 3-bet. |
| **AF** | (Total Bets + Total Raises) / Total Calls. | No se incluyen acciones pre-flop. |
| **WTSD** | Veces que se llega al Showdown / Veces que se ve el Flop. | Foldear en el flop, turn o river invalida el spot. |

### Reglas de Multipot y Limp
- **Limped Pots:** Si el pozo viene limpeado y el jugador checkea en la BB, el VPIP **no aumenta** (la inversión no es voluntaria).
- **Multiway:** Las estadísticas se registran igual independientemente del número de jugadores en el pozo, siempre que se respete la jerarquía de posiciones 6-max.

---

## 2. Ventanas de Agregación y Filtros

Dado que el sistema cuenta con 64GB de RAM, las agregaciones se realizarán "on-the-fly" en DuckDB.

- **Lifetime:** Historial completo del jugador en la base de datos.
- **Sesión Actual:** Manos desde el inicio del proceso `File Watcher` actual.
- **Últimas X Manos:** Ventanas móviles (1k, 5k, 10k) para detectar cambios de tendencia.
- **Por Stake:** Separación estricta (ej: NL2 vs NL10) para evitar sesgos de nivel de juego.

---

## 3. Fiabilidad y Sample Size

Para evitar decisiones basadas en varianza, la interfaz mostrará indicadores de confianza:
- **Muestra Insuficiente (<50 manos):** Las stats se muestran en color gris tenue.
- **Muestra Fiable (>300 manos):** Las stats se muestran en el color estándar de la UI.
- **Muestra Sólida (>2k manos):** Se activa el análisis de tendencias (flechas de subida/bajada).

---

## 4. Cálculo de Líneas de Ganancias y EV

Todos los cálculos monetarios se realizan en **centavos (BigInt)**.

### 4.1 All-in EV (Equity Adjusted)
Cuando un jugador va All-in antes del River y hay Showdown, se calcula el EV ajustado para el Hero **thesmoy**.

$$EV_{Adj} = (\text{Equity} \times \text{Pot Total}) - \text{Inversión Hero}$$

- La **Equity** se obtiene mediante el motor SIMD AVX2 de Rust.
- Si el Hero gana la mano pero su Equity era del 20%, la línea de EV mostrará una desviación negativa respecto al "Net Won".

### 4.2 Cálculo de Rake (Winamax)
Winamax aplica el rake basándose en el pozo final.
- **Rake Real:** Se extrae directamente de la línea `Rake [X]` en el `*** SUMMARY ***` del historial.
- **Rake Contribution:** Se calcula de forma ponderada según el % del pozo aportado por el Hero para obtener el **Rakeback estimado**.

---

## 5. Especialización Posicional (Cash 6-max)
El motor lógico debe forzar el mapeo de 5-max a 6-max omitiendo **EP (UTG)** para que las estadísticas de posición sean comparables entre diferentes tipos de mesas de Winamax.

## 6. Integración de Rangos Estratégicos
El sistema comparará la acción real de **thesmoy** con los rangos definidos en `preflop-ranges.md`.
- **Identificación de Situación:** Se utiliza el `situationId` (ej: `SB_Open_Raise_01`) para vincular la mano parseada con el rango teórico.
- **Análisis de Desviación:** Si la mano jugada tiene una frecuencia de 0.0 en el rango estratégico para esa acción, se marcará como un "Leak" en la UI.