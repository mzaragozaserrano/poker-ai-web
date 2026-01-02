import React from 'react';

export interface CardProps {
  /**
   * Contenido del header (opcional).
   */
  header?: React.ReactNode;
  
  /**
   * Contenido principal del card.
   */
  children: React.ReactNode;
  
  /**
   * Contenido del footer (opcional).
   */
  footer?: React.ReactNode;
  
  /**
   * Si true, agrega padding al contenido.
   * @default true
   */
  padding?: boolean;
  
  /**
   * Si true, hace el card clickeable con efectos hover.
   * @default false
   */
  interactive?: boolean;
  
  /**
   * Función onClick si el card es interactive.
   */
  onClick?: () => void;
  
  /**
   * Clases CSS adicionales.
   */
  className?: string;
}

/**
 * Componente Card del sistema de diseño.
 * 
 * Contenedor base para agrupar información relacionada.
 * Soporta header, body y footer opcionales.
 * 
 * @example
 * ```tsx
 * <Card
 *   header={<h3>Estadísticas de Sesión</h3>}
 *   footer={<Button variant="ghost">Ver detalles</Button>}
 * >
 *   <p>Manos jugadas: 245</p>
 *   <p>Beneficio: +15.5bb/100</p>
 * </Card>
 * 
 * <Card interactive onClick={handleClick}>
 *   <h4>Mano #12345</h4>
 *   <p>Pot: €5.50</p>
 * </Card>
 * ```
 */
export const Card: React.FC<CardProps> = ({
  header,
  children,
  footer,
  padding = true,
  interactive = false,
  onClick,
  className = '',
}) => {
  // Clases base del card
  const baseClasses = [
    'bg-slate-800',
    'rounded-lg',
    'border',
    'border-slate-700',
    'overflow-hidden',
  ];

  // Clases interactivas
  const interactiveClasses = interactive
    ? [
        'cursor-pointer',
        'transition-all',
        'duration-200',
        'hover:border-accent-violet',
        'hover:shadow-lg',
        'hover:shadow-accent-violet/10',
        'active:scale-[0.98]',
      ]
    : [];

  // Combinar clases
  const cardClasses = [...baseClasses, ...interactiveClasses, className]
    .filter(Boolean)
    .join(' ');

  // Clases de padding
  const paddingClasses = padding ? 'p-6' : '';

  return (
    <div
      className={cardClasses}
      onClick={interactive ? onClick : undefined}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}
      onKeyDown={
        interactive
          ? (e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                onClick?.();
              }
            }
          : undefined
      }
    >
      {/* Header */}
      {header && (
        <div className="px-6 py-4 border-b border-slate-700">
          {header}
        </div>
      )}

      {/* Body */}
      <div className={paddingClasses}>
        {children}
      </div>

      {/* Footer */}
      {footer && (
        <div className="px-6 py-4 border-t border-slate-700 bg-slate-800/50">
          {footer}
        </div>
      )}
    </div>
  );
};

Card.displayName = 'Card';

