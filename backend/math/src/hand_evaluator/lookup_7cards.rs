//! # Perfect Hash Table para Evaluación de 7 Cartas
//!
//! Tabla pre-calculada de ~133 millones de entradas para evaluación O(1)
//! de manos de 7 cartas en Texas Hold'em.
//!
//! ## Algoritmo
//!
//! Usa indexación combinatoria (combinadic) para mapear cualquier combinación
//! de 7 cartas a un índice único en el rango [0, C(52,7)-1].
//!
//! ## Performance
//!
//! - Lookup: O(1), < 50ns
//! - Memoria: ~267MB (133,784,560 entradas * 2 bytes)
//! - Carga: < 5 segundos con memory mapping
//!
//! ## Uso
//!
//! ```rust,ignore
//! use poker_math::hand_evaluator::lookup_7cards::evaluate_7cards_lookup;
//!
//! let cards: [Card; 7] = [...];
//! let rank = evaluate_7cards_lookup(&cards);
//! ```

use super::cards::Card;
use super::evaluator::evaluate_7cards as evaluate_7cards_iterative;
use super::hand_rank::HandRank;
use memmap2::Mmap;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Número total de combinaciones C(52,7)
pub const TOTAL_7CARD_COMBOS: usize = 133_784_560;

/// Tamaño de la tabla en bytes (u16 por entrada)
pub const TABLE_SIZE_BYTES: usize = TOTAL_7CARD_COMBOS * 2;

/// Nombre del archivo de la tabla pre-calculada
pub const TABLE_FILENAME: &str = "lookup_7cards.bin";

/// Coeficientes binomiales pre-calculados C(n, k) para n <= 52, k <= 7
/// Usados para calcular el índice combinatorio rápidamente
static BINOMIAL: Lazy<[[u64; 8]; 53]> = Lazy::new(|| {
    let mut table = [[0u64; 8]; 53];
    for n in 0..=52 {
        table[n][0] = 1;
        for k in 1..=7.min(n) {
            table[n][k] = table[n - 1][k - 1] + table[n - 1][k];
        }
    }
    table
});

/// Tabla de lookup de 7 cartas cargada en memoria
/// Se inicializa lazy la primera vez que se accede
static LOOKUP_TABLE: Lazy<Option<LookupTable>> = Lazy::new(|| {
    // Intentar cargar desde el directorio de datos del crate
    let paths = [
        "data/lookup_7cards.bin",
        "backend/math/data/lookup_7cards.bin",
        "../math/data/lookup_7cards.bin",
    ];

    for path in &paths {
        if let Ok(table) = LookupTable::load(path) {
            return Some(table);
        }
    }

    // Si no existe, devolver None (se usará el método iterativo)
    None
});

/// Estructura que contiene la tabla mapeada en memoria
pub struct LookupTable {
    /// Memory-mapped file con los rankings
    _mmap: Mmap,
    /// Puntero a los datos como slice de u16
    data: &'static [u16],
}

// SAFETY: LookupTable es Send + Sync porque Mmap es Send + Sync
// y data es una referencia inmutable a memoria mapeada de solo lectura
unsafe impl Send for LookupTable {}
unsafe impl Sync for LookupTable {}

impl LookupTable {
    /// Carga la tabla desde un archivo binario usando memory mapping
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        // Verificar tamaño
        if mmap.len() != TABLE_SIZE_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Tamaño de tabla inválido: esperado {}, encontrado {}",
                    TABLE_SIZE_BYTES,
                    mmap.len()
                ),
            ));
        }

        // Convertir a slice de u16
        // SAFETY: El archivo fue generado con u16 little-endian alineados
        let data: &[u16] =
            unsafe { std::slice::from_raw_parts(mmap.as_ptr() as *const u16, TOTAL_7CARD_COMBOS) };

        // Extender lifetime a 'static para el lazy static
        // SAFETY: La tabla vive tanto como el programa
        let data: &'static [u16] = unsafe { std::mem::transmute(data) };

        Ok(LookupTable { _mmap: mmap, data })
    }

    /// Obtiene el ranking para un índice dado
    #[inline]
    pub fn get(&self, index: usize) -> Option<u16> {
        self.data.get(index).copied()
    }
}

/// Calcula el índice combinatorio para 7 cartas ordenadas
///
/// Dado un conjunto de 7 cartas con índices c0 < c1 < c2 < c3 < c4 < c5 < c6,
/// calcula el índice único usando la fórmula combinatoria:
///
/// index = C(c0,1) + C(c1,2) + C(c2,3) + C(c3,4) + C(c4,5) + C(c5,6) + C(c6,7)
///
/// donde C(n,k) es el coeficiente binomial.
#[inline]
pub fn cards_to_index(cards: &[Card; 7]) -> usize {
    // Extraer y ordenar los índices de las cartas
    let mut indices: [u8; 7] = [
        cards[0].index(),
        cards[1].index(),
        cards[2].index(),
        cards[3].index(),
        cards[4].index(),
        cards[5].index(),
        cards[6].index(),
    ];
    indices.sort_unstable();

    // Calcular índice combinatorio
    let binomial = &*BINOMIAL;
    let index = binomial[indices[0] as usize][1]
        + binomial[indices[1] as usize][2]
        + binomial[indices[2] as usize][3]
        + binomial[indices[3] as usize][4]
        + binomial[indices[4] as usize][5]
        + binomial[indices[5] as usize][6]
        + binomial[indices[6] as usize][7];

    index as usize
}

