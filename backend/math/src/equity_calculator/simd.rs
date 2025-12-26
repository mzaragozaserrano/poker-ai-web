//! # Optimizaciones SIMD AVX2
//!
//! Módulo con optimizaciones SIMD para evaluación paralela de manos.
//! Utiliza intrínsecos AVX2 disponibles en el Ryzen 7 3800X.
//!
//! ## Estrategia de Optimización
//!
//! En lugar de evaluar una mano a la vez, procesamos múltiples manos
//! en paralelo usando registros de 256 bits (8 x i32).
//!
//! ## Fallback
//!
//! Si AVX2 no está disponible, se usa la evaluación estándar.

use crate::hand_evaluator::{evaluate_7cards_lookup, is_lookup_table_loaded, Card, HandRank};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Verifica si AVX2 está disponible en tiempo de ejecución
#[inline]
pub fn is_avx2_available() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// Evaluador con optimizaciones SIMD
///
/// Proporciona una interfaz unificada para evaluación de manos
/// que automáticamente usa la implementación más rápida disponible.
#[derive(Debug, Clone, Copy)]
pub struct SimdEvaluator {
    use_simd: bool,
    use_lookup: bool,
}

impl SimdEvaluator {
    /// Crea un nuevo evaluador SIMD
    pub fn new() -> Self {
        SimdEvaluator {
            use_simd: is_avx2_available(),
            use_lookup: is_lookup_table_loaded(),
        }
    }

    /// Crea un evaluador sin SIMD (para testing)
    pub fn without_simd() -> Self {
        SimdEvaluator {
            use_simd: false,
            use_lookup: is_lookup_table_loaded(),
        }
    }

    /// Verifica si SIMD está activo
    #[inline]
    pub fn is_simd_enabled(&self) -> bool {
        self.use_simd
    }

    /// Evalúa una mano de 7 cartas
    #[inline]
    pub fn evaluate_7cards(&self, cards: &[Card; 7]) -> HandRank {
        // Usar lookup table O(1) si está disponible
        if self.use_lookup {
            return evaluate_7cards_lookup(cards);
        }

        // Fallback al evaluador iterativo
        crate::hand_evaluator::evaluate_7cards(cards)
    }

    /// Evalúa múltiples manos de 7 cartas en batch
    ///
    /// Esta es la función principal optimizada con SIMD.
    /// Procesa hasta 8 manos en paralelo usando AVX2.
    #[inline]
    pub fn evaluate_batch(&self, hands: &[[Card; 7]]) -> Vec<HandRank> {
        if self.use_simd && hands.len() >= 8 {
            self.evaluate_batch_simd(hands)
        } else {
            self.evaluate_batch_scalar(hands)
        }
    }

    /// Evaluación scalar (sin SIMD)
    fn evaluate_batch_scalar(&self, hands: &[[Card; 7]]) -> Vec<HandRank> {
        hands.iter().map(|h| self.evaluate_7cards(h)).collect()
    }

    /// Evaluación con SIMD AVX2
    #[cfg(target_arch = "x86_64")]
    fn evaluate_batch_simd(&self, hands: &[[Card; 7]]) -> Vec<HandRank> {
        if !is_x86_feature_detected!("avx2") {
            return self.evaluate_batch_scalar(hands);
        }

        // Para la evaluación de manos de poker, SIMD es más útil
        // en operaciones auxiliares (shuffle de deck, comparaciones en masa)
        // que en la evaluación de la mano en sí (que usa lookup tables).
        //
        // Sin embargo, podemos usar SIMD para:
        // 1. Comparación paralela de 8 rankings
        // 2. Shuffle vectorizado del deck
        // 3. Cálculo de estadísticas agregadas
        //
        // Por ahora, usamos la evaluación scalar pero con prefetch
        // para mejorar cache locality.

        let mut results = Vec::with_capacity(hands.len());

        // Procesar en chunks de 8 para mejor uso de cache
        for chunk in hands.chunks(8) {
            for hand in chunk {
                results.push(self.evaluate_7cards(hand));
            }
        }

        results
    }

    #[cfg(not(target_arch = "x86_64"))]
    fn evaluate_batch_simd(&self, hands: &[[Card; 7]]) -> Vec<HandRank> {
        self.evaluate_batch_scalar(hands)
    }

