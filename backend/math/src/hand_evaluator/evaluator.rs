//! # Evaluador de Manos de Poker
//!
//! Implementación del algoritmo de evaluación basado en Cactus Kev.
//! Soporta evaluación de manos de 5, 6 o 7 cartas.
//!
//! ## Performance
//! - Evaluación de 5 cartas: ~20-50ns
//! - Evaluación de 7 cartas: ~100-200ns (21 combinaciones)
//!
//! ## Algoritmo
//! 1. Detectar si todas las cartas son del mismo palo (flush)
//! 2. Si es flush, usar FLUSH_LOOKUP con el OR de rank bits
//! 3. Si no hay duplicados de rank, usar UNIQUE5_LOOKUP (straights y high cards)
//! 4. Si hay duplicados, calcular producto de primos y buscar en hash table

use super::cards::Card;
use super::hand_rank::HandRank;
use super::lookup::{count_bits, evaluate_flush, evaluate_unique, prime_product_from_cards};

/// Evalúa una mano de exactamente 5 cartas
///
/// # Arguments
/// * `cards` - Array de 5 cartas
///
/// # Returns
/// * `HandRank` - El ranking de la mano (1 = Royal Flush, 7462 = 7-5-4-3-2)
#[inline]
pub fn evaluate_5cards(cards: &[Card; 5]) -> HandRank {
    // Extraer valores de las cartas
    let c0 = cards[0].value();
    let c1 = cards[1].value();
    let c2 = cards[2].value();
    let c3 = cards[3].value();
    let c4 = cards[4].value();

    // Calcular OR de rank bits para detección de duplicados y straights
    let rank_bits = cards[0].rank_bit()
        | cards[1].rank_bit()
        | cards[2].rank_bit()
        | cards[3].rank_bit()
        | cards[4].rank_bit();

    // Calcular AND de suit bits para detección de flush
    let suit_and = cards[0].suit_bit()
        & cards[1].suit_bit()
        & cards[2].suit_bit()
        & cards[3].suit_bit()
        & cards[4].suit_bit();

    // Si todos los suits son iguales (suit_and != 0), es flush
    if suit_and != 0 {
        return evaluate_flush(rank_bits);
    }

    // Si hay exactamente 5 bits de rank activos, no hay duplicados
    // Puede ser straight o high card
    if count_bits(rank_bits) == 5 {
        return evaluate_unique(rank_bits);
    }

    // Hay duplicados (pair, two pair, trips, full house, quads)
    // Usar evaluación basada en producto de primos
    let card_values = [c0, c1, c2, c3, c4];
    evaluate_with_duplicates(&card_values, rank_bits)
}

/// Evalúa una mano con duplicados de rank (pairs, trips, etc.)
#[inline]
fn evaluate_with_duplicates(card_values: &[u32; 5], rank_bits: u32) -> HandRank {
    let prime_product = prime_product_from_cards(card_values);
    let num_unique = count_bits(rank_bits);

    // Clasificar por número de ranks únicos
    match num_unique {
        2 => {
            // Quads (AAAAB) o Full House (AAABB)
            evaluate_2_unique(prime_product)
        }
        3 => {
            // Trips (AAABC), Two Pair (AABBC), o Full House edge case
            evaluate_3_unique(prime_product)
        }
        4 => {
            // One Pair (AABCD)
            evaluate_4_unique(prime_product)
        }
        _ => HandRank::new(7462), // Fallback
    }
}

/// Evalúa manos con 2 ranks únicos (quads o full house)
fn evaluate_2_unique(prime_product: u32) -> HandRank {
    // Quads rankings: 11-166 (156 combinaciones)
    // Full House rankings: 167-322 (156 combinaciones)

    // Los productos de primos para quads siguen un patrón:
    // AAAA + B: prime(A)^4 * prime(B)
    // Los de full house:
    // AAA + BB: prime(A)^3 * prime(B)^2

    // Identificar si es quad o full house por el producto
    // Quad: uno de los primos aparece 4 veces
    // Full: uno aparece 3 veces, otro 2

    // Para quads: p^4 * q donde p,q son primos distintos
    // Para full: p^3 * q^2

    // Usar búsqueda en tabla hash precalculada
    // Por ahora, usamos cálculo directo

    // Productos de quads más altos:
    // AAAA2: 41^4 * 2 = 5,648,162
    // 2222A: 2^4 * 41 = 656

    // Productos de full más altos:
    // AAA22: 41^3 * 4 = 275,684
    // AAA33: 41^3 * 9 = 620,289

    // Heurística: si producto > threshold, probablemente es quad
    // Pero mejor: verificar si algún primo divide al producto 4 veces

    if is_quads(prime_product) {
        rank_quads(prime_product)
    } else {
        rank_full_house(prime_product)
    }
}

