/**
 * features/stats/components/RangeMatrix.tsx
 * Matriz 13x13 para visualización de rangos de starting hands
 */

import React, { useState, useMemo } from 'react'
import type { RangeData, MatrixCell, Hand, HandNotation } from '../../../types/ranges'
import { RANKS as RANK_LIST } from '../../../types/ranges'
import { getHeatmapColor, getTotalFrequency, getPrimaryAction, formatActionBreakdown } from '../utils/rangeUtils'

interface RangeMatrixProps {
  range?: RangeData
  onCellClick?: (hand: HandNotation) => void
  onSelectionChange?: (selectedHands: HandNotation[]) => void
  className?: string
}

/**
 * Genera la mano correspondiente a una posición en la matriz
 * - Diagonal (row === col): Pairs (AA, KK, QQ, ..., 22)
 * - Arriba diagonal (row < col): Suited (AKs, AQs, KQs, ...)
 * - Abajo diagonal (row > col): Offsuit (AKo, AQo, KQo, ...)
 */
function getHandFromPosition(row: number, col: number): Hand {
  const rank1 = RANK_LIST[row]
  const rank2 = RANK_LIST[col]

  if (row === col) {
    // Diagonal: Pairs
    return {
      notation: `${rank1}${rank2}`,
      type: 'pair',
      rank1,
      rank2
    }
  } else if (row < col) {
    // Arriba diagonal: Suited (rank1 > rank2)
    return {
      notation: `${rank1}${rank2}s`,
      type: 'suited',
      rank1,
      rank2
    }
  } else {
    // Abajo diagonal: Offsuit (rank1 < rank2, invertimos)
    return {
      notation: `${rank2}${rank1}o`,
      type: 'offsuit',
      rank1: rank2,
      rank2: rank1
    }
  }
}

/**
 * Genera todas las 169 celdas de la matriz
 */
function generateMatrixCells(range?: RangeData): MatrixCell[] {
  const cells: MatrixCell[] = []

  for (let row = 0; row < 13; row++) {
    for (let col = 0; col < 13; col++) {
      const hand = getHandFromPosition(row, col)
      const entries = range?.[hand.notation] || []

      cells.push({
        position: { row, col },
        hand,
        entries,
        isSelected: false
      })
    }
  }

  return cells
}

export function RangeMatrix({ range, onCellClick, onSelectionChange, className = '' }: RangeMatrixProps) {
  const [selectedCells, setSelectedCells] = useState<Set<string>>(new Set())
  const [isDragging, setIsDragging] = useState(false)
  const [dragStart, setDragStart] = useState<{ row: number; col: number } | null>(null)

  // Generar matriz de celdas
  const cells = useMemo(() => generateMatrixCells(range), [range])

  // Manejar click en celda
  const handleCellClick = (cell: MatrixCell) => {
    const key = `${cell.position.row}-${cell.position.col}`
    const newSelected = new Set(selectedCells)

    if (newSelected.has(key)) {
      newSelected.delete(key)
    } else {
      newSelected.add(key)
    }

    setSelectedCells(newSelected)
    onCellClick?.(cell.hand.notation)

    // Notificar cambio de selección
    const selectedHands = Array.from(newSelected).map(k => {
      const [row, col] = k.split('-').map(Number)
      return getHandFromPosition(row, col).notation
    })
    onSelectionChange?.(selectedHands)
  }

  // Manejar inicio de drag
  const handleMouseDown = (row: number, col: number) => {
    setIsDragging(true)
    setDragStart({ row, col })
  }

  // Manejar fin de drag
  const handleMouseUp = () => {
    setIsDragging(false)
    setDragStart(null)
  }

  // Manejar movimiento durante drag
  const handleMouseEnter = (row: number, col: number) => {
    if (!isDragging || !dragStart) return

    // Calcular área de selección rectangular
    const minRow = Math.min(dragStart.row, row)
    const maxRow = Math.max(dragStart.row, row)
    const minCol = Math.min(dragStart.col, col)
    const maxCol = Math.max(dragStart.col, col)

    const newSelected = new Set<string>()
    for (let r = minRow; r <= maxRow; r++) {
      for (let c = minCol; c <= maxCol; c++) {
        newSelected.add(`${r}-${c}`)
      }
    }

    setSelectedCells(newSelected)

    // Notificar cambio de selección
    const selectedHands = Array.from(newSelected).map(k => {
      const [r, c] = k.split('-').map(Number)
      return getHandFromPosition(r, c).notation
    })
    onSelectionChange?.(selectedHands)
  }

  return (
    <div className={`range-matrix-container ${className}`}>
      {/* Etiquetas superiores (columnas: A-2) */}
      <div className="flex">
        <div className="w-10 h-10" /> {/* Esquina vacía */}
        {RANK_LIST.map(rank => (
          <div
            key={`col-${rank}`}
            className="w-10 h-10 flex items-center justify-center text-slate-400 text-sm font-semibold"
          >
            {rank}
          </div>
        ))}
      </div>

      {/* Filas de la matriz */}
      {RANK_LIST.map((rank, rowIndex) => (
        <div key={`row-${rank}`} className="flex">
          {/* Etiqueta de fila */}
          <div className="w-10 h-10 flex items-center justify-center text-slate-400 text-sm font-semibold">
            {rank}
          </div>

          {/* Celdas de la fila */}
          {RANK_LIST.map((_, colIndex) => {
            const cellIndex = rowIndex * 13 + colIndex
            const cell = cells[cellIndex]
            const key = `${rowIndex}-${colIndex}`
            const isSelected = selectedCells.has(key)

            return (
              <RangeCell
                key={key}
                cell={cell}
                isSelected={isSelected}
                onClick={() => handleCellClick(cell)}
                onMouseDown={() => handleMouseDown(rowIndex, colIndex)}
                onMouseEnter={() => handleMouseEnter(rowIndex, colIndex)}
                onMouseUp={handleMouseUp}
              />
            )
          })}
        </div>
      ))}
    </div>
  )
}

