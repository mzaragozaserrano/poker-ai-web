import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import './App.css'

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-slate-950">
        <Routes>
          <Route path="/" element={<HomePage />} />
        </Routes>
      </div>
    </Router>
  )
}

function HomePage() {
  return (
    <div className="flex items-center justify-center min-h-screen bg-slate-950">
      <div className="text-center">
        <h1 className="text-4xl font-bold text-slate-100 mb-4">
          Poker AI Web
        </h1>
        <p className="text-slate-400 mb-8">
          High-Performance Analysis Platform for Winamax
        </p>
        <div className="flex gap-4 justify-center">
          <button className="px-6 py-3 bg-accent-violet text-white rounded-lg hover:opacity-90 transition-opacity">
            Get Started
          </button>
          <button className="px-6 py-3 border border-slate-700 text-slate-300 rounded-lg hover:bg-slate-900 transition-colors">
            Learn More
          </button>
        </div>
      </div>
    </div>
  )
}

export default App