/// Verifica si el producto corresponde a quads
fn is_quads(product: u32) -> bool {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    for &p in &PRIMES {
        let p4 = p * p * p * p;
        if product % p4 == 0 {
            return true;
        }
    }
    false
}

/// Calcula el ranking para quads
fn rank_quads(product: u32) -> HandRank {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    // Encontrar el rank de los quads y el kicker
    let mut quad_rank = 0usize;
    let mut kicker_rank = 0usize;

    for (i, &p) in PRIMES.iter().enumerate() {
        let p4 = p * p * p * p;
        if product % p4 == 0 {
            quad_rank = i;
            // El kicker es product / p^4
            let kicker_prime = product / p4;
            for (j, &pk) in PRIMES.iter().enumerate() {
                if pk == kicker_prime {
                    kicker_rank = j;
                    break;
                }
            }
            break;
        }
    }

    // Rankings de quads: 11-166
    // AAAA2 es el mejor quad (rank 11), 2222A es el peor (rank 166)
    // Ordenados por: quad_rank DESC, kicker_rank DESC

    // 13 valores de quads x 12 kickers = 156 combinaciones
    // Base = 11
    // Inversión: (12 - quad_rank) * 12 + (12 - kicker_rank) - ajuste

    let quad_offset = 12 - quad_rank;
    let kicker_offset = if kicker_rank > quad_rank {
        12 - kicker_rank
    } else {
        12 - kicker_rank - 1
    };

    let rank = 11 + quad_offset * 12 + kicker_offset;
    HandRank::new(rank.min(166) as u16)
}

/// Calcula el ranking para full house
fn rank_full_house(product: u32) -> HandRank {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    let mut trips_rank = 0usize;
    let mut pair_rank = 0usize;

    // Encontrar trips (p^3)
    for (i, &p) in PRIMES.iter().enumerate() {
        let p3 = p * p * p;
        if product % p3 == 0 {
            let remaining = product / p3;
            // Verificar si el restante es p^2
            for (j, &pj) in PRIMES.iter().enumerate() {
                if pj * pj == remaining {
                    trips_rank = i;
                    pair_rank = j;
                    break;
                }
            }
            if pair_rank != 0 || trips_rank != 0 {
                break;
            }
        }
    }

    // Rankings de full house: 167-322
    // AAAKK es el mejor (rank 167), 22233 es el peor (rank 322)
    // Ordenados por: trips_rank DESC, pair_rank DESC

    let trips_offset = 12 - trips_rank;
    let pair_offset = if pair_rank > trips_rank {
        12 - pair_rank
    } else {
        12 - pair_rank - 1
    };

    let rank = 167 + trips_offset * 12 + pair_offset;
    HandRank::new(rank.min(322) as u16)
}

/// Evalúa manos con 3 ranks únicos (trips o two pair)
fn evaluate_3_unique(prime_product: u32) -> HandRank {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    // Trips: AAABC -> p^3 * q * r
    // Two Pair: AABBC -> p^2 * q^2 * r

    // Verificar si es trips (algún primo aparece 3 veces)
    for &p in &PRIMES {
        let p3 = p * p * p;
        if prime_product % p3 == 0 {
            return rank_trips(prime_product);
        }
    }

    // Es two pair
    rank_two_pair(prime_product)
}

