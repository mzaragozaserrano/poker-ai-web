# CI/CD Workflows

Este directorio contiene los workflows de GitHub Actions para Continuous Integration y Continuous Deployment del proyecto Poker AI Web.

## 📋 Workflows Disponibles

### 1. Backend CI (`backend-ci.yml`)

**Trigger:**
- Pull Requests a `main` o `develop`
- Cambios en `backend/**`

**Jobs:**
- ✅ **Check Formatting**: Verifica formato con `cargo fmt`
- ✅ **Clippy Lints**: Linting estricto con `-D warnings`
- ✅ **Unit Tests**: Tests unitarios (excluye `poker-ffi`)
- ✅ **Build Check**: Compilación del workspace en debug mode

**Plataforma:** Ubuntu (Linux)

---

### 2. Frontend CI (`frontend-ci.yml`)

**Trigger:**
- Pull Requests a `main` o `develop`
- Cambios en `frontend/**`

**Jobs:**
- ✅ **Quality Checks**:
  - Type check (TypeScript)
  - Linting (ESLint)
  - Build (Vite)
  - Bundle size report

**Plataforma:** Ubuntu (Linux)

---

### 3. API CI (`api-ci.yml`)

**Trigger:**
- Pull Requests a `main` o `develop`
- Cambios en `server-api/**`

**Jobs:**
- ✅ **Quality Checks**:
  - Linting (Ruff)
  - Type checking (mypy)
  - Tests (pytest)
  - Coverage report

**Plataforma:** Ubuntu (Linux)

---

### 4. Release (`release.yml`)

**Trigger:**
- Push a `main` o `develop`
- Cambios en `backend/**`
- Manual (`workflow_dispatch`)

**Jobs:**
- ✅ **Test on Windows**: Tests completos en Windows
- ✅ **Build Release**: Genera binarios optimizados (solo en `main`)

**Plataforma:** Windows (compatible con Ryzen 7 3800X)

---

## 🚀 Cómo Funcionan

### Path Filtering

Cada CI solo se ejecuta cuando cambian archivos relevantes:

```yaml
paths:
  - 'frontend/**'
  - '.github/workflows/frontend-ci.yml'
```

Esto optimiza el uso de recursos y reduce tiempos de espera.

### Caching

Todos los workflows usan caching para acelerar builds:

- **Backend**: Cache de Cargo (registry, git, target)
- **Frontend**: Cache de npm (node_modules)
- **API**: Cache de Poetry (dependencias)

### Branch Protection

Para activar protección de branches en GitHub:

1. **Settings → Branches → Branch protection rules → main**
2. ✅ Require status checks to pass before merging
3. ✅ Require branches to be up to date before merging
4. ✅ Status checks requeridos:
   - `Check Formatting`
   - `Clippy Lints`
   - `Unit Tests`
   - `Build Check`
   - `Frontend Quality Checks`
   - `API Quality Checks`

---

## 📊 Estado de los CI

Puedes ver el estado de los CI en:
- **GitHub Actions**: https://github.com/mzaragozaserrano/poker-ai-web/actions
- **Pull Requests**: Cada PR muestra el estado de los checks

---

## 🔧 Configuración Local

### Backend (Rust)

```bash
cd backend
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --exclude poker-ffi --lib --bins
cargo build --workspace
```

### Frontend (React)

```bash
cd frontend
npm ci
npx tsc --noEmit
npm run lint
npm run build
```

### API (Python)

```bash
cd server-api
poetry install --with dev
poetry run ruff check app/
poetry run mypy app/ --ignore-missing-imports
poetry run pytest tests/ -v
```

---

## 🐛 Troubleshooting

### Backend CI falla en tests

- Verifica que los tests pasen localmente
- Revisa que `poker-ffi` esté correctamente excluido

### Frontend CI falla en type check

- Ejecuta `npx tsc --noEmit` localmente
- Verifica que todos los tipos estén definidos

### API CI falla en dependencias

- Verifica que `poetry.lock` esté actualizado
- Asegúrate de que las dev dependencies estén instaladas

---

## 📝 Notas

### poker-ffi en Tests

El crate `poker-ffi` requiere Python para linkear, por lo que se excluye de los tests en `backend-ci.yml`:

```bash
cargo test --workspace --exclude poker-ffi --lib --bins --verbose
```

En `release.yml` (Windows) sí se incluye, asumiendo que Python está disponible.

### Coverage en API

El workflow de API genera un reporte de coverage con `pytest-cov`. Para verlo localmente:

```bash
poetry run pytest tests/ --cov=app --cov-report=html
open htmlcov/index.html
```

---

## 🔄 Actualización de Workflows

Cuando modifiques un workflow:

1. Haz los cambios en el archivo `.yml`
2. Commit y push
3. El propio workflow se actualizará en el PR

Si cambias el path filter, asegúrate de incluir el propio workflow:

```yaml
paths:
  - 'frontend/**'
  - '.github/workflows/frontend-ci.yml'  # Importante!
```

---

## 📚 Referencias

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain)
- [actions/setup-node](https://github.com/actions/setup-node)
- [actions/setup-python](https://github.com/actions/setup-python)
- [actions/cache](https://github.com/actions/cache)

