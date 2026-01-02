import { RouteObject } from 'react-router-dom'
import { MainLayout } from './layouts/MainLayout'
import { Dashboard } from './pages/Dashboard'
import { Sessions } from './pages/Sessions'
import { HandReplayer } from './pages/HandReplayer'
import { Stats } from './pages/Stats'
import { Settings } from './pages/Settings'
import { NotFound } from './pages/NotFound'

export const routes: RouteObject[] = [
  {
    path: '/',
    element: <MainLayout />,
    children: [
      {
        index: true,
        element: <Dashboard />,
      },
      {
        path: 'sessions',
        element: <Sessions />,
      },
      {
        path: 'hands/:handId',
        element: <HandReplayer />,
      },
      {
        path: 'stats',
        element: <Stats />,
      },
      {
        path: 'settings',
        element: <Settings />,
      },
    ],
  },
  {
    path: '*',
    element: <NotFound />,
  },
]