/// Convierte un índice combinatorio de vuelta a 7 cartas
///
/// Útil para el generador de la tabla.
#[inline]
pub fn index_to_cards(mut index: usize) -> [Card; 7] {
    let binomial = &*BINOMIAL;
    let mut cards = [Card::from_index(0).unwrap(); 7];

    // Decodificar en orden inverso (de k=7 a k=1)
    for k in (1..=7).rev() {
        // Encontrar el mayor n tal que C(n,k) <= index
        let mut n = k - 1;
        while n < 52 && binomial[n + 1][k] <= index as u64 {
            n += 1;
        }
        cards[k - 1] = Card::from_index(n as u8).unwrap();
        index -= binomial[n][k] as usize;
    }

    cards
}

/// Evalúa 7 cartas usando la tabla de lookup O(1)
///
/// Si la tabla no está cargada, cae al método iterativo.
#[inline]
pub fn evaluate_7cards_lookup(cards: &[Card; 7]) -> HandRank {
    if let Some(ref table) = *LOOKUP_TABLE {
        let index = cards_to_index(cards);
        if let Some(rank) = table.get(index) {
            return HandRank::new(rank);
        }
    }

    // Fallback al método iterativo si la tabla no está disponible
    evaluate_7cards_iterative(cards)
}

/// Verifica si la tabla de lookup está cargada
#[inline]
pub fn is_lookup_table_loaded() -> bool {
    LOOKUP_TABLE.is_some()
}

