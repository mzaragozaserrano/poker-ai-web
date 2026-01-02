import { useParams } from 'react-router-dom'

export const HandReplayer = () => {
  const { handId } = useParams()

  return (
    <div>
      <h1 className="text-4xl font-bold text-white mb-6">Hand Replayer</h1>
      <p className="text-slate-400 mb-4">Mano ID: {handId}</p>
      <div className="bg-slate-800 rounded-lg border border-slate-700 p-8 h-96 flex items-center justify-center">
        <div className="text-center">
          <p className="text-slate-400 text-lg mb-4">🃏 Replayer de Mano</p>
          <p className="text-slate-500">Canvas con React-Konva estará disponible aquí</p>
        </div>
      </div>
    </div>
  )
}

