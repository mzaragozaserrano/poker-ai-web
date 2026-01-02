import React, { useState } from 'react'
import type { PlaybackSpeed } from '../../../types/poker'

interface ReplayerControlsProps {
  isPlaying: boolean
  currentActionIndex: number
  totalActions: number
  playbackSpeed: PlaybackSpeed
  onPlay: () => void
  onPause: () => void
  onStop: () => void
  onStepForward: () => void
  onStepBackward: () => void
  onSetSpeed: (speed: PlaybackSpeed) => void
}

const SPEED_OPTIONS: PlaybackSpeed[] = [1, 2, 5, 10]

/**
 * Componente de controles para el reproductor de manos
 * Incluye botones Play/Pause, Step, selector de velocidad
 */
export const ReplayerControls: React.FC<ReplayerControlsProps> = ({
  isPlaying,
  currentActionIndex,
  totalActions,
  playbackSpeed,
  onPlay,
  onPause,
  onStop,
  onStepForward,
  onStepBackward,
  onSetSpeed,
}) => {
  const [showSpeedMenu, setShowSpeedMenu] = useState(false)
  const progress = totalActions > 0 ? Math.round((currentActionIndex / (totalActions - 1)) * 100) : 0

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 p-4 space-y-4">
      {/* Indicador de progreso */}
      <div className="space-y-2">
        <div className="flex justify-between text-xs text-slate-400">
          <span>Acción {currentActionIndex + 1} de {totalActions}</span>
          <span>{progress}%</span>
        </div>
        <div className="w-full bg-slate-700 rounded-full h-2 overflow-hidden">
          <div
            className="bg-violet-600 h-full transition-all duration-200"
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>

      {/* Controles principales */}
      <div className="flex items-center gap-2">
        {/* Stop button */}
        <button
          onClick={onStop}
          className="p-2 rounded-lg bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          title="Detener reproducción"
          disabled={currentActionIndex === 0 && !isPlaying}
        >
          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
            <rect x="5" y="5" width="10" height="10" />
          </svg>
        </button>

        {/* Step backward button */}
        <button
          onClick={onStepBackward}
          className="p-2 rounded-lg bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          title="Retroceder una acción"
          disabled={currentActionIndex === 0}
        >
          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10.536 3.464A8 8 0 1 0 17 11H13.172a.5.5 0 0 1-.707-.707l1.415-1.415a.5.5 0 0 0-.707-.707l-1.415 1.415L11.465 8l1.415-1.414a.5.5 0 0 0-.707-.707l-1.415 1.414L10.343 6l1.415-1.414a.5.5 0 0 0-.707-.707L9.636 4.293V3.464M5 10a1 1 0 1 1 2 0 1 1 0 0 1-2 0" />
          </svg>
        </button>

        {/* Play/Pause button */}
        <button
          onClick={isPlaying ? onPause : onPlay}
          className="px-4 py-2 rounded-lg bg-violet-600 hover:bg-violet-700 text-white font-medium transition-colors flex items-center gap-2"
          title={isPlaying ? 'Pausar reproducción' : 'Iniciar reproducción'}
        >
          {isPlaying ? (
            <>
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <rect x="4" y="4" width="3" height="12" />
                <rect x="13" y="4" width="3" height="12" />
              </svg>
              <span>Pausa</span>
            </>
          ) : (
            <>
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path d="M4.5 3a.5.5 0 0 0-.5.5v13a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5V3.5a.5.5 0 0 0-.5-.5h-1M14.5 3a.5.5 0 0 0-.5.5v13a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5V3.5a.5.5 0 0 0-.5-.5h-1" />
              </svg>
              <span>Reproducir</span>
            </>
          )}
        </button>

        {/* Step forward button */}
        <button
          onClick={onStepForward}
          className="p-2 rounded-lg bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          title="Avanzar una acción"
          disabled={currentActionIndex >= totalActions - 1}
        >
          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
            <path d="M9.464 3.464A8 8 0 1 1 3 11h4.828a.5.5 0 0 0 .707-.707L7.12 8.878a.5.5 0 0 1 .707-.707l1.414 1.415L10.657 8l-1.415-1.414a.5.5 0 0 1 .707-.707l1.414 1.414L12.778 6l-1.415-1.414a.5.5 0 0 1 .707-.707l1.414 1.414V3.464" />
          </svg>
        </button>

        {/* Speed selector */}
        <div className="relative">
          <button
            onClick={() => setShowSpeedMenu(!showSpeedMenu)}
            className="px-3 py-2 rounded-lg bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white transition-colors text-sm font-medium"
            title="Cambiar velocidad"
          >
            {playbackSpeed}x
          </button>

          {/* Speed menu dropdown */}
          {showSpeedMenu && (
            <div className="absolute right-0 mt-2 bg-slate-700 border border-slate-600 rounded-lg shadow-lg z-10">
              {SPEED_OPTIONS.map((speed) => (
                <button
                  key={speed}
                  onClick={() => {
                    onSetSpeed(speed)
                    setShowSpeedMenu(false)
                  }}
                  className={`block w-full text-left px-4 py-2 text-sm transition-colors ${
                    speed === playbackSpeed
                      ? 'bg-violet-600 text-white'
                      : 'text-slate-300 hover:bg-slate-600 hover:text-white'
                  }`}
                >
                  {speed}x velocidad
                </button>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Información de estado */}
      <div className="flex items-center justify-between text-xs text-slate-400 pt-2 border-t border-slate-700">
        <span>
          Estado: <span className="text-slate-300 font-medium">{isPlaying ? 'Reproduciendo' : 'Pausado'}</span>
        </span>
        <span>
          Velocidad: <span className="text-slate-300 font-medium">{playbackSpeed}x</span>
        </span>
      </div>
    </div>
  )
}

