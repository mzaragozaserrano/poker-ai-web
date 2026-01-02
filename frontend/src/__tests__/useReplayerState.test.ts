/**
 * Test suite para useReplayerState hook
 * 
 * NOTA: Este es un archivo de especificación de pruebas.
 * Para ejecutar estas pruebas, se necesita configurar:
 * - Vitest + React Testing Library
 * - O Jest + React Testing Library
 * 
 * Instalación:
 * npm install -D vitest @vitest/ui @testing-library/react @testing-library/jest-dom
 * 
 * Configuración en vite.config.ts:
 * ```
 * import { defineConfig } from 'vite'
 * import react from '@vitejs/plugin-react'
 * 
 * export default defineConfig({
 *   plugins: [react()],
 *   test: {
 *     globals: true,
 *     environment: 'jsdom',
 *     setupFiles: ['./src/__tests__/setup.ts'],
 *   }
 * })
 * ```
 */

import { renderHook, act } from '@testing-library/react'
import { describe, it, expect, beforeEach } from 'vitest'
import { useReplayerState } from '../hooks/useReplayerState'

describe('useReplayerState', () => {
  describe('Inicialización', () => {
    it('debe inicializar con estado idle', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))
      expect(result.current.state.state).toBe('idle')
      expect(result.current.state.currentActionIndex).toBe(0)
      expect(result.current.state.totalActions).toBe(10)
      expect(result.current.state.playbackSpeed).toBe(1)
    })

    it('debe permitir establecer velocidad inicial', () => {
      const { result } = renderHook(() =>
        useReplayerState({ totalActions: 10, initialSpeed: 2 })
      )
      expect(result.current.state.playbackSpeed).toBe(2)
    })
  })

  describe('Transiciones de Estado', () => {
    it('debe cambiar a playing cuando se llama play()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.play()
      })

      expect(result.current.state.state).toBe('playing')
      expect(result.current.state.isPaused).toBe(false)
    })

    it('debe cambiar a paused cuando se llama pause()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.play()
        result.current.pause()
      })

      expect(result.current.state.state).toBe('paused')
      expect(result.current.state.isPaused).toBe(true)
    })

    it('debe volver a idle cuando se llama stop()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.play()
        result.current.stepForward()
        result.current.stop()
      })

      expect(result.current.state.state).toBe('idle')
      expect(result.current.state.currentActionIndex).toBe(0)
      expect(result.current.state.isPaused).toBe(false)
    })

    it('debe cambiar a finished cuando se llama finish()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.finish()
      })

      expect(result.current.state.state).toBe('finished')
      expect(result.current.state.isPaused).toBe(true)
    })
  })

  describe('Navegación de Acciones', () => {
    it('debe avanzar a la siguiente acción con stepForward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.stepForward()
      })

      expect(result.current.state.currentActionIndex).toBe(1)
    })

    it('debe retroceder a la acción anterior con stepBackward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.stepForward()
        result.current.stepForward()
        result.current.stepBackward()
      })

      expect(result.current.state.currentActionIndex).toBe(1)
    })

    it('no debe exceder el máximo de acciones con stepForward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        for (let i = 0; i < 15; i++) {
          result.current.stepForward()
        }
      })

      expect(result.current.state.currentActionIndex).toBe(9)
    })

    it('no debe ir por debajo de 0 con stepBackward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.stepBackward()
        result.current.stepBackward()
      })

      expect(result.current.state.currentActionIndex).toBe(0)
    })

    it('debe saltar a una acción específica con jumpToAction()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.jumpToAction(5)
      })

      expect(result.current.state.currentActionIndex).toBe(5)
      expect(result.current.state.state).toBe('paused')
    })

    it('debe validar límites en jumpToAction()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.jumpToAction(20)
      })

      expect(result.current.state.currentActionIndex).toBe(9)

      act(() => {
        result.current.jumpToAction(-5)
      })

      expect(result.current.state.currentActionIndex).toBe(0)
    })
  })

  describe('Control de Velocidad', () => {
    it('debe cambiar la velocidad con setSpeed()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.setSpeed(5)
      })

      expect(result.current.state.playbackSpeed).toBe(5)

      act(() => {
        result.current.setSpeed(10)
      })

      expect(result.current.state.playbackSpeed).toBe(10)
    })

    it('debe permitir cambiar velocidad durante reproducción', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.play()
        result.current.setSpeed(2)
      })

      expect(result.current.state.state).toBe('playing')
      expect(result.current.state.playbackSpeed).toBe(2)
    })
  })

  describe('Comportamiento al Terminar', () => {
    it('debe cambiar a finished después de la última acción con stepForward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 3 }))

      act(() => {
        result.current.stepForward()
        result.current.stepForward()
        result.current.stepForward()
      })

      expect(result.current.state.state).toBe('finished')
      expect(result.current.state.currentActionIndex).toBe(2)
    })

    it('debe pausar automáticamente al usar stepForward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.play()
        result.current.stepForward()
      })

      expect(result.current.state.state).toBe('paused')
      expect(result.current.state.isPaused).toBe(true)
    })

    it('debe pausar automáticamente al usar stepBackward()', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 10 }))

      act(() => {
        result.current.play()
        result.current.stepForward()
        result.current.stepForward()
        result.current.stepBackward()
      })

      expect(result.current.state.state).toBe('paused')
      expect(result.current.state.isPaused).toBe(true)
    })
  })

  describe('Casos de Uso Completos', () => {
    it('escenario: reproducción completa', () => {
      const { result } = renderHook(() => useReplayerState({ totalActions: 5 }))

      // Iniciar reproducción
      act(() => {
        result.current.play()
      })
      expect(result.current.state.state).toBe('playing')

      // Avanzar manual
      act(() => {
        result.current.stepForward()
      })
      expect(result.current.state.currentActionIndex).toBe(1)
      expect(result.current.state.state).toBe('paused')

      // Reanudar
      act(() => {
        result.current.play()
      })
      expect(result.current.state.state).toBe('playing')

      // Cambiar velocidad
      act(() => {
        result.current.setSpeed(2)
      })
      expect(result.current.state.playbackSpeed).toBe(2)

      // Pausar
      act(() => {
        result.current.pause()
      })
      expect(result.current.state.state).toBe('paused')

      // Saltar a final
      act(() => {
        result.current.jumpToAction(4)
      })
      expect(result.current.state.currentActionIndex).toBe(4)

      // Terminar
      act(() => {
        result.current.finish()
      })
      expect(result.current.state.state).toBe('finished')

      // Reiniciar
      act(() => {
        result.current.stop()
      })
      expect(result.current.state.state).toBe('idle')
      expect(result.current.state.currentActionIndex).toBe(0)
    })
  })
})

