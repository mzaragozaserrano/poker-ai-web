import { useReducer, useCallback } from 'react'
import type { ReplayerState, ReplayerAction, PlaybackSpeed } from '../types/poker'

/**
 * Máquina de estados para el reproductor de manos
 * Estados: idle, playing, paused, finished
 */
function replayerReducer(state: ReplayerState, action: ReplayerAction): ReplayerState {
  switch (action.type) {
    case 'PLAY':
      return {
        ...state,
        state: 'playing',
        isPaused: false,
      }

    case 'PAUSE':
      return {
        ...state,
        state: 'paused',
        isPaused: true,
      }

    case 'STOP':
      return {
        ...state,
        state: 'idle',
        currentActionIndex: 0,
        isPaused: false,
      }

    case 'STEP_FORWARD':
      return {
        ...state,
        currentActionIndex: Math.min(state.currentActionIndex + 1, state.totalActions - 1),
        state: state.currentActionIndex + 1 >= state.totalActions ? 'finished' : 'paused',
        isPaused: true,
      }

    case 'STEP_BACKWARD':
      return {
        ...state,
        currentActionIndex: Math.max(state.currentActionIndex - 1, 0),
        state: 'paused',
        isPaused: true,
      }

    case 'SET_SPEED': {
      const speed = action.payload as PlaybackSpeed
      return {
        ...state,
        playbackSpeed: speed,
      }
    }

    case 'JUMP_TO_ACTION': {
      const index = action.payload as number
      return {
        ...state,
        currentActionIndex: Math.max(0, Math.min(index, state.totalActions - 1)),
        state: 'paused',
        isPaused: true,
      }
    }

    case 'FINISH':
      return {
        ...state,
        state: 'finished',
        currentActionIndex: state.totalActions - 1,
        isPaused: true,
      }

    default:
      return state
  }
}

interface UseReplayerStateOptions {
  totalActions: number
  initialSpeed?: PlaybackSpeed
}

/**
 * Hook para manejar la lógica de reproducción de manos
 * Proporciona el estado actual y funciones para controlar la reproducción
 */
export function useReplayerState(options: UseReplayerStateOptions) {
  const { totalActions, initialSpeed = 1 } = options

  const initialState: ReplayerState = {
    state: 'idle',
    currentActionIndex: 0,
    totalActions,
    playbackSpeed: initialSpeed,
    isPaused: false,
  }

  const [state, dispatch] = useReducer(replayerReducer, initialState)

  const play = useCallback(() => {
    dispatch({ type: 'PLAY' })
  }, [])

  const pause = useCallback(() => {
    dispatch({ type: 'PAUSE' })
  }, [])

  const stop = useCallback(() => {
    dispatch({ type: 'STOP' })
  }, [])

  const stepForward = useCallback(() => {
    dispatch({ type: 'STEP_FORWARD' })
  }, [])

  const stepBackward = useCallback(() => {
    dispatch({ type: 'STEP_BACKWARD' })
  }, [])

  const setSpeed = useCallback((speed: PlaybackSpeed) => {
    dispatch({ type: 'SET_SPEED', payload: speed })
  }, [])

  const jumpToAction = useCallback((index: number) => {
    dispatch({ type: 'JUMP_TO_ACTION', payload: index })
  }, [])

  const finish = useCallback(() => {
    dispatch({ type: 'FINISH' })
  }, [])

  return {
    state,
    play,
    pause,
    stop,
    stepForward,
    stepBackward,
    setSpeed,
    jumpToAction,
    finish,
  }
}

