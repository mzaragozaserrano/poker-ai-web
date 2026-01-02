import { Link, useLocation } from 'react-router-dom'
import clsx from 'clsx'

const navItems = [
  { href: '/', label: 'Dashboard', icon: '📊' },
  { href: '/sessions', label: 'Sesiones', icon: '🎮' },
  { href: '/hands/:handId', label: 'Manos', icon: '🃏' },
  { href: '/stats', label: 'Estadísticas', icon: '📈' },
  { href: '/settings', label: 'Configuración', icon: '⚙️' },
]

export interface SidebarProps {
  isCollapsed: boolean
  onToggleCollapse: () => void
}

export const Sidebar = ({ isCollapsed, onToggleCollapse }: SidebarProps) => {
  const location = useLocation()

  const isActiveRoute = (href: string) => {
    if (href === '/' && location.pathname === '/') return true
    if (href !== '/' && location.pathname.startsWith(href.split(':')[0])) return true
    return false
  }

  return (
    <aside
      className={clsx(
        'fixed left-0 top-0 h-screen bg-slate-800 border-r border-slate-700 transition-all duration-300 z-40',
        isCollapsed ? 'w-20' : 'w-64'
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between h-16 px-4 border-b border-slate-700">
        {!isCollapsed && (
          <h1 className="text-xl font-bold text-violet-400">Poker AI</h1>
        )}
        <button
          onClick={onToggleCollapse}
          className="p-2 hover:bg-slate-700 rounded-md transition-colors"
          title={isCollapsed ? 'Expandir' : 'Contraer'}
        >
          {isCollapsed ? '→' : '←'}
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4">
        <ul className="space-y-2 px-2">
          {navItems.map((item) => (
            <li key={item.href}>
              <Link
                to={item.href === '/hands/:handId' ? '/hands/example' : item.href}
                className={clsx(
                  'flex items-center gap-3 px-4 py-2 rounded-md transition-colors',
                  isActiveRoute(item.href)
                    ? 'bg-violet-500 text-white'
                    : 'text-slate-300 hover:bg-slate-700'
                )}
                title={isCollapsed ? item.label : undefined}
              >
                <span className="text-lg">{item.icon}</span>
                {!isCollapsed && <span>{item.label}</span>}
              </Link>
            </li>
          ))}
        </ul>
      </nav>

      {/* Footer */}
      <div className="border-t border-slate-700 p-4">
        {!isCollapsed && (
          <div className="text-sm text-slate-400">
            <p className="font-semibold text-slate-300">thesmoy</p>
            <p className="text-xs">Online</p>
          </div>
        )}
      </div>
    </aside>
  )
}