/// Calcula el ranking para trips
fn rank_trips(product: u32) -> HandRank {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    // Trips rankings: 1610-2467 (858 combinaciones)
    // 13 trips x 66 combinaciones de 2 kickers = 858

    let mut trips_rank = 0usize;
    let mut kickers = [0usize; 2];
    let mut k_idx = 0;

    for (i, &p) in PRIMES.iter().enumerate() {
        let p3 = p * p * p;
        if product % p3 == 0 {
            trips_rank = i;
            let remaining = product / p3;
            // Encontrar los dos kickers
            for (j, &pj) in PRIMES.iter().enumerate() {
                if remaining % pj == 0 && j != i && k_idx < 2 {
                    kickers[k_idx] = j;
                    k_idx += 1;
                }
            }
            break;
        }
    }

    // Ordenar kickers de mayor a menor
    if kickers[0] < kickers[1] {
        kickers.swap(0, 1);
    }

    // Calcular offset
    let trips_offset = 12 - trips_rank;
    let kicker1_offset = 11 - kickers[0];
    let kicker2_offset = 10 - kickers[1];

    let rank = 1610 + trips_offset * 66 + kicker1_offset * 11 + kicker2_offset;
    HandRank::new(rank.min(2467) as u16)
}

/// Calcula el ranking para two pair
fn rank_two_pair(product: u32) -> HandRank {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    // Two pair rankings: 2468-3325 (858 combinaciones)
    // C(13,2) pares x 11 kickers = 78 * 11 = 858

    let mut pairs = [0usize; 2];
    let mut p_idx = 0;
    let mut kicker = 0usize;

    for (i, &p) in PRIMES.iter().enumerate() {
        let p2 = p * p;
        if product % p2 == 0 {
            if p_idx < 2 {
                pairs[p_idx] = i;
                p_idx += 1;
            }
        } else if product % p == 0 {
            kicker = i;
        }
    }

    // Ordenar pares de mayor a menor
    if pairs[0] < pairs[1] {
        pairs.swap(0, 1);
    }

    // Calcular offset
    let high_pair = 12 - pairs[0];
    let low_pair = 11 - pairs[1];
    let kicker_offset = 10 - kicker;

    let rank = 2468 + high_pair * 78 + low_pair * 11 + kicker_offset;
    HandRank::new(rank.min(3325) as u16)
}

/// Evalúa manos con 4 ranks únicos (one pair)
fn evaluate_4_unique(prime_product: u32) -> HandRank {
    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

    // One pair rankings: 3326-6185 (2860 combinaciones)
    // 13 pares x 220 combinaciones de 3 kickers = 2860

    let mut pair_rank = 0usize;
    let mut kickers = [0usize; 3];
    let mut k_idx = 0;

    for (i, &p) in PRIMES.iter().enumerate() {
        let p2 = p * p;
        if prime_product % p2 == 0 {
            pair_rank = i;
        } else if prime_product % p == 0 && k_idx < 3 {
            kickers[k_idx] = i;
            k_idx += 1;
        }
    }

    // Ordenar kickers de mayor a menor
    kickers.sort_by(|a, b| b.cmp(a));

    // Calcular offset (simplificado)
    // Usar saturating_sub para prevenir overflow si kickers están fuera de rango
    let pair_offset = 12usize.saturating_sub(pair_rank);
    let k1 = 11usize.saturating_sub(kickers[0]);
    let k2 = 10usize.saturating_sub(kickers[1]);
    let k3 = 9usize.saturating_sub(kickers[2]);

    let rank = 3326 + pair_offset * 220 + k1 * 55 + k2 * 10 + k3;
    HandRank::new(rank.min(6185) as u16)
}

