# Especificación de Dominio Winamax

Este documento detalla la lógica de negocio y los patrones técnicos necesarios para interpretar los archivos de historial de Winamax, cubriendo casos estándar y excepciones de ingeniería.

---

## 1. Identidad y Rutas locales
- **Hero Principal:** `thesmoy`. Todas las métricas de winrate y filtros "Solo Hero" utilizan este identificador.
- **Ruta de Historiales:** `C:\Users\Miguel\AppData\Roaming\winamax\documents\accounts\thesmoy\history`.

---

## 2. Clasificación de Juego y Mesa
El parser debe identificar el formato en la primera línea de cada bloque de mano.

### 2.1 Identificación del Formato
- **Cash Game:** Detectar string `Winamax Poker - CashGame`.
- **Torneo/Expresso:** Detectar string `Winamax Poker - Tournament "[Name]"`.

### 2.2 Lógica de Posiciones (Especialización 6-max)
Independientemente del número de asientos (5-max o 6-max), el sistema utiliza una estructura interna de 6 posiciones. En mesas de 5 jugadores (comunes en Cash), se omite **EP (UTG)**.

| Jugadores | Asignación (desde el Botón en sentido horario) |
| :--- | :--- |
| **6-max** | 0:BTN, 1:SB, 2:BB, 3:UTG(EP), 4:MP, 5:CO |
| **5-max** | 0:BTN, 1:SB, 2:BB, **(EP Salto)**, 4:MP, 5:CO |

---

## 3. Parsing de Hand History (FSM Tokens)

### 3.1 Soporte Multi-idioma y Locales
El parser debe mapear los tokens de acción para los locales de Winamax (Inglés/Francés/Español):
- **Apostar:** `bets`, `mise`.
- **Igualar:** `calls`, `suit`.
- **Subir:** `raises [X] to [Y]`, `relance [X] à [Y]`. Siempre extraer **[Y]** como la apuesta total de la calle.
- **No Jugado:** `folds`, `passe`.
- **Ciegas/Ante:** `posts small blind`, `poste la petite blinde`.

### 3.2 Manejo de Divisas y Valores
- **Normalización:** Convertir importes (ej: `0.02€`, `1.50€`) a **BigInt en centavos** multiplicando por 100 para evitar errores de coma flotante.
- **Play Money:** Si el importe contiene `v` o `chips` en lugar de `€`, ignorar para estadísticas reales o marcar como `play_money = true`.

---

## 4. Casos Límite y Errores Típicos

### 4.1 Eventos de Mesa
- **Cambio de Mesa:** Detectar nueva cabecera `Table: '[Name]'` dentro del mismo archivo de torneo sin cerrar el anterior.
- **Disconnected/Sit-out:** Identificar líneas `[Player] posts blind [X] out of position` o jugadores que no reciben cartas (`Dealt to thesmoy` no aparece para ese asiento).
- **Side Pots:** Winamax detalla `collected [X] from main pot` y `collected [Y] from side pot 1`. El parser debe sumar ambos valores para validar el `Total pot` del `*** SUMMARY ***`.

### 4.2 Acciones de Torneo
- **Re-entries/Add-ons:** Detectar líneas de compra de fichas adicionales fuera del flujo normal de la mano para ajustar el cálculo del ROI en la tabla `tournaments`.
- **Evento de Eliminación:** Si el HH registra `thesmoy collected 0€ from pot` tras un All-in contra un oponente que se queda sin fichas, marcar mano con `bounty_pending = true`.