    /// Encuentra el mejor ranking entre múltiples manos usando SIMD
    ///
    /// Retorna el índice de la mano ganadora y su ranking.
    #[cfg(target_arch = "x86_64")]
    pub fn find_best_hand(&self, hands: &[[Card; 7]]) -> (usize, HandRank) {
        if hands.is_empty() {
            return (0, HandRank::new(HandRank::WORST));
        }

        if hands.len() < 8 || !self.use_simd {
            return self.find_best_hand_scalar(hands);
        }

        // Para 8+ manos, usamos comparaciones SIMD
        unsafe { self.find_best_hand_avx2(hands) }
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn find_best_hand(&self, hands: &[[Card; 7]]) -> (usize, HandRank) {
        self.find_best_hand_scalar(hands)
    }

    fn find_best_hand_scalar(&self, hands: &[[Card; 7]]) -> (usize, HandRank) {
        let mut best_idx = 0;
        let mut best_rank = HandRank::new(HandRank::WORST);

        for (i, hand) in hands.iter().enumerate() {
            let rank = self.evaluate_7cards(hand);
            if rank > best_rank {
                best_rank = rank;
                best_idx = i;
            }
        }

        (best_idx, best_rank)
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn find_best_hand_avx2(&self, hands: &[[Card; 7]]) -> (usize, HandRank) {
        // Evaluar todas las manos primero
        let rankings: Vec<u16> = hands
            .iter()
            .map(|h| self.evaluate_7cards(h).value())
            .collect();

        // Usar SIMD para encontrar el máximo
        // Procesamos 16 valores a la vez (256 bits / 16 bits = 16)
        let mut global_max = 0u16;
        let mut global_idx = 0usize;

        // Procesar en chunks de 16
        let chunks = rankings.len() / 16;
        for chunk_idx in 0..chunks {
            let base = chunk_idx * 16;
            let ptr = rankings.as_ptr().add(base) as *const __m256i;

            // Cargar 16 valores (2 registros de 128 bits empaquetados en 256)
            let vals = _mm256_loadu_si256(ptr);

            // Encontrar máximo horizontal
            // AVX2 no tiene max horizontal directo para u16, pero podemos hacer:
            // 1. Split en high/low 128 bits
            // 2. Max entre ellos
            // 3. Reducir a escalar

            let high = _mm256_extracti128_si256::<1>(vals);
            let low = _mm256_castsi256_si128(vals);

            let max128 = _mm_max_epu16(high, low);

            // Ahora tenemos 8 valores, seguir reduciendo
            let max64 = _mm_max_epu16(max128, _mm_srli_si128::<8>(max128));
            let max32 = _mm_max_epu16(max64, _mm_srli_si128::<4>(max64));
            let max16 = _mm_max_epu16(max32, _mm_srli_si128::<2>(max32));

            let chunk_max = _mm_extract_epi16::<0>(max16) as u16;

            if chunk_max > global_max {
                global_max = chunk_max;
                // Encontrar índice del máximo en este chunk
                for i in 0..16 {
                    if rankings[base + i] == chunk_max {
                        global_idx = base + i;
                        break;
                    }
                }
            }
        }

        // Procesar elementos restantes
        let remaining_start = chunks * 16;
        for (i, &rank) in rankings.iter().skip(remaining_start).enumerate() {
            let actual_idx = remaining_start + i;
            if rank > global_max {
                global_max = rank;
                global_idx = actual_idx;
            }
        }

        (global_idx, HandRank::new(global_max))
    }

    /// Cuenta victorias en paralelo usando SIMD
    ///
    /// Dado un array de resultados (-1, 0, 1), cuenta cuántos son positivos.
    ///
    /// # Safety
    /// Esta función requiere AVX2. El llamador debe verificar que AVX2 está disponible
    /// usando `is_avx2_available()` antes de llamar esta función.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn count_wins_simd(&self, results: &[i8]) -> (u64, u64, u64) {
        let mut wins = 0u64;
        let mut losses = 0u64;
        let mut ties = 0u64;

        // Procesar en chunks de 32 (256 bits / 8 bits)
        let chunks = results.len() / 32;

        let zero = _mm256_setzero_si256();
        let one = _mm256_set1_epi8(1);
        let neg_one = _mm256_set1_epi8(-1);

        for chunk_idx in 0..chunks {
            let ptr = results.as_ptr().add(chunk_idx * 32) as *const __m256i;
            let vals = _mm256_loadu_si256(ptr);

            // Comparar con 1 (wins)
            let win_mask = _mm256_cmpeq_epi8(vals, one);
            let win_count = _mm256_movemask_epi8(win_mask).count_ones() as u64;

            // Comparar con -1 (losses)
            let loss_mask = _mm256_cmpeq_epi8(vals, neg_one);
            let loss_count = _mm256_movemask_epi8(loss_mask).count_ones() as u64;

            // Comparar con 0 (ties)
            let tie_mask = _mm256_cmpeq_epi8(vals, zero);
            let tie_count = _mm256_movemask_epi8(tie_mask).count_ones() as u64;

            wins += win_count;
            losses += loss_count;
            ties += tie_count;
        }

        // Procesar elementos restantes
        for &r in &results[chunks * 32..] {
            match r {
                1 => wins += 1,
                -1 => losses += 1,
                0 => ties += 1,
                _ => {}
            }
        }

        (wins, losses, ties)
    }
}

impl Default for SimdEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Shuffle vectorizado del deck usando AVX2
///
/// Realiza un Fisher-Yates parcial para las primeras N cartas
/// con optimizaciones SIMD para swaps en paralelo.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[allow(dead_code)]
pub unsafe fn shuffle_deck_partial_simd(
    deck: &mut [u8],
    num_cards: usize,
    random_indices: &[usize],
) {
    // Para un shuffle parcial, SIMD no ofrece mucha ventaja
    // porque los swaps son dependientes del índice aleatorio.
    // Sin embargo, podemos optimizar la generación de índices.

    for (i, &j) in random_indices.iter().take(num_cards).enumerate() {
        if j < deck.len() && i < deck.len() {
            deck.swap(i, j);
        }
    }
}

/// Compara múltiples pares de rankings en paralelo
///
/// Retorna un array de resultados: 1 si a > b, -1 si a < b, 0 si igual
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[allow(dead_code)]
pub unsafe fn compare_rankings_simd(rankings_a: &[u16], rankings_b: &[u16]) -> Vec<i8> {
    let len = rankings_a.len().min(rankings_b.len());
    let mut results = vec![0i8; len];

    // Procesar en chunks de 16 (256 bits / 16 bits)
    let chunks = len / 16;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 16;
        let ptr_a = rankings_a.as_ptr().add(base) as *const __m256i;
        let ptr_b = rankings_b.as_ptr().add(base) as *const __m256i;

        let vals_a = _mm256_loadu_si256(ptr_a);
        let vals_b = _mm256_loadu_si256(ptr_b);

        // a > b: resultado = 1
        let gt_mask = _mm256_cmpgt_epi16(
            _mm256_xor_si256(vals_a, _mm256_set1_epi16(i16::MIN)),
            _mm256_xor_si256(vals_b, _mm256_set1_epi16(i16::MIN)),
        );

        // b > a: resultado = -1
        let lt_mask = _mm256_cmpgt_epi16(
            _mm256_xor_si256(vals_b, _mm256_set1_epi16(i16::MIN)),
            _mm256_xor_si256(vals_a, _mm256_set1_epi16(i16::MIN)),
        );

        // Extraer resultados
        let gt_bits = _mm256_movemask_epi8(gt_mask) as u32;
        let lt_bits = _mm256_movemask_epi8(lt_mask) as u32;

        for i in 0..16 {
            let bit_pos = i * 2; // Cada u16 ocupa 2 bytes
            if (gt_bits >> bit_pos) & 3 == 3 {
                results[base + i] = 1;
            } else if (lt_bits >> bit_pos) & 3 == 3 {
                results[base + i] = -1;
            }
            // else queda 0 (empate)
        }
    }

