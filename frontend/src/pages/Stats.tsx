export const Stats = () => {
  return (
    <div>
      <h1 className="text-4xl font-bold text-white mb-6">Estadísticas</h1>
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h2 className="text-xl font-bold text-white mb-4">Evolución de Ganancia</h2>
          <div className="h-64 bg-slate-900 rounded flex items-center justify-center">
            <p className="text-slate-500">Gráfico con Recharts</p>
          </div>
        </div>
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6">
          <h2 className="text-xl font-bold text-white mb-4">Resumen</h2>
          <div className="space-y-4">
            <div>
              <p className="text-slate-400 text-sm">Hands/Hour</p>
              <p className="text-2xl font-bold text-white">35.2</p>
            </div>
            <div>
              <p className="text-slate-400 text-sm">Win Rate (bb/100)</p>
              <p className="text-2xl font-bold text-green-500">8.3</p>
            </div>
            <div>
              <p className="text-slate-400 text-sm">ROI</p>
              <p className="text-2xl font-bold text-blue-500">+15.2%</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

