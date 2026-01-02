/**
 * features/stats/components/RangePresets.tsx
 * Presets de rangos GTO para 6-max Cash Games
 */

import type { RangePreset, RangeData } from '../../../types/ranges'

interface RangePresetsProps {
  onPresetSelect: (preset: RangePreset) => void
  selectedPresetId?: string
  className?: string
}

export function RangePresets({ onPresetSelect, selectedPresetId, className = '' }: RangePresetsProps) {
  return (
    <div className={`range-presets ${className}`}>
      <h3 className="text-lg font-semibold text-white mb-4">Rangos Predefinidos</h3>

      {/* RFI (Raise First In) */}
      <div className="mb-6">
        <h4 className="text-sm font-medium text-slate-400 mb-2">RFI - Raise First In</h4>
        <div className="space-y-2">
          {RFI_PRESETS.map(preset => (
            <PresetButton
              key={preset.id}
              preset={preset}
              isSelected={selectedPresetId === preset.id}
              onClick={() => onPresetSelect(preset)}
            />
          ))}
        </div>
      </div>

      {/* 3Bet */}
      <div className="mb-6">
        <h4 className="text-sm font-medium text-slate-400 mb-2">3Bet</h4>
        <div className="space-y-2">
          {THREEBET_PRESETS.map(preset => (
            <PresetButton
              key={preset.id}
              preset={preset}
              isSelected={selectedPresetId === preset.id}
              onClick={() => onPresetSelect(preset)}
            />
          ))}
        </div>
      </div>

      {/* Blind Defense */}
      <div className="mb-6">
        <h4 className="text-sm font-medium text-slate-400 mb-2">Defensa de Ciegas</h4>
        <div className="space-y-2">
          {BLIND_DEFENSE_PRESETS.map(preset => (
            <PresetButton
              key={preset.id}
              preset={preset}
              isSelected={selectedPresetId === preset.id}
              onClick={() => onPresetSelect(preset)}
            />
          ))}
        </div>
      </div>
    </div>
  )
}

// ============================================================================
// COMPONENTE AUXILIAR: PresetButton
// ============================================================================

interface PresetButtonProps {
  preset: RangePreset
  isSelected: boolean
  onClick: () => void
}

function PresetButton({ preset, isSelected, onClick }: PresetButtonProps) {
  return (
    <button
      onClick={onClick}
      className={[
        'w-full text-left px-3 py-2 rounded-lg text-sm transition-colors',
        isSelected
          ? 'bg-violet-600 text-white'
          : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
      ].join(' ')}
    >
      <div className="font-medium">{preset.name}</div>
      <div className="text-xs opacity-75">{preset.description}</div>
    </button>
  )
}

// ============================================================================
// PRESETS DE RANGOS GTO 6-MAX
// ============================================================================

/**
 * RFI (Raise First In) Presets
 * Rangos de apertura según posición
 */
