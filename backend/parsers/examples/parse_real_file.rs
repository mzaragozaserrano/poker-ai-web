//! Ejemplo de parsing de un archivo real de historiales Winamax.
//!
//! Este ejemplo lee el archivo `docs/winamax/example_winamax.txt` y prueba
//! el parser con datos reales.

use poker_parsers::WinamaxParser;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Intentar diferentes rutas posibles
    let possible_paths = vec![
        PathBuf::from("docs/winamax/example_winamax.txt"), // Desde raíz del proyecto
        PathBuf::from("../docs/winamax/example_winamax.txt"), // Desde backend/
        PathBuf::from("../../docs/winamax/example_winamax.txt"), // Desde backend/parsers/
    ];

    let file_path = possible_paths
        .iter()
        .find(|p| p.exists())
        .cloned()
        .unwrap_or_else(|| PathBuf::from("docs/winamax/example_winamax.txt"));

    println!("=== Prueba del Parser Winamax con Archivo Real ===\n");
    println!("Leyendo archivo: {:?}", file_path);

    // Leer el archivo
    let content = match fs::read_to_string(&file_path) {
        Ok(content) => {
            println!("✓ Archivo leído exitosamente");
            println!(
                "  Tamaño: {} bytes ({} líneas)\n",
                content.len(),
                content.lines().count()
            );
            content
        }
        Err(e) => {
            eprintln!("✗ Error al leer el archivo: {}", e);
            eprintln!("  Asegúrate de ejecutar desde la raíz del proyecto");
            return;
        }
    };

    // Crear parser y parsear
    println!("Parseando historial...");
    let mut parser = WinamaxParser::new();
    let result = parser.parse(&content);

    // Mostrar resultados
    println!("\n=== Resultados del Parsing ===\n");
    println!("Manos parseadas: {}", result.hands.len());
    println!("Errores encontrados: {}", result.error_count);

    if !result.errors.is_empty() {
        println!("\nErrores:");
        for (i, error) in result.errors.iter().take(10).enumerate() {
            println!("  {}. {}", i + 1, error);
        }
        if result.errors.len() > 10 {
            println!("  ... y {} errores más", result.errors.len() - 10);
        }
    }

    // Mostrar detalles de las primeras manos
    if !result.hands.is_empty() {
        println!("\n=== Detalles de las Primeras 5 Manos ===\n");

        for (i, hand) in result.hands.iter().take(5).enumerate() {
            println!("Mano #{}", i + 1);
            println!("  Hand ID: {}", hand.hand_id);
            println!("  Fecha: {}", hand.timestamp);
            println!("  Mesa: {} ({:?})", hand.table_name, hand.game_type);
            println!(
                "  Blinds: {:.2}€/{:.2}€",
                hand.small_blind_cents as f64 / 100.0,
                hand.big_blind_cents as f64 / 100.0
            );
            println!("  Jugadores: {}", hand.players.len());
            println!("  Acciones: {}", hand.actions.len());
            println!("  Pot total: {:.2}€", hand.pot.total_cents as f64 / 100.0);
            println!("  Rake: {:.2}€", hand.pot.rake_cents as f64 / 100.0);

            // Buscar si thesmoy participó
            let hero_participated = hand.players.iter().any(|p| p.name == "thesmoy");

            if hero_participated {
                if let Some(hero_cards) = &hand.hero_cards {
                    let card1 = format!("{}{}", hero_cards[0].rank, hero_cards[0].suit);
                    let card2 = format!("{}{}", hero_cards[1].rank, hero_cards[1].suit);
                    println!("  Hero cards: {} {}", card1, card2);
                }
            }

            println!();
        }

        // Estadísticas generales
        println!("=== Estadísticas Generales ===\n");

        let total_actions: usize = result.hands.iter().map(|h| h.actions.len()).sum();

        let hands_with_hero: usize = result
            .hands
            .iter()
            .filter(|h| h.players.iter().any(|p| p.name == "thesmoy"))
            .count();

        let total_pot: i64 = result.hands.iter().map(|h| h.pot.total_cents).sum();

        let total_rake: i64 = result.hands.iter().map(|h| h.pot.rake_cents).sum();

        println!("Total de manos: {}", result.hands.len());
        println!("Manos con thesmoy: {}", hands_with_hero);
        println!("Total de acciones: {}", total_actions);
        println!("Pot total acumulado: {:.2}€", total_pot as f64 / 100.0);
        println!("Rake total acumulado: {:.2}€", total_rake as f64 / 100.0);
    } else {
        println!("\n⚠ No se parsearon manos. Revisa los errores arriba.");
    }

    println!("\n=== Prueba Completada ===");
}
