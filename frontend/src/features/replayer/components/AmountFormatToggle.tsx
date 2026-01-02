import React from 'react'
import type { AmountFormat } from '../../../hooks/useAmountFormat'

interface AmountFormatToggleProps {
  format: AmountFormat
  onToggle: () => void
  disabled?: boolean
}

/**
 * Componente toggle para alternar entre formato BB y EUR
 * Se integra en la barra de controles del replayer
 */
export const AmountFormatToggle: React.FC<AmountFormatToggleProps> = ({
  format,
  onToggle,
  disabled = false,
}) => {
  return (
    <button
      onClick={onToggle}
      disabled={disabled}
      className="px-3 py-2 rounded-lg bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm font-medium"
      title={`Formato actual: ${format.toUpperCase()}. Haz clic para cambiar a ${format === 'bb' ? 'EUR' : 'BB'}`}
    >
      {format === 'bb' ? 'BB' : '€'}
    </button>
  )
}

