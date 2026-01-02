export const Sessions = () => {
  return (
    <div>
      <h1 className="text-4xl font-bold text-white mb-6">Sesiones</h1>
      <div className="bg-slate-800 rounded-lg border border-slate-700 overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="border-b border-slate-700 bg-slate-900">
              <th className="px-6 py-3 text-left text-sm font-semibold text-slate-300">Fecha</th>
              <th className="px-6 py-3 text-left text-sm font-semibold text-slate-300">Duración</th>
              <th className="px-6 py-3 text-left text-sm font-semibold text-slate-300">Manos</th>
              <th className="px-6 py-3 text-left text-sm font-semibold text-slate-300">Ganancia</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b border-slate-700 hover:bg-slate-700 transition-colors">
              <td className="px-6 py-3 text-slate-300">2024-01-15</td>
              <td className="px-6 py-3 text-slate-300">2h 30m</td>
              <td className="px-6 py-3 text-slate-300">145</td>
              <td className="px-6 py-3 text-green-500 font-semibold">+$345</td>
            </tr>
            <tr className="border-b border-slate-700 hover:bg-slate-700 transition-colors">
              <td className="px-6 py-3 text-slate-300">2024-01-14</td>
              <td className="px-6 py-3 text-slate-300">1h 45m</td>
              <td className="px-6 py-3 text-slate-300">98</td>
              <td className="px-6 py-3 text-green-500 font-semibold">+$212</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  )
}

