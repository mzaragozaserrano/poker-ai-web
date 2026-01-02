# Sistema de Diseño - Componentes Base

Biblioteca de componentes reutilizables para Poker AI Web.

## Componentes Disponibles

### Button

Botón con múltiples variantes y tamaños. Incluye soporte para estados de carga y disabled.

**Variantes:**
- `primary` - Acento violeta para acciones principales (default)
- `secondary` - Slate para acciones secundarias
- `ghost` - Transparente con borde
- `destructive` - Rojo para acciones destructivas
- `raise` - Color poker para acciones de raise
- `call` - Color poker para acciones de call

**Ejemplo:**
```tsx
import { Button } from '@/components';

<Button variant="primary" size="md" onClick={handleClick}>
  Analizar Mano
</Button>

<Button variant="raise" isLoading>
  Procesando...
</Button>
```

---

### Input

Input de formulario con soporte para etiquetas, iconos y mensajes de ayuda.

**Variantes:**
- `default` - Estado normal (default)
- `error` - Para mostrar errores de validación
- `success` - Para confirmar entrada válida

**Ejemplo:**
```tsx
import { Input } from '@/components';

<Input
  label="Nombre de usuario"
  placeholder="thesmoy"
  helperText="Tu identificador en Winamax"
/>

<Input
  label="Email"
  type="email"
  variant="error"
  helperText="El email es inválido"
/>
```

---

### Badge

Badges para mostrar etiquetas, estados y acciones de poker.

**Variantes:**
- `default` - Gris neutro (default)
- `primary` - Violeta acento
- `success` - Verde
- `error` - Rojo
- `warning` - Amarillo
- `raise` - Color poker (rojo)
- `call` - Color poker (azul)
- `fold` - Color poker (gris con opacidad)
- `equity` - Color poker (verde)

**Ejemplo:**
```tsx
import { Badge } from '@/components';

<Badge variant="raise">RAISE</Badge>
<Badge variant="call">CALL</Badge>
<Badge variant="primary">Hero</Badge>
```

---

### Card

Contenedor base para agrupar información relacionada. Soporta header, body y footer opcionales.

**Props principales:**
- `header` - Contenido del encabezado
- `footer` - Contenido del pie
- `interactive` - Hace el card clickeable con efectos hover
- `padding` - Controla el padding del contenido (default: true)

**Ejemplo:**
```tsx
import { Card } from '@/components';

<Card
  header={<h3>Estadísticas de Sesión</h3>}
  footer={<Button variant="ghost">Ver detalles</Button>}
>
  <p>Manos jugadas: 245</p>
  <p>Beneficio: +15.5bb/100</p>
</Card>

<Card interactive onClick={handleClick}>
  <h4>Mano #12345</h4>
  <p>Click para ver detalles</p>
</Card>
```

---

### Modal

Modal/Dialog para mostrar contenido en una ventana emergente con overlay.

**Props principales:**
- `isOpen` - Controla la visibilidad
- `onClose` - Callback para cerrar
- `title` - Título del modal
- `footer` - Botones del footer
- `size` - Tamaño del modal (sm, md, lg, xl, full)
- `closeOnOverlayClick` - Cierra al hacer click en el overlay (default: true)
- `closeOnEscape` - Cierra al presionar ESC (default: true)

**Ejemplo:**
```tsx
import { Modal, Button } from '@/components';

const [isOpen, setIsOpen] = useState(false);

<Modal
  isOpen={isOpen}
  onClose={() => setIsOpen(false)}
  title="Confirmar acción"
  footer={
    <>
      <Button variant="ghost" onClick={() => setIsOpen(false)}>
        Cancelar
      </Button>
      <Button variant="primary" onClick={handleConfirm}>
        Confirmar
      </Button>
    </>
  }
>
  <p>¿Estás seguro que deseas continuar?</p>
</Modal>
```

---

### Navbar

Barra de navegación principal con logo, items de navegación y área de usuario.

**Ejemplo:**
```tsx
import { Navbar, Badge, Button } from '@/components';

<Navbar
  logo={<span className="font-bold text-xl">Poker AI</span>}
  items={[
    { id: '1', label: 'Dashboard', href: '/', isActive: true },
    { id: '2', label: 'Sesiones', href: '/sessions' },
    { id: '3', label: 'Análisis', href: '/analysis' },
  ]}
  userArea={
    <div className="flex items-center gap-2">
      <Badge variant="primary">thesmoy</Badge>
      <Button size="sm" variant="ghost">Salir</Button>
    </div>
  }
/>
```

---

## Paleta de Colores

Todos los componentes usan la paleta de colores definida en:
- **Documentación:** `docs/project/ui-foundations.md`
- **Configuración:** `frontend/tailwind.config.js`
- **Variables CSS:** `frontend/src/styles/variables.css`

### Colores Base
- `bg-slate-950` - Background principal (#0F172A)
- `bg-slate-800` - Surfaces/Cards (#1E293B)
- `border-slate-700` - Borders (#334155)

### Colores de Acento
- `bg-accent-violet` - Acciones primarias (#8B5CF6)

### Colores de Poker
- `bg-poker-raise` - Acciones agresivas (#EF4444)
- `bg-poker-call` - Acciones pasivas (#3B82F6)
- `bg-poker-fold` - Descarte (#64748B)
- `bg-poker-equity-high` - Equity alta (#10B981)

---

## Demo

Para ver todos los componentes en acción, la aplicación muestra automáticamente la página `ComponentShowcase` que contiene ejemplos de todos los componentes en sus diferentes variantes y estados.

---

## Accesibilidad

Todos los componentes incluyen:
- **Focus rings visibles** para navegación por teclado
- **ARIA labels** apropiados
- **Soporte para teclado** (Enter, Space, Escape)
- **Contraste adecuado** para modo oscuro

---

## TypeScript

Todos los componentes están completamente tipados con TypeScript. Exportan sus interfaces de props para facilitar el uso:

```tsx
import type { ButtonProps, ButtonVariant } from '@/components';
```

---

## Exportaciones

Todos los componentes se exportan desde `src/components/index.ts`:

```tsx
import {
  Badge,
  Button,
  Card,
  Input,
  Modal,
  Navbar,
  type BadgeProps,
  type ButtonProps,
  type CardProps,
  type InputProps,
  type ModalProps,
  type NavbarProps,
} from '@/components';
```

