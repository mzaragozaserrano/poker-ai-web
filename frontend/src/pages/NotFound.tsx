import { Link } from 'react-router-dom'

export const NotFound = () => {
  return (
    <div className="flex items-center justify-center min-h-screen bg-slate-950">
      <div className="text-center">
        <h1 className="text-6xl font-bold text-white mb-4">404</h1>
        <p className="text-xl text-slate-400 mb-8">Página no encontrada</p>
        <Link
          to="/"
          className="inline-block px-6 py-2 bg-violet-500 text-white font-semibold rounded-md hover:bg-violet-600 transition-colors"
        >
          Volver al Dashboard
        </Link>
      </div>
    </div>
  )
}

