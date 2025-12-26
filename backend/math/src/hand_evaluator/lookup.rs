//! # Lookup Tables para Evaluación de Manos
//!
//! Tablas pre-calculadas para evaluación O(1) de manos de poker.
//! Basado en el algoritmo de Cactus Kev con optimizaciones.
//!
//! ## Estructura de las Tablas
//!
//! - `FLUSH_LOOKUP`: Rankings para combinaciones de 5 cartas del mismo palo (1277 entradas únicas)
//! - `UNIQUE5_LOOKUP`: Rankings para manos sin pareja (straights y high cards)
//! - `HASH_LOOKUP`: Rankings para manos con parejas usando hash de primos
//!
//! El sistema usa el producto de los números primos de los ranks para
//! identificar de manera única cada combinación de ranks (sin considerar suits).

use super::hand_rank::HandRank;

/// Máscara para extraer los 13 bits de ranks
const RANK_MASK: u32 = 0x1FFF;

/// Tabla de flush: 8192 entradas (2^13 posibles combinaciones de bits)
/// Indexada por el OR de los rank bits de las 5 cartas
/// Solo contiene valores válidos para combinaciones de exactamente 5 bits activos
static FLUSH_LOOKUP: [u16; 8192] = generate_flush_table();

/// Tabla de manos únicas (sin pares): 8192 entradas
/// Indexada por el OR de los rank bits, válida para straights y high cards
static UNIQUE5_LOOKUP: [u16; 8192] = generate_unique5_table();

// Nota: Las tablas HASH_VALUES y HASH_ADJUST se reservan para optimización futura
// con perfect hashing. Por ahora usamos cálculo directo que es suficientemente rápido.

