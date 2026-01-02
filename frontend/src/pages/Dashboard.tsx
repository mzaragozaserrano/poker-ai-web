export const Dashboard = () => {
  return (
    <div>
      <h1 className="text-4xl font-bold text-white mb-6">Dashboard</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="bg-slate-800 p-6 rounded-lg border border-slate-700">
          <h2 className="text-slate-400 text-sm font-semibold mb-2">Manos Jugadas</h2>
          <p className="text-3xl font-bold text-white">1,234</p>
        </div>
        <div className="bg-slate-800 p-6 rounded-lg border border-slate-700">
          <h2 className="text-slate-400 text-sm font-semibold mb-2">Ganancia</h2>
          <p className="text-3xl font-bold text-green-500">+$2,450</p>
        </div>
        <div className="bg-slate-800 p-6 rounded-lg border border-slate-700">
          <h2 className="text-slate-400 text-sm font-semibold mb-2">VPIP</h2>
          <p className="text-3xl font-bold text-blue-500">28%</p>
        </div>
        <div className="bg-slate-800 p-6 rounded-lg border border-slate-700">
          <h2 className="text-slate-400 text-sm font-semibold mb-2">PFR</h2>
          <p className="text-3xl font-bold text-violet-500">21%</p>
        </div>
      </div>
    </div>
  )
}

