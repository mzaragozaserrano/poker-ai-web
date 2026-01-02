/** @type {import('tailwindcss').Config} */

// PALETA DE COLORES DARK MODE PARA POKER
// ========================================
// Documentación: docs/project/ui-foundations.md
//
// Base Slate (Fondo y Superficies)
// - slate-950: #0F172A - Background principal profundo
// - slate-800: #1E293B - Surface, tarjetas, paneles
// - slate-700: #334155 - Borders y divisores
//
// Colores de Acciones de Poker
// - poker-raise: #EF4444 (red-500) - Agresividad, raise
// - poker-call: #3B82F6 (blue-500) - Pasividad, call
// - poker-fold: #64748B (slate-500) - Fold/descarte
// - poker-equity-high: #10B981 (emerald-500) - Probabilidad alta
//
// Acento
// - accent-violet: #8B5CF6 (violet-500) - Acciones primarias, Hero

export default {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  
  // Dark mode forzado como único modo disponible
  darkMode: 'class',
  
  theme: {
    // Override de paleta completa (no solo extend)
    colors: {
      // Colores de utilidad de Tailwind
      transparent: 'transparent',
      current: 'currentColor',
      white: '#FFFFFF',
      black: '#000000',
      
      // Paleta Slate personalizada para dark mode
      slate: {
        50: '#F8FAFC',
        100: '#F1F5F9',
        200: '#E2E8F0',
        300: '#CBD5E1',
        400: '#94A3B8',
        500: '#64748B',
        600: '#475569',
        700: '#334155',
        800: '#1E293B',
        900: '#0F172A',
        950: '#0F172A', // Principal background
      },
      
      // Colores específicos de Poker (acciones)
      'poker-raise': '#EF4444',   // Red - Raise/Agresividad
      'poker-call': '#3B82F6',    // Blue - Call/Pasividad
      'poker-fold': '#64748B',    // Slate - Fold/Descarte
      'poker-equity-high': '#10B981', // Emerald - Probabilidad alta
      
      // Accent para Hero y acciones primarias
      accent: {
        violet: '#8B5CF6',
      },
      
      // Colores de estado (reutilizados de Tailwind)
      red: {
        500: '#EF4444',
      },
      blue: {
        500: '#3B82F6',
      },
      emerald: {
        500: '#10B981',
      },
      violet: {
        500: '#8B5CF6',
      },
      gray: {
        100: '#F3F4F6',
        200: '#E5E7EB',
        300: '#D1D5DB',
        400: '#9CA3AF',
        500: '#6B7280',
        600: '#4B5563',
        700: '#374151',
        800: '#1F2937',
        900: '#111827',
      },
    },
    
    extend: {
      // Extensiones adicionales de tema
      spacing: {
        // Custom spacing si es necesario
      },
      fontSize: {
        // Custom font sizes si es necesario
      },
      borderRadius: {
        // Custom border radius si es necesario
      },
    },
  },
  
  plugins: [],
  
  // Configuración de Corepack para asegurar reproducibilidad
  corePlugins: {
    // Desactivar plugins que no sean necesarios en dark mode only
  },
}
