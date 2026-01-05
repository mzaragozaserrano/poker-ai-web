# Security Specification - Localhost-Only API

## Objetivo

Garantizar que la API REST y WebSocket del Poker AI Web solo escuchan en `127.0.0.1` (localhost) para proteger la privacidad de los datos del usuario. Los datos de poker nunca deben salir de la máquina local.

## Principios de Seguridad

### 1. Localhost-Only Binding

**Regla:** La API DEBE escuchar únicamente en `127.0.0.1` o `::1` (IPv6 localhost).

**Prohibido:**
- Binding a `0.0.0.0` (todas las interfaces)
- Binding a IPs externas (192.168.x.x, 10.x.x.x, etc.)
- Binding a IPs públicas

**Razón:** Los datos de poker son sensibles y privados. Exponerlos en la red local o internet violaría la privacidad del usuario.

### 2. Proxy Headers Blocking

**Regla:** La API DEBE rechazar requests con headers de proxy/forwarding.

**Headers Bloqueados:**
- `X-Forwarded-For`
- `X-Real-IP`
- `X-Forwarded-Host`
- `X-Forwarded-Proto`
- `X-Forwarded-Port`
- `Forwarded`

**Razón:** Estos headers podrían usarse para intentar bypass de la validación de IP localhost.

### 3. CORS Restrictivo

**Regla:** CORS solo permite orígenes localhost.

**Orígenes Permitidos:**
- `http://localhost:3000`
- `http://127.0.0.1:3000`
- `http://localhost:5173` (Vite dev server)
- `http://127.0.0.1:5173`

**Prohibido:**
- Wildcard `*`
- Orígenes externos
- Subdominios remotos

## Implementación

### Componentes de Seguridad

#### 1. Settings Configuration

Archivo: `server-api/app/config/settings.py`

```python
class Settings(BaseSettings):
    api_host: str = "127.0.0.1"  # SECURITY: Only localhost
```

**Validación:** El host por defecto es siempre `127.0.0.1`.

#### 2. LocalhostOnlyMiddleware

Archivo: `server-api/app/middleware/security.py`

**Funciones:**
- Valida que `request.client.host` es localhost
- Bloquea requests con proxy headers
- Añade security headers a responses
- Registra intentos de acceso no autorizados

**Respuesta a violaciones:**
- Status Code: `403 Forbidden`
- Logging: Warning con IP y path intentado

#### 3. Host Validation Function

Función: `validate_server_host(host: str)`

**Uso:** Llamada antes de iniciar Uvicorn para validar configuración.

**Comportamiento:**
- Si host es localhost → OK
- Si host NO es localhost → Raise `ValueError` con mensaje de error

#### 4. Startup Script Validation

Archivo: `server-api/run.ps1`

**Validación PowerShell:**
```powershell
$allowedHosts = @("127.0.0.1", "localhost", "::1")
if ($Host -notin $allowedHosts) {
    Write-Host "SECURITY ERROR: Invalid host '$Host'" -ForegroundColor Red
    exit 1
}
```

### Security Headers

Todas las responses incluyen:

| Header | Valor | Propósito |
|--------|-------|-----------|
| `X-Content-Type-Options` | `nosniff` | Prevenir MIME sniffing |
| `X-Frame-Options` | `DENY` | Prevenir clickjacking |
| `X-Localhost-Only` | `true` | Indicador de modo localhost |

## Testing

### Test Suite

Archivo: `server-api/tests/test_security.py`

**Tests Implementados:**

1. **test_localhost_connection_allowed**
   - Verifica que conexiones desde localhost son aceptadas
   - Status: 200 OK

2. **test_proxy_headers_blocked**
   - Verifica que headers de proxy son bloqueados
   - Status: 403 Forbidden

3. **test_security_headers_added**
   - Verifica que security headers están presentes

4. **test_validate_server_host**
   - Valida que `127.0.0.1`, `localhost`, `::1` son aceptados
   - Valida que `0.0.0.0` y IPs externas son rechazadas

### Ejecutar Tests

```bash
cd server-api
poetry run pytest tests/test_security.py -v
```

## Verificación Manual

### 1. Verificar Binding del Servidor

**Windows (PowerShell):**
```powershell
netstat -an | Select-String "8000"
```

**Linux/macOS:**
```bash
ss -tlnp | grep 8000
# o
netstat -tlnp | grep 8000
```

**Resultado Esperado:**
```
127.0.0.1:8000    LISTEN
```