const RFI_PRESETS: RangePreset[] = [
  {
    id: 'utg_open',
    name: 'UTG Open',
    description: 'Early Position Open Raise',
    position: 'UTG',
    range: createRangeFromNotation({
      // Premium pairs
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1,
      // Medium pairs
      '99': 1, '88': 1, '77': 1, '66': 1,
      // Premium suited
      'AKs': 1, 'AQs': 1, 'AJs': 1, 'ATs': 1,
      'KQs': 1, 'KJs': 1, 'KTs': 1,
      'QJs': 1, 'QTs': 1, 'JTs': 1,
      // Suited connectors
      'T9s': 1, '98s': 1, '87s': 1, '76s': 1,
      // Premium offsuit
      'AKo': 1, 'AQo': 1, 'AJo': 1,
      'KQo': 1
    })
  },
  {
    id: 'mp_open',
    name: 'MP Open',
    description: 'Middle Position Open Raise',
    position: 'MP',
    range: createRangeFromNotation({
      // All UTG hands
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1,
      '99': 1, '88': 1, '77': 1, '66': 1, '55': 1,
      'AKs': 1, 'AQs': 1, 'AJs': 1, 'ATs': 1, 'A9s': 0.5,
      'KQs': 1, 'KJs': 1, 'KTs': 1, 'K9s': 0.5,
      'QJs': 1, 'QTs': 1, 'Q9s': 0.5, 'JTs': 1, 'J9s': 0.5,
      'T9s': 1, '98s': 1, '87s': 1, '76s': 1, '65s': 1,
      'AKo': 1, 'AQo': 1, 'AJo': 1, 'ATo': 1,
      'KQo': 1, 'KJo': 1
    })
  },
  {
    id: 'co_open',
    name: 'CO Open',
    description: 'Cutoff Open Raise',
    position: 'CO',
    range: createRangeFromNotation({
      // Pairs
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1,
      '99': 1, '88': 1, '77': 1, '66': 1, '55': 1, '44': 1, '33': 1, '22': 1,
      // Suited aces
      'AKs': 1, 'AQs': 1, 'AJs': 1, 'ATs': 1, 'A9s': 1, 'A8s': 1, 'A7s': 1, 'A6s': 1, 'A5s': 1, 'A4s': 1, 'A3s': 1, 'A2s': 1,
      // Suited kings
      'KQs': 1, 'KJs': 1, 'KTs': 1, 'K9s': 1, 'K8s': 0.5, 'K7s': 0.5,
      // Suited queens
      'QJs': 1, 'QTs': 1, 'Q9s': 1, 'Q8s': 0.5,
      // Suited jacks
      'JTs': 1, 'J9s': 1, 'J8s': 0.5,
      // Suited connectors
      'T9s': 1, 'T8s': 1, '98s': 1, '97s': 1, '87s': 1, '86s': 1, '76s': 1, '75s': 1, '65s': 1, '54s': 1,
      // Offsuit broadway
      'AKo': 1, 'AQo': 1, 'AJo': 1, 'ATo': 1, 'A9o': 0.5,
      'KQo': 1, 'KJo': 1, 'KTo': 1,
      'QJo': 1, 'QTo': 1,
      'JTo': 1
    })
  },
  {
    id: 'btn_open',
    name: 'BTN Open',
    description: 'Button Open Raise',
    position: 'BTN',
    range: createRangeFromNotation({
      // Casi todas las parejas
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1,
      '99': 1, '88': 1, '77': 1, '66': 1, '55': 1, '44': 1, '33': 1, '22': 1,
      // Suited aces (todos)
      'AKs': 1, 'AQs': 1, 'AJs': 1, 'ATs': 1, 'A9s': 1, 'A8s': 1, 'A7s': 1, 'A6s': 1, 'A5s': 1, 'A4s': 1, 'A3s': 1, 'A2s': 1,
      // Suited kings
      'KQs': 1, 'KJs': 1, 'KTs': 1, 'K9s': 1, 'K8s': 1, 'K7s': 1, 'K6s': 1, 'K5s': 1, 'K4s': 1, 'K3s': 1, 'K2s': 1,
      // Suited queens
      'QJs': 1, 'QTs': 1, 'Q9s': 1, 'Q8s': 1, 'Q7s': 1, 'Q6s': 1, 'Q5s': 1, 'Q4s': 1,
      // Suited jacks
      'JTs': 1, 'J9s': 1, 'J8s': 1, 'J7s': 1, 'J6s': 1,
      // Suited tens
      'T9s': 1, 'T8s': 1, 'T7s': 1, 'T6s': 1,
      // Suited connectors y gappers
      '98s': 1, '97s': 1, '96s': 1, '87s': 1, '86s': 1, '85s': 1, '76s': 1, '75s': 1, '65s': 1, '64s': 1, '54s': 1, '53s': 1,
      // Offsuit broadway y aces
      'AKo': 1, 'AQo': 1, 'AJo': 1, 'ATo': 1, 'A9o': 1, 'A8o': 1, 'A7o': 1, 'A6o': 1, 'A5o': 1,
      'KQo': 1, 'KJo': 1, 'KTo': 1, 'K9o': 1,
      'QJo': 1, 'QTo': 1, 'Q9o': 1,
      'JTo': 1, 'J9o': 1,
      'T9o': 1, 'T8o': 1
    })
  },
  {
    id: 'sb_open',
    name: 'SB Open',
    description: 'Small Blind Open Raise',
    position: 'SB',
    range: createRangeFromNotation({
      // Similar a BTN pero ligeramente más tight
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1,
      '99': 1, '88': 1, '77': 1, '66': 1, '55': 1, '44': 1, '33': 1, '22': 1,
      'AKs': 1, 'AQs': 1, 'AJs': 1, 'ATs': 1, 'A9s': 1, 'A8s': 1, 'A7s': 1, 'A6s': 1, 'A5s': 1, 'A4s': 1, 'A3s': 1, 'A2s': 1,
      'KQs': 1, 'KJs': 1, 'KTs': 1, 'K9s': 1, 'K8s': 1, 'K7s': 1, 'K6s': 1, 'K5s': 1, 'K4s': 1,
      'QJs': 1, 'QTs': 1, 'Q9s': 1, 'Q8s': 1, 'Q7s': 1, 'Q6s': 1,
      'JTs': 1, 'J9s': 1, 'J8s': 1, 'J7s': 1,
      'T9s': 1, 'T8s': 1, 'T7s': 1,
      '98s': 1, '97s': 1, '87s': 1, '86s': 1, '76s': 1, '75s': 1, '65s': 1, '54s': 1,
      'AKo': 1, 'AQo': 1, 'AJo': 1, 'ATo': 1, 'A9o': 1, 'A8o': 1, 'A7o': 1,
      'KQo': 1, 'KJo': 1, 'KTo': 1, 'K9o': 1,
      'QJo': 1, 'QTo': 1,
      'JTo': 1, 'T9o': 1
    })
  }
]

