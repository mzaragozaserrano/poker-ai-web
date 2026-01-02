import React from 'react';

/**
 * ColorPaletteReference - Componente para visualizar la paleta de colores
 * Referencia: docs/project/ui-foundations.md
 * Ubicación: frontend/src/components/ColorPaletteReference.tsx
 *
 * Este componente es solo para referencia visual durante el desarrollo.
 * NO incluir en producción.
 */

interface ColorItem {
  name: string;
  tailwindClass: string;
  cssVariable: string;
  hex: string;
  description: string;
}

const PALETTE_BASE: ColorItem[] = [
  {
    name: 'Slate 950 - Background Principal',
    tailwindClass: 'bg-slate-950',
    cssVariable: '--color-slate-950',
    hex: '#0F172A',
    description: 'Fondo principal profundo para contraste óptimo',
  },
  {
    name: 'Slate 800 - Surface',
    tailwindClass: 'bg-slate-800',
    cssVariable: '--color-slate-800',
    hex: '#1E293B',
    description: 'Tarjetas, paneles, modales',
  },
  {
    name: 'Slate 700 - Borders',
    tailwindClass: 'bg-slate-700',
    cssVariable: '--color-slate-700',
    hex: '#334155',
    description: 'Bordes y divisores sutiles',
  },
];

const PALETTE_POKER: ColorItem[] = [
  {
    name: 'Poker Raise - Agresividad',
    tailwindClass: 'bg-poker-raise',
    cssVariable: '--color-poker-raise',
    hex: '#EF4444',
    description: 'Rojo - Raise, all-in, acciones agresivas',
  },
  {
    name: 'Poker Call - Pasividad',
    tailwindClass: 'bg-poker-call',
    cssVariable: '--color-poker-call',
    hex: '#3B82F6',
    description: 'Azul - Call, check, acciones pasivas',
  },
  {
    name: 'Poker Fold - Descarte',
    tailwindClass: 'bg-poker-fold',
    cssVariable: '--color-poker-fold',
    hex: '#64748B',
    description: 'Gris - Fold, manos descartadas',
  },
  {
    name: 'Poker Equity High',
    tailwindClass: 'bg-poker-equity-high',
    cssVariable: '--color-poker-equity-high',
    hex: '#10B981',
    description: 'Verde - Alta probabilidad de victoria',
  },
];

const PALETTE_ACCENT: ColorItem[] = [
  {
    name: 'Accent Violet - Hero',
    tailwindClass: 'bg-accent-violet',
    cssVariable: '--color-accent-violet',
    hex: '#8B5CF6',
    description: 'Violeta - Acciones primarias, Hero (thesmoy)',
  },
];

const ColorSwatch: React.FC<ColorItem> = ({
  name,
  tailwindClass,
  cssVariable,
  hex,
  description,
}) => (
  <div className="card mb-4">
    <div className={`${tailwindClass} h-24 rounded mb-3`} />
    <h3 className="text-slate-100 font-semibold mb-2">{name}</h3>
    <p className="text-slate-400 text-sm mb-3">{description}</p>
    <div className="grid grid-cols-2 gap-2 text-xs font-mono">
      <div>
        <span className="text-slate-500">Tailwind:</span>
        <div className="text-slate-200 bg-slate-900 px-2 py-1 rounded mt-1">
          {tailwindClass}
        </div>
      </div>
      <div>
        <span className="text-slate-500">Hex:</span>
        <div className="text-slate-200 bg-slate-900 px-2 py-1 rounded mt-1">
          {hex}
        </div>
      </div>
      <div className="col-span-2">
        <span className="text-slate-500">CSS Variable:</span>
        <div className="text-slate-200 bg-slate-900 px-2 py-1 rounded mt-1">
          var({cssVariable})
        </div>
      </div>
    </div>
  </div>
);

