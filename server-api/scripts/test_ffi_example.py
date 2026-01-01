#!/usr/bin/env python3
"""
Script de ejemplo para probar el módulo FFI Rust-Python.

Este script demuestra cómo usar las funciones expuestas por el puente FFI
desde Python.

Requisitos:
    1. Rust instalado (rustup)
    2. Maturin instalado: pip install maturin
    3. Módulo FFI compilado: cd backend/ffi && maturin develop

Uso:
    python scripts/test_ffi_example.py
"""

import sys
import os

# Agregar el directorio padre al path para importar app
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def print_header(title: str) -> None:
    """Imprime un header formateado."""
    print("\n" + "=" * 60)
    print(f"  {title}")
    print("=" * 60)


def test_ffi_availability() -> bool:
    """Verifica si el módulo FFI está disponible."""
    print_header("Test: Disponibilidad FFI")
    
    from app.bridge import is_ffi_available, get_version, get_system_info
    
    available = is_ffi_available()
    print(f"FFI disponible: {available}")
    
    if available:
        print(f"Versión: {get_version()}")
        print(f"\nInfo del sistema:\n{get_system_info()}")
    else:
        print("\n[AVISO] El módulo FFI no está compilado.")
        print("Para compilar, ejecutar:")
        print("  cd backend/ffi")
        print("  maturin develop")
    
    return available


def test_simd_detection() -> None:
    """Verifica detección de SIMD AVX2."""
    print_header("Test: Detección SIMD AVX2")
    
    from app.bridge import is_simd_available
    
    simd = is_simd_available()
    print(f"SIMD AVX2 disponible: {simd}")
    
    if simd:
        print("Las simulaciones Monte Carlo usarán instrucciones vectoriales.")
    else:
        print("Las simulaciones usarán evaluación escalar (más lenta).")


def test_equity_calculation() -> None:
    """Prueba el cálculo de equity."""
    print_header("Test: Cálculo de Equity")
    
    from app.bridge import calculate_equity
    
    test_cases = [
        ("AsAh", "KsKh", "", "AA vs KK preflop"),
        ("AhKh", "QsQd", "", "AKs vs QQ preflop"),
        ("2s2h", "AcKd", "", "22 vs AKo (coinflip)"),
        ("AsAh", "KsKh", "Qh7s2c", "AA vs KK con board seco"),
        ("AhKh", "QsQd", "Kd7c2h", "Top pair vs overpair"),
    ]
    
    for hero, villain, board, description in test_cases:
        try:
            result = calculate_equity(hero, villain, board, 50000)
            board_str = board if board else "preflop"
            print(f"\n{description} ({board_str}):")
            print(f"  Hero ({hero}):    {result['hero_percent']:.1f}%")
            print(f"  Villain ({villain}): {result['villain_percent']:.1f}%")
            if result['tie_equity'] > 0.01:
                print(f"  Empate:           {result['tie_equity'] * 100:.1f}%")
            print(f"  Simulaciones: {result['simulations_run']:,}")
        except Exception as e:
            print(f"\n{description}: ERROR - {e}")


def test_equity_multiway() -> None:
    """Prueba el cálculo de equity multiway."""
    print_header("Test: Equity Multiway (3 jugadores)")
    
    from app.bridge import calculate_equity_multiway
    
    hands = ["AsAh", "KsKh", "QsQh"]
    
    try:
        equities = calculate_equity_multiway(hands, "", 30000)
        
        print(f"\nScenario: {' vs '.join(hands)}")
        for i, (hand, equity) in enumerate(zip(hands, equities)):
            print(f"  Jugador {i+1} ({hand}): {equity * 100:.1f}%")
        
        total = sum(equities)
        print(f"  Total (debe ser ~100%): {total * 100:.1f}%")
    except Exception as e:
        print(f"ERROR: {e}")


def test_parsing_mock() -> None:
    """Demuestra el uso del parsing (con datos mock si FFI no disponible)."""
    print_header("Test: Parsing de Historiales")
    
    from app.bridge import is_ffi_available
    
    if not is_ffi_available():
        print("FFI no disponible. Mostrando ejemplo de uso:")
        print("""
    from app.bridge import parse_files, parse_files_with_details

    # Parsear archivos
    result = parse_files(["history1.txt", "history2.txt"])
    print(f"Parseadas {result['total_hands']} manos")
    print(f"Tiempo: {result['elapsed_ms']}ms")

    # Obtener detalles
    hands = parse_files_with_details(["history.txt"])
    for hand in hands:
        if hand['hero_played']:
            print(f"Hand {hand['hand_id']} - Pot: {hand['total_pot_cents']}c")
        """)
    else:
        from app.bridge import parse_files
        # Aquí iría el test real con archivos
        print("Parsing disponible. Uso:")
        print("  result = parse_files(['path/to/history.txt'])")


def test_error_handling() -> None:
    """Demuestra el manejo de errores."""
    print_header("Test: Manejo de Errores")
    
    from app.bridge import is_ffi_available, calculate_equity
    
    if not is_ffi_available():
        print("FFI no disponible. Ejemplo de manejo de errores:")
        print("""
    try:
        result = calculate_equity("XxYy", "QsQh")  # Cartas inválidas
    except ValueError as e:
        print(f"Error de validación: {e}")  # [201] Carta inválida

    try:
        result = parse_files(["nonexistent.txt"])
    except IOError as e:
        print(f"Error de I/O: {e}")  # [101] No se puede acceder
        """)
        return
    
    # Test con cartas inválidas
    print("\nProbando cartas inválidas...")
    try:
        calculate_equity("XxYy", "QsQh")
    except ValueError as e:
        print(f"  Capturado ValueError: {e}")
    except Exception as e:
        print(f"  Error inesperado: {type(e).__name__}: {e}")


def main() -> None:
    """Función principal."""
    print("\n" + "#" * 60)
    print("#" + " " * 58 + "#")
    print("#" + "  POKER FFI - Test Suite de Ejemplo".center(58) + "#")
    print("#" + " " * 58 + "#")
    print("#" * 60)
    
    # Verificar disponibilidad
    ffi_available = test_ffi_availability()
    
    if ffi_available:
        # Tests que requieren FFI
        test_simd_detection()
        test_equity_calculation()
        test_equity_multiway()
    
    # Tests que funcionan con o sin FFI
    test_parsing_mock()
    test_error_handling()
    
    print_header("Resumen")
    if ffi_available:
        print("Todos los tests completados exitosamente.")
    else:
        print("Tests completados en modo demostración.")
        print("\nPara habilitar todas las funciones:")
        print("  1. Instalar Rust: https://rustup.rs/")
        print("  2. cd backend/ffi")
        print("  3. pip install maturin")
        print("  4. maturin develop")
    
    print("\n" + "=" * 60 + "\n")


if __name__ == "__main__":
    main()