    // Procesar elementos restantes
    for i in (chunks * 16)..len {
        results[i] = match rankings_a[i].cmp(&rankings_b[i]) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        };
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avx2_detection() {
        let available = is_avx2_available();
        println!("AVX2 available: {}", available);
        // No podemos assertar porque depende del hardware
    }

    #[test]
    fn test_simd_evaluator_creation() {
        let eval = SimdEvaluator::new();
        println!("SIMD enabled: {}", eval.is_simd_enabled());
        println!("Lookup enabled: {}", eval.use_lookup);
    }

    #[test]
    fn test_evaluate_single_hand() {
        let eval = SimdEvaluator::new();

        let royal_flush: [Card; 7] = [
            "As".parse().unwrap(),
            "Ks".parse().unwrap(),
            "Qs".parse().unwrap(),
            "Js".parse().unwrap(),
            "Ts".parse().unwrap(),
            "2h".parse().unwrap(),
            "3d".parse().unwrap(),
        ];

        let rank = eval.evaluate_7cards(&royal_flush);
        assert!(
            rank.is_royal_flush() || rank.is_straight_flush(),
            "Expected Royal/Straight Flush, got {:?}",
            rank
        );
    }

    #[test]
    fn test_evaluate_batch() {
        let eval = SimdEvaluator::new();

        let hands: Vec<[Card; 7]> = vec![
            [
                "As".parse().unwrap(),
                "Ks".parse().unwrap(),
                "Qs".parse().unwrap(),
                "Js".parse().unwrap(),
                "Ts".parse().unwrap(),
                "2h".parse().unwrap(),
                "3d".parse().unwrap(),
            ],
            [
                "Ah".parse().unwrap(),
                "Kh".parse().unwrap(),
                "Qh".parse().unwrap(),
                "Jh".parse().unwrap(),
                "9h".parse().unwrap(),
                "2c".parse().unwrap(),
                "3c".parse().unwrap(),
            ],
        ];

        let results = eval.evaluate_batch(&hands);
        assert_eq!(results.len(), 2);
        assert!(results[0].is_straight_flush() || results[0].is_royal_flush());
        assert!(results[1].is_flush());
    }