// ============================================================================
// COMPONENTE AUXILIAR: RangeCell
// ============================================================================

interface RangeCellProps {
  cell: MatrixCell
  isSelected: boolean
  onClick: () => void
  onMouseDown: () => void
  onMouseEnter: () => void
  onMouseUp: () => void
}

function RangeCell({ cell, isSelected, onClick, onMouseDown, onMouseEnter, onMouseUp }: RangeCellProps) {
  const { hand, entries } = cell
  const [showTooltip, setShowTooltip] = useState(false)

  // Calcular frecuencia total y acción principal usando utilidades
  const totalFrequency = getTotalFrequency(entries)
  const primaryAction = getPrimaryAction(entries)

  // Calcular color de fondo basado en frecuencia
  const backgroundColor = primaryAction
    ? getHeatmapColor(primaryAction.action, totalFrequency)
    : 'transparent'

  // Clases CSS
  const cellClasses = [
    'w-10 h-10',
    'flex items-center justify-center',
    'text-xs font-medium',
    'border border-slate-700',
    'cursor-pointer',
    'transition-all duration-150',
    'select-none',
    'relative',
    isSelected ? 'ring-2 ring-violet-500 ring-inset' : '',
    'hover:ring-2 hover:ring-slate-500 hover:ring-inset'
  ].join(' ')

  // Texto de la celda con sufijo s/o
  const displayText = hand.type === 'pair'
    ? hand.notation
    : hand.type === 'suited'
      ? `${hand.rank1}${hand.rank2}s`
      : `${hand.rank1}${hand.rank2}o`

  // Tooltip content
  const tooltipContent = entries.length > 0
    ? formatActionBreakdown(entries)
    : 'Fold 100%'

  return (
    <div
      className={cellClasses}
      style={{ backgroundColor }}
      onClick={onClick}
      onMouseDown={onMouseDown}
      onMouseEnter={(e) => {
        onMouseEnter()
        setShowTooltip(true)
      }}
      onMouseLeave={() => setShowTooltip(false)}
      onMouseUp={onMouseUp}
    >
      <span className={totalFrequency > 0.5 ? 'text-white' : 'text-slate-300'}>
        {displayText}
      </span>

      {/* Tooltip */}
      {showTooltip && (
        <div className="absolute z-50 bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-slate-900 text-white text-xs rounded shadow-lg whitespace-pre-line pointer-events-none">
          <div className="font-semibold mb-1">{displayText}</div>
          <div className="text-slate-300">{tooltipContent}</div>
          {/* Flecha del tooltip */}
          <div className="absolute top-full left-1/2 transform -translate-x-1/2 border-4 border-transparent border-t-slate-900" />
        </div>
      )}
    </div>
  )
}