/**
 * 3Bet Presets
 */
const THREEBET_PRESETS: RangePreset[] = [
  {
    id: 'bb_vs_btn_3bet',
    name: 'BB vs BTN 3Bet',
    description: 'Big Blind 3Bet vs Button Open',
    position: 'BB',
    range: createRangeFromNotation({
      // Premium value
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1,
      'AKs': 1, 'AQs': 1, 'AJs': 1,
      'AKo': 1, 'AQo': 1,
      // Bluffs polarizados
      'A5s': 0.7, 'A4s': 0.7, 'A3s': 0.7, 'A2s': 0.7,
      'K9s': 0.3, 'K8s': 0.3,
      'Q9s': 0.3, 'J9s': 0.3,
      '87s': 0.3, '76s': 0.3, '65s': 0.3, '54s': 0.3
    })
  },
  {
    id: 'sb_vs_btn_3bet',
    name: 'SB vs BTN 3Bet',
    description: 'Small Blind 3Bet vs Button Open',
    position: 'SB',
    range: createRangeFromNotation({
      // Premium value
      'AA': 1, 'KK': 1, 'QQ': 1, 'JJ': 1, 'TT': 1, '99': 0.5,
      'AKs': 1, 'AQs': 1, 'AJs': 1, 'ATs': 0.5,
      'AKo': 1, 'AQo': 1, 'AJo': 0.5,
      // Bluffs
      'A5s': 0.5, 'A4s': 0.5, 'A3s': 0.5, 'A2s': 0.5,
      'K9s': 0.3, 'Q9s': 0.3,
      '76s': 0.3, '65s': 0.3, '54s': 0.3
    })
  }
]

/**
 * Blind Defense Presets
 */
const BLIND_DEFENSE_PRESETS: RangePreset[] = [
  {
    id: 'bb_vs_sb_call',
    name: 'BB vs SB Call',
    description: 'Big Blind Call vs Small Blind Open',
    position: 'BB',
    range: createRangeFromNotation({
      // Pairs medianas
      '99': 1, '88': 1, '77': 1, '66': 1, '55': 1, '44': 1, '33': 1, '22': 1,
      // Suited broadway
      'KQs': 1, 'KJs': 1, 'KTs': 1, 'K9s': 1,
      'QJs': 1, 'QTs': 1, 'Q9s': 1, 'Q8s': 1,
      'JTs': 1, 'J9s': 1, 'J8s': 1,
      'T9s': 1, 'T8s': 1, 'T7s': 1,
      // Suited connectors
      '98s': 1, '97s': 1, '87s': 1, '86s': 1, '76s': 1, '75s': 1, '65s': 1, '64s': 1, '54s': 1, '53s': 1,
      // Suited aces
      'A9s': 1, 'A8s': 1, 'A7s': 1, 'A6s': 1,
      // Offsuit broadway
      'KQo': 1, 'KJo': 1, 'KTo': 1,
      'QJo': 1, 'QTo': 1,
      'JTo': 1
    })
  }
]

// ============================================================================
// UTILIDADES
// ============================================================================

/**
 * Crea un RangeData desde notación compacta
 * Ejemplo: { 'AA': 1, 'AKs': 0.8, 'AKo': 0.5 }
 */
function createRangeFromNotation(notation: Record<string, number>): RangeData {
  const range: RangeData = {}

  for (const [hand, frequency] of Object.entries(notation)) {
    range[hand] = [
      {
        hand,
        action: 'RAISE',
        frequency
      }
    ]
  }

  return range
}