export const ColorPaletteReference: React.FC = () => {
  return (
    <div className="bg-slate-950 p-6 min-h-screen">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-4xl font-bold text-slate-100 mb-2">
          Paleta de Colores - Poker AI
        </h1>
        <p className="text-slate-400 mb-8">
          Referencia visual de la paleta Dark Mode. Solo para desarrollo.
        </p>

        {/* Sección Base */}
        <section className="mb-10">
          <h2 className="text-2xl font-bold text-slate-100 mb-4">
            Base - Slate
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {PALETTE_BASE.map((color) => (
              <ColorSwatch key={color.hex} {...color} />
            ))}
          </div>
        </section>

        {/* Sección Poker */}
        <section className="mb-10">
          <h2 className="text-2xl font-bold text-slate-100 mb-4">
            Acciones de Poker
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {PALETTE_POKER.map((color) => (
              <ColorSwatch key={color.hex} {...color} />
            ))}
          </div>
        </section>

        {/* Sección Accent */}
        <section className="mb-10">
          <h2 className="text-2xl font-bold text-slate-100 mb-4">
            Acento - Hero
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {PALETTE_ACCENT.map((color) => (
              <ColorSwatch key={color.hex} {...color} />
            ))}
          </div>
        </section>

        {/* Sección de Componentes */}
        <section className="mb-10">
          <h2 className="text-2xl font-bold text-slate-100 mb-4">
            Componentes de Ejemplo
          </h2>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* Botones */}
            <div className="card">
              <h3 className="text-slate-100 font-semibold mb-4">Botones</h3>
              <div className="space-y-2">
                <button className="btn bg-poker-raise text-white w-full hover:opacity-90">
                  RAISE
                </button>
                <button className="btn bg-poker-call text-white w-full hover:opacity-90">
                  CALL
                </button>
                <button className="btn bg-poker-fold text-white w-full hover:opacity-90">
                  FOLD
                </button>
                <button className="btn bg-accent-violet text-white w-full hover:opacity-90">
                  PRIMARY ACTION
                </button>
              </div>
            </div>

            {/* Badges */}
            <div className="card">
              <h3 className="text-slate-100 font-semibold mb-4">Badges</h3>
              <div className="flex flex-wrap gap-2">
                <span className="badge-raise">RAISE</span>
                <span className="badge-call">CALL</span>
                <span className="badge-fold">FOLD</span>
                <span className="badge-equity">EQUITY</span>
              </div>
            </div>

            {/* Texto */}
            <div className="card">
              <h3 className="text-slate-100 font-semibold mb-4">Variaciones de Texto</h3>
              <p className="text-slate-200 mb-2">Texto primario (slate-200)</p>
              <p className="text-slate-400 mb-2">Texto secundario (slate-400)</p>
              <p className="text-poker-raise mb-2">Texto raise (rojo)</p>
              <p className="text-poker-call mb-2">Texto call (azul)</p>
              <p className="text-accent-violet">Texto accent (violeta)</p>
            </div>

            {/* Bordes */}
            <div className="card">
              <h3 className="text-slate-100 font-semibold mb-4">Bordes</h3>
              <div className="border border-slate-700 p-3 rounded mb-2">
                Border slate-700
              </div>
              <div className="border border-poker-raise p-3 rounded">
                Border poker-raise
              </div>
            </div>
          </div>
        </section>

        {/* Instrucciones */}
        <section className="bg-slate-800 p-6 rounded-lg">
          <h2 className="text-2xl font-bold text-slate-100 mb-4">
            Cómo Usar
          </h2>
          <div className="space-y-4 text-slate-300">
            <div>
              <h3 className="font-semibold text-slate-100 mb-2">En Tailwind:</h3>
              <code className="bg-slate-900 px-3 py-2 rounded block font-mono text-sm">
                {'<div className="bg-poker-raise text-white">Raise</div>'}
              </code>
            </div>
            <div>
              <h3 className="font-semibold text-slate-100 mb-2">
                En CSS Puro:
              </h3>
              <code className="bg-slate-900 px-3 py-2 rounded block font-mono text-sm">
                {`background-color: var(--color-poker-raise);`}
              </code>
            </div>
            <div>
              <h3 className="font-semibold text-slate-100 mb-2">
                Documentación:
              </h3>
              <p>
                Consulta <code className="bg-slate-900 px-2 py-1 rounded">frontend/TAILWIND_PALETTE.md</code> para más detalles.
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
};

export default ColorPaletteReference;

