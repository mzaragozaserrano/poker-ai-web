# Optimización del CI/CD Pipeline

## Problema Original
El workflow único `ci-rust.yml` tardaba ~20 minutos por PR debido a:
- Jobs secuenciales (test → build-release)
- Duplicación de setup (Rust + cache en cada job)
- Build release innecesario en PRs
- Ejecución secuencial de fmt/clippy/test
- Windows runner (más lento para checks genéricos)

## Solución Implementada

### 1. Separación de Workflows

#### `fast-checks.yml` (Pull Requests)
**Objetivo:** Feedback rápido < 5 minutos  
**Estrategia:** Paralelización máxima + Linux runners

**Jobs paralelos:**
- `fmt`: Validación de formato (< 30s)
- `clippy`: Lints estáticos (2-3 min)
- `test`: Tests unitarios (2-3 min)
- `build`: Verificación de compilación debug (2-3 min)

**Optimizaciones:**
- Usa `ubuntu-latest` (más rápido y barato)
- Cache independiente por job (key específica)
- Solo compila en modo debug
- Tests unitarios (`--lib --bins`) sin integración

#### `full-ci.yml` (Main/Develop)
**Objetivo:** Validación completa en plataforma target  
**Estrategia:** Exhaustividad + artifacts

**Jobs:**
- `test-windows`: Suite completa en Windows (target real)
- `build-release`: Build optimizado solo en main

**Características:**
- Windows runner para validar compatibilidad
- Tests con `--all-features`
- Build release solo en merge a main
- Artifacts subidos para distribución

### 2. Mejoras de Cache

**Antes:**
```yaml
key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

**Ahora:**
```yaml
key: ${{ runner.os }}-cargo-<job>-${{ hashFiles('**/Cargo.lock') }}
```

**Beneficios:**
- Cache específica por job (evita invalidaciones innecesarias)
- Restore-keys con fallback jerárquico
- Reutilización entre jobs del mismo tipo

### 3. Paralelización

**Antes (secuencial):**
```
fmt → clippy → test → build → build-release
Total: ~20 min
```

**Ahora (paralelo en PR):**
```
fmt (30s)
clippy (2-3 min)  ┐
test (2-3 min)    ├─ En paralelo
build (2-3 min)   ┘
Total: ~3-4 min
```

### 4. Optimización de Runners

| Workflow | Runner | Justificación |
|----------|--------|---------------|
| fast-checks | ubuntu-latest | Rust es cross-platform, Linux es 2-3x más rápido |
| full-ci | windows-latest | Validación en plataforma target (Ryzen 7 3800X) |

## Tiempos Esperados

### Pull Request (fast-checks.yml)
- Formato: 20-30s
- Clippy: 2-3 min (con cache)
- Tests: 2-3 min (con cache)
- Build: 2-3 min (con cache)
- **Total: 3-4 minutos** (vs 20 min anterior)

### Push a Main (full-ci.yml)
- Tests Windows: 8-10 min
- Build Release: 5-7 min
- **Total: 13-17 minutos** (solo en merge)

## Próximas Optimizaciones (Opcionales)

### 1. sccache (Compilación distribuida)
```yaml
- name: Install sccache
  run: cargo install sccache --locked
- name: Configure sccache
  run: echo "RUSTC_WRAPPER=sccache" >> $GITHUB_ENV
```
**Impacto:** Reduce compilaciones repetidas en 40-60%

### 2. Self-Hosted Runner
**Configuración:** Runner en tu Ryzen 7 3800X  
**Beneficios:**
- Cache persistente entre runs
- 16 threads disponibles (vs 2-4 en GitHub)
- Latencia cero para acceso a disco
- **Estimado:** < 2 min para fast-checks

### 3. Cargo nextest
```yaml
- name: Install nextest
  run: cargo install cargo-nextest
- name: Run tests
  run: cargo nextest run --workspace
```
**Beneficio:** Ejecución paralela de tests (20-30% más rápido)

### 4. Selective Testing
```yaml
paths-ignore:
  - 'docs/**'
  - '**.md'
  - 'frontend/**'
```
**Beneficio:** Evita CI innecesario en cambios de documentación

## Monitoreo

Para verificar mejoras:
1. GitHub Actions UI → Workflow runs
2. Comparar tiempos antes/después
3. Identificar jobs lentos en la vista de timeline

## Referencias
- [GitHub Actions: Caching dependencies](https://docs.github.com/en/actions/using-workflows/caching-dependencies-to-speed-up-workflows)
- [Rust CI Best Practices](https://matklad.github.io/2021/09/04/fast-rust-builds.html)
- [cargo-nextest](https://nexte.st/)

