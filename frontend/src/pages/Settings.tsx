export const Settings = () => {
  return (
    <div>
      <h1 className="text-4xl font-bold text-white mb-6">Configuración</h1>
      <div className="max-w-2xl">
        <div className="bg-slate-800 rounded-lg border border-slate-700 p-6 space-y-6">
          <div>
            <h2 className="text-lg font-semibold text-white mb-4">Cuenta</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Nombre de Jugador
                </label>
                <input
                  type="text"
                  value="thesmoy"
                  disabled
                  className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-md text-slate-300 disabled:opacity-50"
                />
              </div>
            </div>
          </div>

          <div className="border-t border-slate-700 pt-6">
            <h2 className="text-lg font-semibold text-white mb-4">Preferencias</h2>
            <div className="space-y-4">
              <div className="flex items-center gap-3">
                <input
                  type="checkbox"
                  id="darkMode"
                  defaultChecked
                  className="w-4 h-4 rounded"
                />
                <label htmlFor="darkMode" className="text-slate-300">
                  Modo Oscuro (siempre activo)
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

