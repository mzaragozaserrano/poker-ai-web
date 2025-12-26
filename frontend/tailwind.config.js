/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Base palette - Dark Mode
        'slate-950': '#0F172A',
        'slate-900': '#0F172A',
        'slate-800': '#1E293B',
        'slate-700': '#334155',
        
        // Poker action colors
        'poker-raise': '#EF4444',
        'poker-call': '#3B82F6',
        'poker-fold': '#64748B',
        'poker-equity-high': '#10B981',
        
        // Accent
        'accent-violet': '#8B5CF6',
      },
      backgroundColor: {
        primary: '#0F172A',
        surface: '#1E293B',
        'poker-raise': '#EF4444',
        'poker-call': '#3B82F6',
        'poker-fold': '#64748B',
        'poker-equity-high': '#10B981',
      },
      textColor: {
        primary: '#F1F5F9',
        secondary: '#CBD5E1',
        'poker-raise': '#EF4444',
        'poker-call': '#3B82F6',
      },
      borderColor: {
        primary: '#334155',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}



