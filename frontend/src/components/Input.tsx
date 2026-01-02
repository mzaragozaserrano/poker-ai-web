import React from 'react';

/**
 * Variantes visuales para el Input según su estado.
 */
export type InputVariant = 'default' | 'error' | 'success';

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  /**
   * Etiqueta descriptiva del input.
   */
  label?: string;
  
  /**
   * Mensaje de ayuda o error que se muestra debajo del input.
   */
  helperText?: string;
  
  /**
   * Variante visual del input.
   * @default 'default'
   */
  variant?: InputVariant;
  
  /**
   * Icono opcional que se muestra a la izquierda del input.
   */
  leftIcon?: React.ReactNode;
  
  /**
   * Icono opcional que se muestra a la derecha del input.
   */
  rightIcon?: React.ReactNode;
  
  /**
   * Ancho completo (100%).
   * @default false
   */
  fullWidth?: boolean;
}

/**
 * Componente Input del sistema de diseño.
 * 
 * Input de formulario con soporte para etiquetas, iconos, mensajes de ayuda
 * y estados visuales (error, success).
 * 
 * @example
 * ```tsx
 * <Input
 *   label="Nombre de usuario"
 *   placeholder="thesmoy"
 *   helperText="Tu identificador en Winamax"
 * />
 * 
 * <Input
 *   label="Email"
 *   type="email"
 *   variant="error"
 *   helperText="El email es inválido"
 * />
 * ```
 */
export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    {
      label,
      helperText,
      variant = 'default',
      leftIcon,
      rightIcon,
      fullWidth = false,
      disabled,
      className = '',
      id,
      ...props
    },
    ref
  ) => {
    // Generar ID único si no se proporciona
    const inputId = id || `input-${Math.random().toString(36).substring(2, 9)}`;

    // Clases base para el contenedor del input
    const containerClasses = [
      'relative',
      'flex',
      'items-center',
      fullWidth ? 'w-full' : '',
    ]
      .filter(Boolean)
      .join(' ');

    // Clases base para el input
    const baseInputClasses = [
      'block',
      'px-4',
      'py-2',
      'rounded-lg',
      'bg-slate-800',
      'text-slate-200',
      'placeholder:text-slate-500',
      'transition-all',
      'duration-200',
      'focus:outline-none',
      'focus:ring-2',
      'focus:ring-offset-2',
      'focus:ring-offset-slate-950',
      'disabled:opacity-50',
      'disabled:cursor-not-allowed',
      fullWidth ? 'w-full' : '',
    ];

    // Clases específicas por variante
    const variantClasses: Record<InputVariant, string[]> = {
      default: [
        'border',
        'border-slate-700',
        'focus:border-accent-violet',
        'focus:ring-accent-violet',
      ],
      error: [
        'border',
        'border-red-500',
        'focus:border-red-500',
        'focus:ring-red-500',
      ],
      success: [
        'border',
        'border-poker-equity-high',
        'focus:border-poker-equity-high',
        'focus:ring-poker-equity-high',
      ],
    };

    // Ajustar padding si hay iconos
    const paddingClasses = [];
    if (leftIcon) paddingClasses.push('pl-10');
    if (rightIcon) paddingClasses.push('pr-10');

    // Combinar todas las clases del input
    const inputClasses = [
      ...baseInputClasses,
      ...variantClasses[variant],
      ...paddingClasses,
      className,
    ]
      .filter(Boolean)
      .join(' ');

    // Clases para el helper text
    const helperTextClasses = [
      'mt-1.5',
      'text-sm',
      variant === 'error' ? 'text-red-400' : '',
      variant === 'success' ? 'text-poker-equity-high' : '',
      variant === 'default' ? 'text-slate-400' : '',
    ]
      .filter(Boolean)
      .join(' ');

    return (
      <div className={fullWidth ? 'w-full' : ''}>
        {/* Label */}
        {label && (
          <label
            htmlFor={inputId}
            className="block mb-2 text-sm font-medium text-slate-200"
          >
            {label}
          </label>
        )}

        {/* Input Container */}
        <div className={containerClasses}>
          {/* Left Icon */}
          {leftIcon && (
            <div className="absolute left-3 pointer-events-none text-slate-400">
              {leftIcon}
            </div>
          )}

          {/* Input */}
          <input
            ref={ref}
            id={inputId}
            disabled={disabled}
            className={inputClasses}
            {...props}
          />

          {/* Right Icon */}
          {rightIcon && (
            <div className="absolute right-3 pointer-events-none text-slate-400">
              {rightIcon}
            </div>
          )}
        </div>

        {/* Helper Text */}
        {helperText && (
          <p className={helperTextClasses}>
            {helperText}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