/// Genera la tabla completa de 7 cartas
///
/// Esta función es costosa (~2-5 minutos) y solo debe ejecutarse una vez
/// para generar el archivo binario.
///
/// # Arguments
/// * `output_path` - Ruta donde guardar el archivo .bin
///
/// # Returns
/// * `Ok(())` si la generación fue exitosa
/// * `Err` si hubo error de I/O
pub fn generate_lookup_table<P: AsRef<Path>>(output_path: P) -> std::io::Result<()> {
    use rayon::prelude::*;

    println!("Generando tabla de {} combinaciones...", TOTAL_7CARD_COMBOS);

    // Crear buffer para toda la tabla
    let mut rankings: Vec<u16> = vec![0u16; TOTAL_7CARD_COMBOS];

    // Procesar en paralelo con Rayon
    // Dividimos el trabajo en chunks para mejor balance de carga
    let chunk_size = 100_000;
    let num_chunks = (TOTAL_7CARD_COMBOS + chunk_size - 1) / chunk_size;

    rankings
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let start_idx = chunk_idx * chunk_size;

            for (i, rank) in chunk.iter_mut().enumerate() {
                let idx = start_idx + i;
                if idx >= TOTAL_7CARD_COMBOS {
                    break;
                }

                // Convertir índice a cartas
                let cards = index_to_cards(idx);

                // Evaluar la mano usando el método iterativo
                let hand_rank = evaluate_7cards_iterative(&cards);
                *rank = hand_rank.value();
            }

            // Progreso (solo cada 10 chunks para no saturar)
            if chunk_idx % 10 == 0 {
                let progress = (chunk_idx as f64 / num_chunks as f64) * 100.0;
                println!("Progreso: {:.1}%", progress);
            }
        });

    println!("Escribiendo archivo...");

    // Escribir a archivo
    let file = File::create(output_path)?;
    let mut writer = BufWriter::with_capacity(1024 * 1024 * 64, file); // 64MB buffer

    // Escribir como bytes (little-endian)
    for rank in &rankings {
        writer.write_all(&rank.to_le_bytes())?;
    }

    writer.flush()?;

    println!(
        "Tabla generada: {} bytes",
        TOTAL_7CARD_COMBOS * std::mem::size_of::<u16>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_coefficients() {
        let binomial = &*BINOMIAL;

        // C(52,7) = 133,784,560
        assert_eq!(binomial[52][7], TOTAL_7CARD_COMBOS as u64);

        // C(n,0) = 1
        for row in binomial.iter().take(53) {
            assert_eq!(row[0], 1);
        }

        // C(n,1) = n
        for (n, row) in binomial.iter().enumerate().take(53).skip(1) {
            assert_eq!(row[1], n as u64);
        }

        // C(7,7) = 1
        assert_eq!(binomial[7][7], 1);
    }

    #[test]
    fn test_index_roundtrip() {
        // Probar algunas combinaciones conocidas
        let test_indices = [
            0,
            1,
            100,
            1000,
            10000,
            100000,
            1000000,
            TOTAL_7CARD_COMBOS - 1,
        ];

        for &idx in &test_indices {
            let cards = index_to_cards(idx);
            let recovered_idx = cards_to_index(&cards);
            assert_eq!(idx, recovered_idx, "Roundtrip falló para índice {}", idx);
        }
    }

    #[test]
    fn test_index_ordering() {
        // Los índices de las cartas deben estar ordenados
        let cards = index_to_cards(12345);
        for i in 0..6 {
            assert!(
                cards[i].index() < cards[i + 1].index(),
                "Cartas no ordenadas en índice 12345"
            );
        }
    }

    #[test]
    fn test_first_and_last_combos() {
        // Primera combinación: 0,1,2,3,4,5,6 (las 7 cartas más bajas)
        let first = index_to_cards(0);
        assert_eq!(first[0].index(), 0);
        assert_eq!(first[6].index(), 6);

        // Última combinación: 45,46,47,48,49,50,51 (las 7 cartas más altas)
        let last = index_to_cards(TOTAL_7CARD_COMBOS - 1);
        assert_eq!(last[0].index(), 45);
        assert_eq!(last[6].index(), 51);
    }

    #[test]
    fn test_evaluate_7cards_lookup_fallback() {
        // Sin tabla cargada, debe usar el método iterativo
        let cards: [Card; 7] = [
            "As".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Js".parse().unwrap(),
            "Ts".parse().unwrap(),
            "2h".parse().unwrap(),
            "3d".parse().unwrap(),
        ];

        let rank = evaluate_7cards_lookup(&cards);
        assert!(rank.is_straight_flush() || rank.is_royal_flush());
    }

    #[test]
    fn test_cards_to_index_consistency() {
        // Probar que diferentes ordenaciones de las mismas cartas dan el mismo índice
        let cards1: [Card; 7] = [
            "As".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Js".parse().unwrap(),
            "Ts".parse().unwrap(),
            "2h".parse().unwrap(),
            "3d".parse().unwrap(),
        ];

        let cards2: [Card; 7] = [
            "3d".parse().unwrap(),
            "2h".parse().unwrap(),
            "Ts".parse().unwrap(),
            "Js".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Ks".parse().unwrap(),
            "As".parse().unwrap(),
        ];

        assert_eq!(cards_to_index(&cards1), cards_to_index(&cards2));
    }

    #[test]
    fn test_lookup_known_hands() {
        // Verificar manos conocidas con el lookup
        // Solo ejecutar si la tabla está cargada
        if !is_lookup_table_loaded() {
            println!("SKIP: Tabla de lookup no cargada");
            return;
        }

        // Royal Flush
        let royal_flush: [Card; 7] = [
            "As".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Js".parse().unwrap(),
            "Ts".parse().unwrap(),
            "2h".parse().unwrap(),
            "3d".parse().unwrap(),
        ];
        let rank = evaluate_7cards_lookup(&royal_flush);
        assert!(
            rank.is_royal_flush(),
            "Expected Royal Flush, got {:?}",
            rank
        );

        // Straight Flush
        let straight_flush: [Card; 7] = [
            "9h".parse().unwrap(),
            "8h".parse().unwrap(),
            "7h".parse().unwrap(),
            "6h".parse().unwrap(),
            "5h".parse().unwrap(),
            "Ac".parse().unwrap(),
            "Kd".parse().unwrap(),
        ];
        let rank = evaluate_7cards_lookup(&straight_flush);
        assert!(
            rank.is_straight_flush(),
            "Expected Straight Flush, got {:?}",
            rank
        );

        // Full House
        let full_house: [Card; 7] = [
            "Kh".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Kd".parse().unwrap(),
            "Qc".parse().unwrap(),
            "Qh".parse().unwrap(),
            "2c".parse().unwrap(),
            "3d".parse().unwrap(),
        ];
        let rank = evaluate_7cards_lookup(&full_house);
        assert!(rank.is_full_house(), "Expected Full House, got {:?}", rank);

        // Flush
        let flush: [Card; 7] = [
            "Ah".parse().unwrap(),
            "Kh".parse().unwrap(),
            "Jh".parse().unwrap(),
            "9h".parse().unwrap(),
            "5h".parse().unwrap(),
            "2c".parse().unwrap(),
            "3d".parse().unwrap(),
        ];
        let rank = evaluate_7cards_lookup(&flush);
        assert!(rank.is_flush(), "Expected Flush, got {:?}", rank);
    }

    #[test]
    fn test_lookup_sample_consistency() {
        // Verificar que los primeros 100 índices producen manos válidas
        // Solo ejecutar si la tabla está cargada
        if !is_lookup_table_loaded() {
            println!("SKIP: Tabla de lookup no cargada");
            return;
        }

        for idx in 0..100 {
            let cards = index_to_cards(idx);
            let rank = evaluate_7cards_lookup(&cards);

            // Verificar que el ranking está en rango válido (1-7462)
            assert!(
                rank.value() >= 1 && rank.value() <= 7462,
                "Ranking inválido {} para índice {}",
                rank.value(),
                idx
            );
        }
    }
}
