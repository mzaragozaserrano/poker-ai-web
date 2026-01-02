import React from 'react';

/**
 * Variantes disponibles para el componente Badge.
 * Incluye variantes específicas de poker para acciones.
 */
export type BadgeVariant = 
  | 'default' 
  | 'primary'
  | 'success' 
  | 'error'
  | 'warning'
  | 'raise'
  | 'call'
  | 'fold'
  | 'equity';

/**
 * Tamaños disponibles para el Badge.
 */
export type BadgeSize = 'sm' | 'md' | 'lg';

export interface BadgeProps {
  /**
   * Variante visual del badge.
   * @default 'default'
   */
  variant?: BadgeVariant;
  
  /**
   * Tamaño del badge.
   * @default 'md'
   */
  size?: BadgeSize;
  
  /**
   * Contenido del badge.
   */
  children: React.ReactNode;
  
  /**
   * Icono opcional que se muestra a la izquierda.
   */
  icon?: React.ReactNode;
  
  /**
   * Clases CSS adicionales.
   */
  className?: string;
}

/**
 * Componente Badge del sistema de diseño.
 * 
 * Badges para mostrar etiquetas, estados y acciones de poker.
 * Incluye variantes específicas del dominio (RAISE, CALL, FOLD, EQUITY).
 * 
 * @example
 * ```tsx
 * <Badge variant="raise">RAISE</Badge>
 * <Badge variant="call">CALL</Badge>
 * <Badge variant="fold">FOLD</Badge>
 * <Badge variant="equity">HIGH EQUITY</Badge>
 * <Badge variant="primary">Hero</Badge>
 * ```
 */
export const Badge: React.FC<BadgeProps> = ({
  variant = 'default',
  size = 'md',
  children,
  icon,
  className = '',
}) => {
  // Clases base comunes a todos los badges
  const baseClasses = [
    'inline-flex',
    'items-center',
    'font-semibold',
    'rounded-full',
    'whitespace-nowrap',
  ];

  // Clases específicas por variante
  const variantClasses: Record<BadgeVariant, string[]> = {
    default: [
      'bg-slate-700',
      'text-slate-200',
    ],
    primary: [
      'bg-accent-violet',
      'text-white',
    ],
    success: [
      'bg-poker-equity-high',
      'text-white',
    ],
    error: [
      'bg-red-600',
      'text-white',
    ],
    warning: [
      'bg-yellow-500',
      'text-slate-950',
    ],
    raise: [
      'bg-poker-raise',
      'text-white',
    ],
    call: [
      'bg-poker-call',
      'text-white',
    ],
    fold: [
      'bg-poker-fold',
      'text-white',
      'opacity-70',
    ],
    equity: [
      'bg-poker-equity-high',
      'text-white',
    ],
  };

  // Clases específicas por tamaño
  const sizeClasses: Record<BadgeSize, string[]> = {
    sm: ['px-2', 'py-0.5', 'text-xs', 'gap-1'],
    md: ['px-2.5', 'py-1', 'text-sm', 'gap-1.5'],
    lg: ['px-3', 'py-1.5', 'text-base', 'gap-2'],
  };

  // Combinar todas las clases
  const badgeClasses = [
    ...baseClasses,
    ...variantClasses[variant],
    ...sizeClasses[size],
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <span className={badgeClasses}>
      {icon && <span className="flex-shrink-0">{icon}</span>}
      {children}
    </span>
  );
};

Badge.displayName = 'Badge';