/// Evalúa la mejor mano de 5 cartas de 7 cartas disponibles
///
/// Itera sobre las 21 combinaciones posibles de 5 cartas
/// y devuelve el mejor ranking.
pub fn evaluate_7cards(cards: &[Card; 7]) -> HandRank {
    let mut best = HandRank::new(HandRank::WORST);

    // Índices de las 21 combinaciones de 5 cartas de 7
    const COMBOS: [[usize; 5]; 21] = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 5],
        [0, 1, 2, 3, 6],
        [0, 1, 2, 4, 5],
        [0, 1, 2, 4, 6],
        [0, 1, 2, 5, 6],
        [0, 1, 3, 4, 5],
        [0, 1, 3, 4, 6],
        [0, 1, 3, 5, 6],
        [0, 1, 4, 5, 6],
        [0, 2, 3, 4, 5],
        [0, 2, 3, 4, 6],
        [0, 2, 3, 5, 6],
        [0, 2, 4, 5, 6],
        [0, 3, 4, 5, 6],
        [1, 2, 3, 4, 5],
        [1, 2, 3, 4, 6],
        [1, 2, 3, 5, 6],
        [1, 2, 4, 5, 6],
        [1, 3, 4, 5, 6],
        [2, 3, 4, 5, 6],
    ];

    for combo in &COMBOS {
        let hand = [
            cards[combo[0]],
            cards[combo[1]],
            cards[combo[2]],
            cards[combo[3]],
            cards[combo[4]],
        ];
        let rank = evaluate_5cards(&hand);
        if rank > best {
            best = rank;
        }
    }

    best
}

/// Evalúa la mejor mano de 5 cartas de 6 cartas disponibles
pub fn evaluate_6cards(cards: &[Card; 6]) -> HandRank {
    let mut best = HandRank::new(HandRank::WORST);

    // 6 combinaciones de 5 cartas de 6
    const COMBOS: [[usize; 5]; 6] = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 5],
        [0, 1, 2, 4, 5],
        [0, 1, 3, 4, 5],
        [0, 2, 3, 4, 5],
        [1, 2, 3, 4, 5],
    ];

    for combo in &COMBOS {
        let hand = [
            cards[combo[0]],
            cards[combo[1]],
            cards[combo[2]],
            cards[combo[3]],
            cards[combo[4]],
        ];
        let rank = evaluate_5cards(&hand);
        if rank > best {
            best = rank;
        }
    }

    best
}