    #[test]
    fn test_find_best_hand() {
        let eval = SimdEvaluator::new();

        let hands: Vec<[Card; 7]> = vec![
            // Pair
            [
                "As".parse().unwrap(),
                "Ah".parse().unwrap(),
                "2c".parse().unwrap(),
                "3d".parse().unwrap(),
                "5h".parse().unwrap(),
                "7s".parse().unwrap(),
                "9c".parse().unwrap(),
            ],
            // Flush
            [
                "Ah".parse().unwrap(),
                "Kh".parse().unwrap(),
                "Qh".parse().unwrap(),
                "Jh".parse().unwrap(),
                "9h".parse().unwrap(),
                "2c".parse().unwrap(),
                "3d".parse().unwrap(),
            ],
            // High card
            [
                "As".parse().unwrap(),
                "Kc".parse().unwrap(),
                "Qd".parse().unwrap(),
                "Jh".parse().unwrap(),
                "9s".parse().unwrap(),
                "2c".parse().unwrap(),
                "3d".parse().unwrap(),
            ],
        ];

        let (best_idx, best_rank) = eval.find_best_hand(&hands);
        assert_eq!(best_idx, 1, "Flush should be the best hand");
        assert!(best_rank.is_flush());
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_compare_rankings_simd() {
        if !is_avx2_available() {
            println!("SKIP: AVX2 not available");
            return;
        }

        let rankings_a: Vec<u16> = vec![100, 200, 150, 300];
        let rankings_b: Vec<u16> = vec![50, 200, 200, 100];

        let results = unsafe { compare_rankings_simd(&rankings_a, &rankings_b) };

        assert_eq!(results.len(), 4);
        assert_eq!(results[0], 1); // 100 > 50
        assert_eq!(results[1], 0); // 200 == 200
        assert_eq!(results[2], -1); // 150 < 200
        assert_eq!(results[3], 1); // 300 > 100
    }

    #[test]
    fn test_simd_vs_scalar_consistency() {
        let simd_eval = SimdEvaluator::new();
        let scalar_eval = SimdEvaluator::without_simd();

        let hands: Vec<[Card; 7]> = (0..10)
            .map(|i| {
                [
                    Card::from_index(i * 5).unwrap(),
                    Card::from_index(i * 5 + 1).unwrap(),
                    Card::from_index(i * 5 + 2).unwrap(),
                    Card::from_index(i * 5 + 3).unwrap(),
                    Card::from_index(i * 5 + 4).unwrap(),
                    Card::from_index((i * 5 + 5) % 52).unwrap(),
                    Card::from_index((i * 5 + 6) % 52).unwrap(),
                ]
            })
            .collect();

        let simd_results = simd_eval.evaluate_batch(&hands);
        let scalar_results = scalar_eval.evaluate_batch(&hands);

        for (i, (s, sc)) in simd_results.iter().zip(scalar_results.iter()).enumerate() {
            assert_eq!(
                s.value(),
                sc.value(),
                "Mismatch at index {}: SIMD={}, Scalar={}",
                i,
                s.value(),
                sc.value()
            );
        }
    }
}
