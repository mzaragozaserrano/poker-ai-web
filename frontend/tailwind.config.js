/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'poker-raise': '#EF4444',
        'poker-call': '#3B82F6',
        'poker-fold': '#64748B',
        'poker-equity-high': '#10B981',
      },
    },
  },
  plugins: [],
}
