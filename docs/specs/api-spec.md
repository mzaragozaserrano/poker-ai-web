# API Specification: FastAPI REST

Este documento define los contratos de comunicación externa de la plataforma, asegurando la consistencia entre el motor de Rust, DuckDB y el frontend en React.

---

## 1. Globales y Versionado

- **Base URL:** `http://127.0.0.1:8000/api/v1`
- **Protocolo:** REST para consulta de datos históricos y analíticos.
- **Seguridad:** Acceso restringido exclusivamente a `localhost` para garantizar la soberanía de los datos.

---

## 2. Endpoints REST

### `GET /stats/{player_name}`
Obtiene las métricas agregadas de un jugador basándose en el historial almacenado en DuckDB.

**Query Parameters:**
- `start_date` (string, ISO-8601): Fecha inicio del filtrado.
- `end_date` (string, ISO-8601): Fecha fin del filtrado.
- `stake` (string): Nivel de ciegas (ej: 'NL2').
- `game_type` (enum): 'NLHE' o 'PLO'.
- `min_hands` (int): Tamaño de muestra mínimo para mostrar la estadística (default: 1).

**JSON Response Schema:**
```json
{
  "player": "thesmoy",
  "is_hero": true,
  "summary": {
    "hands": 1540,
    "vpip": 24.5,
    "pfr": 20.1,
    "three_bet": 8.2,
    "wtsd": 28.4,
    "af": 2.5,
    "net_won_cents": 4500,
    "bb_100": 15.2,
    "ev_bb_100": 12.8
  },
  "positional": {
    "BTN": { "vpip": 45.0, "pfr": 38.0, "hands": 250 },
    "SB": { "vpip": 32.0, "pfr": 28.0, "hands": 250 },
    "BB": { "vpip": 12.0, "pfr": 0.0, "hands": 250 },
    "MP": { "vpip": 18.0, "pfr": 15.0, "hands": 250 },
    "CO": { "vpip": 26.0, "pfr": 22.0, "hands": 250 }
  }
}
```

**Nota:** Se omite UTG (EP) siguiendo la especialización de 6-max para mesas de 5 jugadores.

---

## 3. Manejo de Errores

La API utiliza códigos de estado HTTP estándar acompañados de un cuerpo JSON estructurado.

| Código | Condición | Descripción |
|--------|-----------|-------------|
| 200 | OK | Solicitud exitosa. |
| 400 | Bad Request | Parámetros de filtrado inválidos (ej: fecha mal formada). |
| 404 | Not Found | El jugador solicitado no existe en la base de datos DuckDB. |
| 500 | Internal Error | Error crítico en el núcleo de Rust o en la query de DuckDB. |

**Cuerpo de Error:**

```json
{
  "error": {
    "code": "PLAYER_NOT_FOUND",
    "message": "El nickname 'thesmoy' no tiene manos registradas.",
    "trace_id": "uuid-v4-generated"
  }
}
```