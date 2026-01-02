import React from 'react';

export interface NavItem {
  /**
   * Identificador único del item.
   */
  id: string;
  
  /**
   * Texto del item.
   */
  label: string;
  
  /**
   * Ruta o acción al hacer click.
   */
  href?: string;
  
  /**
   * Icono opcional (JSX).
   */
  icon?: React.ReactNode;
  
  /**
   * Si true, el item está activo.
   */
  isActive?: boolean;
  
  /**
   * Función onClick personalizada.
   */
  onClick?: () => void;
}

export interface NavbarProps {
  /**
   * Logo o título de la aplicación.
   */
  logo?: React.ReactNode;
  
  /**
   * Items de navegación.
   */
  items?: NavItem[];
  
  /**
   * Contenido del área de usuario (avatar, menú, etc.).
   */
  userArea?: React.ReactNode;
  
  /**
   * Clases CSS adicionales.
   */
  className?: string;
}

/**
 * Componente Navbar del sistema de diseño.
 * 
 * Barra de navegación principal con logo, items de navegación y área de usuario.
 * Diseñado para modo oscuro con accent violet para items activos.
 * 
 * @example
 * ```tsx
 * <Navbar
 *   logo={<span className="font-bold text-xl">Poker AI</span>}
 *   items={[
 *     { id: '1', label: 'Dashboard', href: '/', isActive: true },
 *     { id: '2', label: 'Sesiones', href: '/sessions' },
 *     { id: '3', label: 'Análisis', href: '/analysis' },
 *   ]}
 *   userArea={
 *     <div className="flex items-center gap-2">
 *       <Badge variant="primary">thesmoy</Badge>
 *       <Button size="sm" variant="ghost">Salir</Button>
 *     </div>
 *   }
 * />
 * ```
 */
export const Navbar: React.FC<NavbarProps> = ({
  logo,
  items = [],
  userArea,
  className = '',
}) => {
  return (
    <nav
      className={`
        w-full
        bg-slate-800
        border-b
        border-slate-700
        ${className}
      `}
    >
      <div className="px-6 py-4">
        <div className="flex items-center justify-between">
          {/* Logo / Brand */}
          <div className="flex-shrink-0">
            {logo ? (
              <div className="text-slate-200">{logo}</div>
            ) : (
              <span className="text-xl font-bold text-accent-violet">
                Poker AI
              </span>
            )}
          </div>

          {/* Navigation Items */}
          {items.length > 0 && (
            <div className="hidden md:flex items-center gap-2 flex-1 justify-center">
              {items.map((item) => (
                <NavbarItem key={item.id} item={item} />
              ))}
            </div>
          )}

          {/* User Area */}
          {userArea && (
            <div className="flex-shrink-0 flex items-center gap-3">
              {userArea}
            </div>
          )}
        </div>

        {/* Mobile Navigation (si hay items) */}
        {items.length > 0 && (
          <div className="md:hidden mt-4 flex flex-col gap-1">
            {items.map((item) => (
              <NavbarItem key={item.id} item={item} />
            ))}
          </div>
        )}
      </div>
    </nav>
  );
};

/**
 * Componente interno para renderizar un item de navegación.
 */
const NavbarItem: React.FC<{ item: NavItem }> = ({ item }) => {
  const baseClasses = [
    'flex',
    'items-center',
    'gap-2',
    'px-4',
    'py-2',
    'rounded-lg',
    'text-sm',
    'font-medium',
    'transition-all',
    'duration-200',
    'focus:outline-none',
    'focus:ring-2',
    'focus:ring-accent-violet',
    'focus:ring-offset-2',
    'focus:ring-offset-slate-800',
  ];

  const stateClasses = item.isActive
    ? [
        'bg-accent-violet',
        'text-white',
      ]
    : [
        'text-slate-300',
        'hover:bg-slate-700',
        'hover:text-slate-200',
      ];

  const classes = [...baseClasses, ...stateClasses].filter(Boolean).join(' ');

  const handleClick = (e: React.MouseEvent) => {
    if (item.onClick) {
      e.preventDefault();
      item.onClick();
    }
  };

  const content = (
    <>
      {item.icon && <span className="flex-shrink-0">{item.icon}</span>}
      <span>{item.label}</span>
    </>
  );

  if (item.href) {
    return (
      <a href={item.href} className={classes} onClick={handleClick}>
        {content}
      </a>
    );
  }

  return (
    <button className={classes} onClick={handleClick}>
      {content}
    </button>
  );
};

Navbar.displayName = 'Navbar';

