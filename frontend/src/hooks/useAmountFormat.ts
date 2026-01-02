import { useState, useEffect } from 'react'

export type AmountFormat = 'bb' | 'eur'

const STORAGE_KEY = 'poker-amount-format'
const DEFAULT_FORMAT: AmountFormat = 'bb'

/**
 * Hook para gestionar el formato de visualización de cantidades (BB vs EUR)
 * Persiste la preferencia en localStorage
 */
export const useAmountFormat = () => {
  const [format, setFormatState] = useState<AmountFormat>(DEFAULT_FORMAT)
  const [isLoaded, setIsLoaded] = useState(false)

  // Cargar preferencia de localStorage al montar
  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored === 'bb' || stored === 'eur') {
      setFormatState(stored)
    }
    setIsLoaded(true)
  }, [])

  // Guardar preferencia en localStorage al cambiar
  const setFormat = (newFormat: AmountFormat) => {
    setFormatState(newFormat)
    localStorage.setItem(STORAGE_KEY, newFormat)
  }

  // Toggle entre formatos
  const toggleFormat = () => {
    const newFormat = format === 'bb' ? 'eur' : 'bb'
    setFormat(newFormat)
  }

  return {
    format,
    setFormat,
    toggleFormat,
    isLoaded,
  }
}

/**
 * Formatea una cantidad según el formato especificado
 * @param amount Cantidad en centavos (100 = 1 EUR)
 * @param format Formato: 'bb' o 'eur'
 * @param bigBlind Big blind en centavos (ej: 200 para 2 EUR)
 * @returns String formateado
 */
export const formatAmount = (
  amount: number,
  format: AmountFormat,
  bigBlind: number
): string => {
  if (format === 'eur') {
    const eur = amount / 100
    return `${eur.toFixed(2)}€`
  }

  // Formato BB
  const bb = amount / bigBlind
  // Mostrar decimales solo si hay parte decimal significativa
  if (Math.abs(bb - Math.round(bb)) < 0.01) {
    return `${Math.round(bb)}bb`
  }
  return `${bb.toFixed(1)}bb`
}

