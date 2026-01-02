import { Outlet } from 'react-router-dom'
import { Sidebar } from '../components/Sidebar'
import { useState } from 'react'
import clsx from 'clsx'

export const MainLayout = () => {
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false)

  return (
    <div className="flex h-screen bg-slate-950">
      <Sidebar
        isCollapsed={isSidebarCollapsed}
        onToggleCollapse={() => setIsSidebarCollapsed(!isSidebarCollapsed)}
      />
      <main
        className={clsx(
          'flex-1 overflow-auto transition-all duration-300',
          isSidebarCollapsed ? 'ml-20' : 'ml-64'
        )}
      >
        <div className="p-8">
          <Outlet />
        </div>
      </main>
    </div>
  )
}