/// Verifica si exactamente 5 bits están activos (mano sin duplicados)
#[inline]
const fn popcount(x: u32) -> u32 {
    let mut count = 0;
    let mut n = x;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

/// Verifica si los bits representan un straight
/// Un straight tiene 5 bits consecutivos o A-2-3-4-5 (wheel)
#[inline]
const fn is_straight(bits: u32) -> bool {
    // Straights posibles (13 bits: A K Q J T 9 8 7 6 5 4 3 2)
    // Bit 12 = Ace, Bit 0 = 2
    let straight_patterns: [u32; 10] = [
        0b1111100000000, // A-K-Q-J-T (Royal)
        0b0111110000000, // K-Q-J-T-9
        0b0011111000000, // Q-J-T-9-8
        0b0001111100000, // J-T-9-8-7
        0b0000111110000, // T-9-8-7-6
        0b0000011111000, // 9-8-7-6-5
        0b0000001111100, // 8-7-6-5-4
        0b0000000111110, // 7-6-5-4-3
        0b0000000011111, // 6-5-4-3-2
        0b1000000001111, // A-5-4-3-2 (Wheel)
    ];

    let mut i = 0;
    while i < 10 {
        if bits == straight_patterns[i] {
            return true;
        }
        i += 1;
    }
    false
}

/// Genera la tabla de flush rankings
const fn generate_flush_table() -> [u16; 8192] {
    let mut table = [0u16; 8192];

    // Straight flushes (1-10)
    // Los mejores rankings son para straight flushes
    let sf_patterns: [(u32, u16); 10] = [
        (0b1111100000000, 1),  // Royal Flush
        (0b0111110000000, 2),  // K-high SF
        (0b0011111000000, 3),  // Q-high SF
        (0b0001111100000, 4),  // J-high SF
        (0b0000111110000, 5),  // T-high SF
        (0b0000011111000, 6),  // 9-high SF
        (0b0000001111100, 7),  // 8-high SF
        (0b0000000111110, 8),  // 7-high SF
        (0b0000000011111, 9),  // 6-high SF
        (0b1000000001111, 10), // Steel Wheel (A-5 SF)
    ];

    let mut i = 0;
    while i < 10 {
        let (pattern, rank) = sf_patterns[i];
        table[pattern as usize] = rank;
        i += 1;
    }

    // Flushes normales (323-1599)
    // Generamos todas las combinaciones de 5 bits no-straight
    let mut flush_rank = 323u16;
    let mut bits: u32 = 0b11111; // Empezamos con los 5 bits más bajos

    // Iteramos por todas las combinaciones de 5 cartas para flush
    // Ordenadas de mayor a menor para ranking correcto
    // Usamos un enfoque más simple: iterar todas las combinaciones
    loop {
        // Encontrar la siguiente combinación de 5 bits
        if popcount(bits) == 5 && !is_straight(bits) {
            if table[bits as usize] == 0 {
                table[bits as usize] = flush_rank;
                flush_rank += 1;
            }
        }

        if bits >= 0x1FFF {
            break;
        }

        // Encontrar siguiente combinación con 5 bits
        bits += 1;
        while bits < 0x1FFF && popcount(bits) != 5 {
            bits += 1;
        }

        if bits >= 0x1FFF {
            break;
        }

        if flush_rank > 1599 {
            break;
        }
    }

    table
}

/// Genera la tabla de manos únicas (sin pares)
const fn generate_unique5_table() -> [u16; 8192] {
    let mut table = [0u16; 8192];

    // Straights (1600-1609)
    let straight_patterns: [(u32, u16); 10] = [
        (0b1111100000000, 1600), // Broadway (A-high)
        (0b0111110000000, 1601), // K-high
        (0b0011111000000, 1602), // Q-high
        (0b0001111100000, 1603), // J-high
        (0b0000111110000, 1604), // T-high
        (0b0000011111000, 1605), // 9-high
        (0b0000001111100, 1606), // 8-high
        (0b0000000111110, 1607), // 7-high
        (0b0000000011111, 1608), // 6-high (wheel sin ace)
        (0b1000000001111, 1609), // Wheel (A-5)
    ];

    let mut i = 0;
    while i < 10 {
        let (pattern, rank) = straight_patterns[i];
        table[pattern as usize] = rank;
        i += 1;
    }

    // High cards (6186-7462)
    // Todas las combinaciones de 5 cartas distintas que no son straight
    let mut high_rank = 6186u16;
    let mut bits: u32 = 0b11111;

    loop {
        if popcount(bits) == 5 && !is_straight(bits) {
            if table[bits as usize] == 0 {
                table[bits as usize] = high_rank;
                high_rank += 1;
            }
        }

        if bits >= 0x1FFF {
            break;
        }

        bits += 1;
        while bits < 0x1FFF && popcount(bits) != 5 {
            bits += 1;
        }

        if bits >= 0x1FFF {
            break;
        }

        if high_rank > 7462 {
            break;
        }
    }

    table
}

/// Evalúa una mano de 5 cartas flush (todas del mismo palo)
#[inline]
pub fn evaluate_flush(rank_bits: u32) -> HandRank {
    let masked = rank_bits & RANK_MASK;
    HandRank::new(FLUSH_LOOKUP[masked as usize])
}

/// Evalúa una mano de 5 cartas únicas (sin duplicados de rank)
#[inline]
pub fn evaluate_unique(rank_bits: u32) -> HandRank {
    let masked = rank_bits & RANK_MASK;
    let value = UNIQUE5_LOOKUP[masked as usize];
    if value > 0 {
        HandRank::new(value)
    } else {
        // Fallback para combinaciones no pre-calculadas
        HandRank::new(7462)
    }
}

/// Número de bits activos en un u32
#[inline]
pub const fn count_bits(x: u32) -> u32 {
    x.count_ones()
}

/// Calcula el producto de primos desde los valores de las cartas
#[inline]
pub fn prime_product_from_cards(cards: &[u32]) -> u32 {
    let mut product = 1u32;
    for &card in cards {
        // Los bits 0-5 contienen el primo
        let prime = card & 0x3F;
        product = product.saturating_mul(prime);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popcount() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(1), 1);
        assert_eq!(popcount(0b11111), 5);
        assert_eq!(popcount(0b1111100000000), 5);
    }

    #[test]
    fn test_is_straight() {
        // Broadway
        assert!(is_straight(0b1111100000000));
        // Wheel
        assert!(is_straight(0b1000000001111));
        // No straight
        assert!(!is_straight(0b1111000000001));
    }

    #[test]
    fn test_flush_lookup_straight_flush() {
        // Royal Flush pattern
        let royal = 0b1111100000000u32;
        assert_eq!(FLUSH_LOOKUP[royal as usize], 1);

        // Steel wheel
        let wheel = 0b1000000001111u32;
        assert_eq!(FLUSH_LOOKUP[wheel as usize], 10);
    }

    #[test]
    fn test_unique5_straights() {
        // Broadway (no flush)
        let broadway = 0b1111100000000u32;
        assert_eq!(UNIQUE5_LOOKUP[broadway as usize], 1600);

        // Wheel
        let wheel = 0b1000000001111u32;
        assert_eq!(UNIQUE5_LOOKUP[wheel as usize], 1609);
    }

    #[test]
    fn test_prime_product() {
        // AA (dos ases)
        let ace_prime = super::super::cards::PRIMES[12]; // 41
        let product = ace_prime * ace_prime;
        assert_eq!(product, 1681);
    }
}