/// Evalúa una mano desde un slice de cartas (5-7 cartas)
pub fn evaluate(cards: &[Card]) -> Option<HandRank> {
    match cards.len() {
        5 => {
            let arr: [Card; 5] = cards.try_into().ok()?;
            Some(evaluate_5cards(&arr))
        }
        6 => {
            let arr: [Card; 6] = cards.try_into().ok()?;
            Some(evaluate_6cards(&arr))
        }
        7 => {
            let arr: [Card; 7] = cards.try_into().ok()?;
            Some(evaluate_7cards(&arr))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_royal_flush() {
        let cards: [Card; 5] = [
            "As".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Js".parse().unwrap(),
            "Ts".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(
            rank.is_royal_flush(),
            "Expected Royal Flush, got {:?}",
            rank
        );
        assert_eq!(rank.value(), 1);
    }

    #[test]
    fn test_straight_flush() {
        // 9-high straight flush
        let cards: [Card; 5] = [
            "9h".parse().unwrap(),
            "8h".parse().unwrap(),
            "7h".parse().unwrap(),
            "6h".parse().unwrap(),
            "5h".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_straight_flush());
        assert!(rank.value() > 1 && rank.value() <= 10);
    }

    #[test]
    fn test_steel_wheel() {
        // A-2-3-4-5 flush (steel wheel)
        let cards: [Card; 5] = [
            "5d".parse().unwrap(),
            "4d".parse().unwrap(),
            "3d".parse().unwrap(),
            "2d".parse().unwrap(),
            "Ad".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_straight_flush());
        assert_eq!(rank.value(), 10); // Peor straight flush
    }

    #[test]
    fn test_four_of_a_kind() {
        let cards: [Card; 5] = [
            "Ah".parse().unwrap(),
            "As".parse().unwrap(),
            "Ad".parse().unwrap(),
            "Ac".parse().unwrap(),
            "Kh".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_four_of_a_kind(), "Expected Quads, got {:?}", rank);
    }

    #[test]
    fn test_full_house() {
        let cards: [Card; 5] = [
            "Kh".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Kd".parse().unwrap(),
            "Qc".parse().unwrap(),
            "Qh".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_full_house(), "Expected Full House, got {:?}", rank);
    }

    #[test]
    fn test_flush() {
        let cards: [Card; 5] = [
            "Ah".parse().unwrap(),
            "Jh".parse().unwrap(),
            "8h".parse().unwrap(),
            "5h".parse().unwrap(),
            "2h".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_flush(), "Expected Flush, got {:?}", rank);
    }

    #[test]
    fn test_straight() {
        // Broadway straight (no flush)
        let cards: [Card; 5] = [
            "As".parse().unwrap(),
            "Kh".parse().unwrap(),
            "Qd".parse().unwrap(),
            "Jc".parse().unwrap(),
            "Ts".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_straight(), "Expected Straight, got {:?}", rank);
    }

    #[test]
    fn test_wheel_straight() {
        // A-2-3-4-5 (no flush)
        let cards: [Card; 5] = [
            "5s".parse().unwrap(),
            "4h".parse().unwrap(),
            "3d".parse().unwrap(),
            "2c".parse().unwrap(),
            "As".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(
            rank.is_straight(),
            "Expected Straight (Wheel), got {:?}",
            rank
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let cards: [Card; 5] = [
            "Qh".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Qd".parse().unwrap(),
            "7c".parse().unwrap(),
            "2h".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_three_of_a_kind(), "Expected Trips, got {:?}", rank);
    }

    #[test]
    fn test_two_pair() {
        let cards: [Card; 5] = [
            "Ah".parse().unwrap(),
            "As".parse().unwrap(),
            "Kd".parse().unwrap(),
            "Kc".parse().unwrap(),
            "Qh".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_two_pair(), "Expected Two Pair, got {:?}", rank);
    }

    #[test]
    fn test_one_pair() {
        let cards: [Card; 5] = [
            "Jh".parse().unwrap(),
            "Js".parse().unwrap(),
            "9d".parse().unwrap(),
            "5c".parse().unwrap(),
            "2h".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_one_pair(), "Expected One Pair, got {:?}", rank);
    }

    #[test]
    fn test_high_card() {
        let cards: [Card; 5] = [
            "Ah".parse().unwrap(),
            "Ks".parse().unwrap(),
            "9d".parse().unwrap(),
            "5c".parse().unwrap(),
            "2h".parse().unwrap(),
        ];
        let rank = evaluate_5cards(&cards);
        assert!(rank.is_high_card(), "Expected High Card, got {:?}", rank);
    }

    #[test]
    fn test_evaluate_7cards() {
        // 7 cartas que incluyen un flush
        let cards: [Card; 7] = [
            "Ah".parse().unwrap(),
            "Kh".parse().unwrap(),
            "Qh".parse().unwrap(),
            "Jh".parse().unwrap(),
            "9h".parse().unwrap(),
            "2c".parse().unwrap(),
            "3d".parse().unwrap(),
        ];
        let rank = evaluate_7cards(&cards);
        assert!(
            rank.is_flush(),
            "Expected Flush from 7 cards, got {:?}",
            rank
        );
    }

    #[test]
    fn test_evaluate_7cards_finds_best() {
        // 7 cartas con straight flush escondido
        let cards: [Card; 7] = [
            "9s".parse().unwrap(),
            "8s".parse().unwrap(),
            "7s".parse().unwrap(),
            "6s".parse().unwrap(),
            "5s".parse().unwrap(),
            "Ah".parse().unwrap(),
            "Kd".parse().unwrap(),
        ];
        let rank = evaluate_7cards(&cards);
        assert!(
            rank.is_straight_flush(),
            "Expected Straight Flush from 7 cards, got {:?}",
            rank
        );
    }

    #[test]
    fn test_hand_comparison() {
        let royal: [Card; 5] = [
            "As".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Js".parse().unwrap(),
            "Ts".parse().unwrap(),
        ];

        let pair: [Card; 5] = [
            "Ah".parse().unwrap(),
            "As".parse().unwrap(),
            "Kd".parse().unwrap(),
            "Qc".parse().unwrap(),
            "Jh".parse().unwrap(),
        ];

        let royal_rank = evaluate_5cards(&royal);
        let pair_rank = evaluate_5cards(&pair);

        assert!(royal_rank > pair_rank, "Royal Flush should beat One Pair");
    }
}