**Resultado NO Válido:**
```
0.0.0.0:8000      LISTEN    # ❌ Expuesto a toda la red
192.168.1.x:8000  LISTEN    # ❌ Expuesto a IP externa
```

### 2. Test de Conexión Externa

**Desde otra máquina en la red:**
```bash
curl http://192.168.1.x:8000/health
```

**Resultado Esperado:** Connection refused o timeout (el servidor no debería ser accesible).

### 3. Test de Proxy Headers

```bash
curl http://127.0.0.1:8000/health \
  -H "X-Forwarded-For: 192.168.1.100"
```

**Resultado Esperado:**
- Status: 403 Forbidden
- Body: "Forbidden: Proxy headers are not allowed"

### 4. Test de CORS

```bash
curl http://127.0.0.1:8000/health \
  -H "Origin: http://malicious-site.com"
```

**Resultado Esperado:** CORS headers no permiten el origen.

## Checklist de Seguridad

### Pre-Deployment

- [ ] Verificar que `settings.api_host` es `127.0.0.1`
- [ ] Confirmar que `LocalhostOnlyMiddleware` está activo
- [ ] Ejecutar test suite de seguridad
- [ ] Verificar CORS origins (solo localhost)
- [ ] Confirmar que no hay `0.0.0.0` en el código

### Post-Deployment

- [ ] Ejecutar `netstat`/`ss` para verificar binding
- [ ] Intentar conexión desde IP externa (debe fallar)
- [ ] Verificar logs de seguridad
- [ ] Test de proxy headers
- [ ] Verificar security headers en responses

### Monitoreo Continuo

- [ ] Revisar logs de intentos de acceso bloqueados
- [ ] Auditar cambios en configuración de host
- [ ] Validar que scripts de inicio no permiten override a `0.0.0.0`

## Configuración de Producción

### Variables de Entorno

**Archivo:** `server-api/.env`

```env
# SECURITY: Never change this to 0.0.0.0
API_HOST=127.0.0.1
API_PORT=8000
DEBUG=false
```

### Uvicorn Configuration

**Startup Command:**
```bash
uvicorn app.main:app \
  --host 127.0.0.1 \
  --port 8000 \
  --no-access-log
```

**Opciones Prohibidas:**
```bash
# ❌ NUNCA usar:
--host 0.0.0.0
--host 192.168.1.x
```

## Respuesta a Incidentes

### Si se detecta binding a 0.0.0.0

1. **Detener el servidor inmediatamente**
2. Verificar configuración en `settings.py`
3. Verificar variables de entorno
4. Revisar scripts de inicio
5. Ejecutar test suite de seguridad
6. Reiniciar con configuración correcta

### Si se detectan intentos de acceso externo

1. Revisar logs: `server-api/logs/security.log`
2. Identificar IP origen
3. Verificar que middleware está activo
4. Confirmar que firewall local está configurado
5. Documentar el incidente

## Referencias

### Archivos Relacionados

- `server-api/app/config/settings.py` - Configuración
- `server-api/app/middleware/security.py` - Middleware de seguridad
- `server-api/app/main.py` - Aplicación principal
- `server-api/tests/test_security.py` - Tests de seguridad
- `server-api/run.ps1` - Script de inicio

### Documentación Externa

- [FastAPI Security](https://fastapi.tiangolo.com/tutorial/security/)
- [OWASP API Security](https://owasp.org/www-project-api-security/)
- [Uvicorn Deployment](https://www.uvicorn.org/deployment/)

## Notas Adicionales

### ¿Por qué no usar autenticación en lugar de localhost-only?

**Respuesta:** La autenticación protege contra usuarios no autorizados, pero no previene:
- Sniffing de red local
- Ataques MITM en la red local
- Exposición accidental de datos

**Localhost-only** es una capa adicional que garantiza que los datos NUNCA salen de la máquina, independientemente de la autenticación.

### ¿Qué pasa si necesito acceso remoto?

**Respuesta:** Si necesitas acceso remoto:
1. Usa SSH tunneling: `ssh -L 8000:localhost:8000 user@remote-machine`
2. Usa VPN para acceder a la máquina remota
3. **NUNCA** expongas la API directamente a la red

### ¿Cómo afecta esto al desarrollo?

**Respuesta:** No afecta. El frontend (React) también corre en localhost, por lo que puede comunicarse con la API sin problemas.

---

**Última actualización:** 2026-01-05  
**Versión:** 1.0  
**Responsable:** Security Team

