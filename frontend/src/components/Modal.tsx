import React, { useEffect, useRef } from 'react';

export interface ModalProps {
  /**
   * Si true, el modal se muestra.
   */
  isOpen: boolean;
  
  /**
   * Función para cerrar el modal.
   */
  onClose: () => void;
  
  /**
   * Título del modal (opcional).
   */
  title?: string;
  
  /**
   * Contenido del modal.
   */
  children: React.ReactNode;
  
  /**
   * Botones del footer (opcional).
   */
  footer?: React.ReactNode;
  
  /**
   * Si true, cierra el modal al hacer click en el overlay.
   * @default true
   */
  closeOnOverlayClick?: boolean;
  
  /**
   * Si true, cierra el modal al presionar ESC.
   * @default true
   */
  closeOnEscape?: boolean;
  
  /**
   * Tamaño del modal.
   * @default 'md'
   */
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  
  /**
   * Clases CSS adicionales para el contenido.
   */
  className?: string;
}

/**
 * Componente Modal del sistema de diseño.
 * 
 * Modal/Dialog para mostrar contenido en una ventana emergente con overlay.
 * Incluye soporte para cierre por overlay/ESC y animaciones de entrada/salida.
 * 
 * @example
 * ```tsx
 * const [isOpen, setIsOpen] = useState(false);
 * 
 * <Modal
 *   isOpen={isOpen}
 *   onClose={() => setIsOpen(false)}
 *   title="Confirmar acción"
 *   footer={
 *     <>
 *       <Button variant="ghost" onClick={() => setIsOpen(false)}>
 *         Cancelar
 *       </Button>
 *       <Button variant="primary" onClick={handleConfirm}>
 *         Confirmar
 *       </Button>
 *     </>
 *   }
 * >
 *   <p>¿Estás seguro que deseas continuar?</p>
 * </Modal>
 * ```
 */
export const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  title,
  children,
  footer,
  closeOnOverlayClick = true,
  closeOnEscape = true,
  size = 'md',
  className = '',
}) => {
  const modalRef = useRef<HTMLDivElement>(null);

  // Manejar cierre con tecla ESC
  useEffect(() => {
    if (!isOpen || !closeOnEscape) return;

    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [isOpen, closeOnEscape, onClose]);

  // Bloquear scroll del body cuando el modal está abierto
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }

    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [isOpen]);

  // Focus trap: enfocar el modal al abrirse
  useEffect(() => {
    if (isOpen && modalRef.current) {
      modalRef.current.focus();
    }
  }, [isOpen]);

  // Manejar click en overlay
  const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (closeOnOverlayClick && e.target === e.currentTarget) {
      onClose();
    }
  };

  // No renderizar nada si está cerrado
  if (!isOpen) return null;

  // Clases de tamaño
  const sizeClasses: Record<typeof size, string> = {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl',
    full: 'max-w-full mx-4',
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm animate-in fade-in duration-200"
      onClick={handleOverlayClick}
      role="dialog"
      aria-modal="true"
      aria-labelledby={title ? 'modal-title' : undefined}
    >
      <div
        ref={modalRef}
        tabIndex={-1}
        className={`
          relative
          w-full
          ${sizeClasses[size]}
          bg-slate-800
          rounded-lg
          border
          border-slate-700
          shadow-2xl
          animate-in
          zoom-in-95
          duration-200
          focus:outline-none
          ${className}
        `}
      >
        {/* Header */}
        {title && (
          <div className="flex items-center justify-between px-6 py-4 border-b border-slate-700">
            <h2
              id="modal-title"
              className="text-xl font-semibold text-slate-200"
            >
              {title}
            </h2>
            <button
              onClick={onClose}
              className="
                p-1
                rounded-lg
                text-slate-400
                hover:text-slate-200
                hover:bg-slate-700
                transition-colors
                focus:outline-none
                focus:ring-2
                focus:ring-accent-violet
                focus:ring-offset-2
                focus:ring-offset-slate-800
              "
              aria-label="Cerrar modal"
            >
              <svg
                className="w-5 h-5"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fillRule="evenodd"
                  d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                  clipRule="evenodd"
                />
              </svg>
            </button>
          </div>
        )}

        {/* Body */}
        <div className="px-6 py-4 text-slate-200">
          {children}
        </div>

        {/* Footer */}
        {footer && (
          <div className="flex items-center justify-end gap-3 px-6 py-4 border-t border-slate-700 bg-slate-800/50">
            {footer}
          </div>
        )}
      </div>
    </div>
  );
};

Modal.displayName = 'Modal';

