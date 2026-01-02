import React from 'react';

/**
 * Variantes disponibles para el componente Button.
 * - primary: Acento violeta para acciones principales
 * - secondary: Slate para acciones secundarias
 * - ghost: Transparente con borde
 * - destructive: Rojo para acciones destructivas/danger
 * - raise: Color poker para acciones de raise
 * - call: Color poker para acciones de call
 */
export type ButtonVariant = 
  | 'primary' 
  | 'secondary' 
  | 'ghost' 
  | 'destructive'
  | 'raise'
  | 'call';

/**
 * Tamaños disponibles para el componente Button.
 */
export type ButtonSize = 'sm' | 'md' | 'lg';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  /**
   * Variante visual del botón.
   * @default 'primary'
   */
  variant?: ButtonVariant;
  
  /**
   * Tamaño del botón.
   * @default 'md'
   */
  size?: ButtonSize;
  
  /**
   * Si está en estado de carga, muestra un spinner.
   * @default false
   */
  isLoading?: boolean;
  
  /**
   * Ancho completo (100%).
   * @default false
   */
  fullWidth?: boolean;
  
  /**
   * Contenido del botón.
   */
  children: React.ReactNode;
}

/**
 * Componente Button del sistema de diseño.
 * 
 * Implementa las variantes de color definidas en la paleta Dark Mode
 * y sigue las especificaciones de accesibilidad (focus rings visibles).
 * 
 * @example
 * ```tsx
 * <Button variant="primary" size="md" onClick={handleClick}>
 *   Analizar Mano
 * </Button>
 * 
 * <Button variant="raise" size="sm" isLoading>
 *   Procesando...
 * </Button>
 * ```
 */
export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      variant = 'primary',
      size = 'md',
      isLoading = false,
      fullWidth = false,
      disabled,
      className = '',
      children,
      ...props
    },
    ref
  ) => {
    // Clases base comunes a todos los botones
    const baseClasses = [
      'inline-flex',
      'items-center',
      'justify-center',
      'font-medium',
      'rounded-lg',
      'transition-all',
      'duration-200',
      'focus:outline-none',
      'focus:ring-2',
      'focus:ring-offset-2',
      'focus:ring-offset-slate-950',
      'disabled:opacity-50',
      'disabled:cursor-not-allowed',
      'disabled:pointer-events-none',
    ];

    // Clases específicas por variante
    const variantClasses: Record<ButtonVariant, string[]> = {
      primary: [
        'bg-accent-violet',
        'text-white',
        'hover:opacity-90',
        'active:opacity-80',
        'focus:ring-accent-violet',
      ],
      secondary: [
        'bg-slate-800',
        'text-slate-200',
        'border',
        'border-slate-700',
        'hover:bg-slate-700',
        'active:bg-slate-600',
        'focus:ring-slate-600',
      ],
      ghost: [
        'bg-transparent',
        'text-slate-200',
        'border',
        'border-slate-700',
        'hover:bg-slate-800',
        'active:bg-slate-700',
        'focus:ring-slate-600',
      ],
      destructive: [
        'bg-red-600',
        'text-white',
        'hover:bg-red-700',
        'active:bg-red-800',
        'focus:ring-red-600',
      ],
      raise: [
        'bg-poker-raise',
        'text-white',
        'hover:opacity-90',
        'active:opacity-80',
        'focus:ring-poker-raise',
      ],
      call: [
        'bg-poker-call',
        'text-white',
        'hover:opacity-90',
        'active:opacity-80',
        'focus:ring-poker-call',
      ],
    };

    // Clases específicas por tamaño
    const sizeClasses: Record<ButtonSize, string[]> = {
      sm: ['px-3', 'py-1.5', 'text-sm', 'gap-1.5'],
      md: ['px-4', 'py-2', 'text-base', 'gap-2'],
      lg: ['px-6', 'py-3', 'text-lg', 'gap-2.5'],
    };

    // Clase de ancho completo
    const widthClass = fullWidth ? 'w-full' : '';

    // Combinar todas las clases
    const buttonClasses = [
      ...baseClasses,
      ...variantClasses[variant],
      ...sizeClasses[size],
      widthClass,
      className,
    ]
      .filter(Boolean)
      .join(' ');

    return (
      <button
        ref={ref}
        disabled={disabled || isLoading}
        className={buttonClasses}
        {...props}
      >
        {isLoading && (
          <svg
            className="animate-spin h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        )}
        {children}
      </button>
    );
  }
);

Button.displayName = 'Button';

