import React, { useState } from 'react';
import {
  Badge,
  Button,
  Card,
  Input,
  Modal,
  Navbar,
} from '../components';

/**
 * ComponentShowcase
 * 
 * Página de demostración de todos los componentes del sistema de diseño.
 * Muestra todas las variantes, tamaños y estados de cada componente.
 * 
 * Esta página sirve como:
 * - Referencia visual de componentes
 * - Guía de uso para desarrolladores
 * - Verificación de consistencia visual
 */
export const ComponentShowcase: React.FC = () => {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [inputValue, setInputValue] = useState('');

  return (
    <div className="min-h-screen bg-slate-950">
      {/* Navbar */}
      <Navbar
        logo={<span className="text-xl font-bold text-accent-violet">Poker AI</span>}
        items={[
          { id: '1', label: 'Dashboard', href: '#', isActive: true },
          { id: '2', label: 'Sesiones', href: '#' },
          { id: '3', label: 'Análisis', href: '#' },
        ]}
        userArea={
          <div className="flex items-center gap-3">
            <Badge variant="primary">thesmoy</Badge>
            <Button size="sm" variant="ghost">Salir</Button>
          </div>
        }
      />

      {/* Main Content */}
      <div className="px-6 py-8 max-w-7xl mx-auto">
        <header className="mb-12">
          <h1 className="text-4xl font-bold text-slate-200 mb-2">
            Sistema de Diseño - Componentes Base
          </h1>
          <p className="text-slate-400">
            Biblioteca de componentes reutilizables para Poker AI Web.
          </p>
        </header>

        <div className="space-y-16">
          {/* Buttons Section */}
          <section>
            <h2 className="text-2xl font-bold text-slate-200 mb-6">Buttons</h2>
            
            <div className="space-y-8">
              {/* Variantes */}
              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">Variantes</h3>
                <div className="flex flex-wrap gap-4">
                  <Button variant="primary">Primary</Button>
                  <Button variant="secondary">Secondary</Button>
                  <Button variant="ghost">Ghost</Button>
                  <Button variant="destructive">Destructive</Button>
                  <Button variant="raise">RAISE</Button>
                  <Button variant="call">CALL</Button>
                </div>
              </div>

              {/* Tamaños */}
              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">Tamaños</h3>
                <div className="flex flex-wrap items-center gap-4">
                  <Button size="sm">Small</Button>
                  <Button size="md">Medium</Button>
                  <Button size="lg">Large</Button>
                </div>
              </div>

              {/* Estados */}
              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">Estados</h3>
                <div className="flex flex-wrap gap-4">
                  <Button disabled>Disabled</Button>
                  <Button isLoading>Loading</Button>
                  <Button fullWidth>Full Width</Button>
                </div>
              </div>
            </div>
          </section>

          {/* Badges Section */}
          <section>
            <h2 className="text-2xl font-bold text-slate-200 mb-6">Badges</h2>
            
            <div className="space-y-8">
              {/* Variantes */}
              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">Variantes</h3>
                <div className="flex flex-wrap gap-3">
                  <Badge variant="default">Default</Badge>
                  <Badge variant="primary">Primary</Badge>
                  <Badge variant="success">Success</Badge>
                  <Badge variant="error">Error</Badge>
                  <Badge variant="warning">Warning</Badge>
                </div>
              </div>

              {/* Poker Specific */}
              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">Poker Actions</h3>
                <div className="flex flex-wrap gap-3">
                  <Badge variant="raise">RAISE</Badge>
                  <Badge variant="call">CALL</Badge>
                  <Badge variant="fold">FOLD</Badge>
                  <Badge variant="equity">HIGH EQUITY</Badge>
                </div>
              </div>

              {/* Tamaños */}
              <div>
                <h3 className="text-lg font-semibold text-slate-300 mb-4">Tamaños</h3>
                <div className="flex flex-wrap items-center gap-3">
                  <Badge size="sm">Small</Badge>
                  <Badge size="md">Medium</Badge>
                  <Badge size="lg">Large</Badge>
                </div>
              </div>
            </div>
          </section>

          {/* Inputs Section */}
          <section>
            <h2 className="text-2xl font-bold text-slate-200 mb-6">Inputs</h2>
            
            <div className="space-y-6 max-w-md">
              <Input
                label="Nombre de usuario"
                placeholder="thesmoy"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
              />
              
              <Input
                label="Email"
                type="email"
                placeholder="user@example.com"
                helperText="Tu email para notificaciones"
              />
              
              <Input
                label="Con error"
                variant="error"
                placeholder="email inválido"
                helperText="El formato del email es incorrecto"
              />
              
              <Input
                label="Con éxito"
                variant="success"
                value="thesmoy"
                helperText="Usuario válido"
              />
              
              <Input
                label="Deshabilitado"
                disabled
                value="No editable"
              />
            </div>
          </section>

          {/* Cards Section */}
          <section>
            <h2 className="text-2xl font-bold text-slate-200 mb-6">Cards</h2>
            
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {/* Card básica */}
              <Card>
                <h3 className="text-lg font-semibold text-slate-200 mb-2">
                  Card Básica
                </h3>
                <p className="text-slate-400">
                  Contenedor simple sin header ni footer.
                </p>
              </Card>

              {/* Card con header */}
              <Card
                header={
                  <h3 className="text-lg font-semibold text-slate-200">
                    Estadísticas
                  </h3>
                }
              >
                <div className="space-y-2">
                  <p className="text-slate-300">Manos: <span className="font-bold">245</span></p>
                  <p className="text-slate-300">VPIP: <span className="font-bold text-accent-violet">24%</span></p>
                  <p className="text-slate-300">PFR: <span className="font-bold text-accent-violet">18%</span></p>
                </div>
              </Card>

              {/* Card con footer */}
              <Card
                header={<h3 className="text-lg font-semibold text-slate-200">Mano #12345</h3>}
                footer={
                  <div className="flex gap-2">
                    <Button size="sm" variant="ghost">Ver</Button>
                    <Button size="sm" variant="primary">Analizar</Button>
                  </div>
                }
              >
                <div className="space-y-2">
                  <p className="text-slate-300">Pot: <span className="font-bold text-poker-equity-high">€5.50</span></p>
                  <p className="text-slate-300">Resultado: <span className="font-bold text-poker-equity-high">+€2.75</span></p>
                </div>
              </Card>

              {/* Card interactiva */}
              <Card
                interactive
                onClick={() => alert('Card clicked!')}
              >
                <h3 className="text-lg font-semibold text-slate-200 mb-2">
                  Card Interactiva
                </h3>
                <p className="text-slate-400">
                  Click para ver detalles
                </p>
              </Card>
            </div>
          </section>

          {/* Modal Section */}
          <section>
            <h2 className="text-2xl font-bold text-slate-200 mb-6">Modal</h2>
            
            <div className="space-y-4">
              <p className="text-slate-400 mb-4">
                Los modales son útiles para confirmaciones, formularios y contenido detallado.
              </p>
              
              <div className="flex gap-4">
                <Button onClick={() => setIsModalOpen(true)}>
                  Abrir Modal
                </Button>
              </div>

              <Modal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                title="Confirmar acción"
                footer={
                  <>
                    <Button
                      variant="ghost"
                      onClick={() => setIsModalOpen(false)}
                    >
                      Cancelar
                    </Button>
                    <Button
                      variant="primary"
                      onClick={() => {
                        alert('Confirmado!');
                        setIsModalOpen(false);
                      }}
                    >
                      Confirmar
                    </Button>
                  </>
                }
              >
                <p className="text-slate-300 mb-4">
                  ¿Estás seguro que deseas realizar esta acción?
                </p>
                <p className="text-slate-400 text-sm">
                  Esta acción no se puede deshacer.
                </p>
              </Modal>
            </div>
          </section>

          {/* Example: Stats Dashboard */}
          <section>
            <h2 className="text-2xl font-bold text-slate-200 mb-6">
              Ejemplo: Dashboard de Estadísticas
            </h2>
            
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <Card>
                <div className="flex items-center justify-between mb-2">
                  <h4 className="text-sm font-medium text-slate-400">VPIP</h4>
                  <Badge size="sm" variant="primary">Hero</Badge>
                </div>
                <p className="text-3xl font-bold text-accent-violet">24%</p>
                <p className="text-xs text-slate-500 mt-1">Voluntarily Put $ In Pot</p>
              </Card>

              <Card>
                <div className="flex items-center justify-between mb-2">
                  <h4 className="text-sm font-medium text-slate-400">PFR</h4>
                  <Badge size="sm" variant="raise">Agresivo</Badge>
                </div>
                <p className="text-3xl font-bold text-poker-raise">18%</p>
                <p className="text-xs text-slate-500 mt-1">Pre-Flop Raise</p>
              </Card>

              <Card>
                <div className="flex items-center justify-between mb-2">
                  <h4 className="text-sm font-medium text-slate-400">3-Bet</h4>
                  <Badge size="sm" variant="call">Estándar</Badge>
                </div>
                <p className="text-3xl font-bold text-poker-call">8%</p>
                <p className="text-xs text-slate-500 mt-1">3-Bet Frequency</p>
              </Card>

              <Card>
                <div className="flex items-center justify-between mb-2">
                  <h4 className="text-sm font-medium text-slate-400">Win Rate</h4>
                  <Badge size="sm" variant="equity">Positivo</Badge>
                </div>
                <p className="text-3xl font-bold text-poker-equity-high">+5.2</p>
                <p className="text-xs text-slate-500 mt-1">bb/100 hands</p>
              </Card>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
};

export default ComponentShowcase;

