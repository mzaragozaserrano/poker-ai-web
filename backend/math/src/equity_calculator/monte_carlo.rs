//! # Simulación Monte Carlo
//!
//! Implementación del simulador Monte Carlo para cálculo de equity.
//! Optimizado para paralelización con Rayon y convergencia temprana.

use crate::hand_evaluator::{Card, HandRank, CARDS};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

use super::simd::SimdEvaluator;

/// Configuración para la simulación Monte Carlo
#[derive(Debug, Clone)]
pub struct MonteCarloConfig {
    /// Número de simulaciones a ejecutar
    pub num_simulations: u32,
    /// Umbral de convergencia (0.001 = 0.1%)
    pub convergence_threshold: f64,
    /// Intervalo de verificación de convergencia (cada N simulaciones)
    pub convergence_check_interval: u32,
    /// Usar SIMD AVX2 si está disponible
    pub use_simd: bool,
    /// Número de threads para paralelización (0 = auto)
    pub num_threads: usize,
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        MonteCarloConfig {
            num_simulations: 100_000,
            convergence_threshold: 0.001, // 0.1%
            convergence_check_interval: 10_000,
            use_simd: true,
            num_threads: 0, // Auto-detect
        }
    }
}

/// Resultado del cálculo de equity
#[derive(Debug, Clone)]
pub struct EquityResult {
    /// Equity del héroe (0.0 - 1.0)
    pub hero_equity: f64,
    /// Equity del villano (0.0 - 1.0)
    pub villain_equity: f64,
    /// Probabilidad de empate (0.0 - 1.0)
    pub tie_equity: f64,
    /// Número de simulaciones ejecutadas
    pub simulations_run: u32,
    /// Si convergió antes de completar todas las simulaciones
    pub converged_early: bool,
    /// Error estándar estimado de la equity
    pub standard_error: f64,
}

impl EquityResult {
    /// Crea un resultado para un escenario sin cartas desconocidas
    pub fn deterministic(hero_wins: bool, tie: bool) -> Self {
        if tie {
            EquityResult {
                hero_equity: 0.5,
                villain_equity: 0.5,
                tie_equity: 1.0,
                simulations_run: 1,
                converged_early: true,
                standard_error: 0.0,
            }
        } else if hero_wins {
            EquityResult {
                hero_equity: 1.0,
                villain_equity: 0.0,
                tie_equity: 0.0,
                simulations_run: 1,
                converged_early: true,
                standard_error: 0.0,
            }
        } else {
            EquityResult {
                hero_equity: 0.0,
                villain_equity: 1.0,
                tie_equity: 0.0,
                simulations_run: 1,
                converged_early: true,
                standard_error: 0.0,
            }
        }
    }
}

/// Calcula la equity de una mano contra otra
///
/// # Arguments
/// * `hero_cards` - Cartas del héroe (2 cartas)
/// * `villain_cards` - Cartas del villano (2 cartas)
/// * `board_cards` - Cartas comunitarias (0-5 cartas)
/// * `num_simulations` - Número de simulaciones a ejecutar
///
/// # Returns
/// * `EquityResult` con las probabilidades calculadas
///
/// # Example
/// ```rust,ignore
/// use poker_math::equity_calculator::calculate_equity;
///
/// let result = calculate_equity(
///     &["As", "Ah"],
///     &["Ks", "Kh"],
///     &[],
///     10000
/// );
/// assert!(result.hero_equity > 0.80); // AA vs KK ~ 82%
/// ```
pub fn calculate_equity(
    hero_cards: &[&str],
    villain_cards: &[&str],
    board_cards: &[&str],
    num_simulations: u32,
) -> EquityResult {
    let config = MonteCarloConfig {
        num_simulations,
        ..Default::default()
    };

    calculate_equity_with_config(hero_cards, villain_cards, board_cards, &config)
}

/// Calcula la equity con configuración personalizada
pub fn calculate_equity_with_config(
    hero_cards: &[&str],
    villain_cards: &[&str],
    board_cards: &[&str],
    config: &MonteCarloConfig,
) -> EquityResult {
    // Parsear cartas
    let hero: Vec<Card> = hero_cards.iter().filter_map(|s| s.parse().ok()).collect();
    let villain: Vec<Card> = villain_cards
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    let board: Vec<Card> = board_cards.iter().filter_map(|s| s.parse().ok()).collect();

    // Validar input
    if hero.len() != 2 || villain.len() != 2 {
        return EquityResult {
            hero_equity: 0.0,
            villain_equity: 0.0,
            tie_equity: 0.0,
            simulations_run: 0,
            converged_early: false,
            standard_error: 1.0,
        };
    }

    if board.len() > 5 {
        return EquityResult {
            hero_equity: 0.0,
            villain_equity: 0.0,
            tie_equity: 0.0,
            simulations_run: 0,
            converged_early: false,
            standard_error: 1.0,
        };
    }

    // Si el board está completo (5 cartas), es determinístico
    if board.len() == 5 {
        return calculate_deterministic(&hero, &villain, &board);
    }

    // Ejecutar Monte Carlo paralelo
    run_monte_carlo_parallel(&hero, &villain, &board, config)
}

/// Calcula equity para un board completo (determinístico)
fn calculate_deterministic(hero: &[Card], villain: &[Card], board: &[Card]) -> EquityResult {
    let evaluator = SimdEvaluator::new();

    // Construir manos de 7 cartas
    let mut hero_hand = [Card::from_index(0).unwrap(); 7];
    let mut villain_hand = [Card::from_index(0).unwrap(); 7];

    hero_hand[0] = hero[0];
    hero_hand[1] = hero[1];
    villain_hand[0] = villain[0];
    villain_hand[1] = villain[1];

    for (i, &card) in board.iter().enumerate() {
        hero_hand[i + 2] = card;
        villain_hand[i + 2] = card;
    }

    let hero_rank = evaluator.evaluate_7cards(&hero_hand);
    let villain_rank = evaluator.evaluate_7cards(&villain_hand);

    match hero_rank.cmp(&villain_rank) {
        std::cmp::Ordering::Greater => EquityResult::deterministic(true, false),
        std::cmp::Ordering::Less => EquityResult::deterministic(false, false),
        std::cmp::Ordering::Equal => EquityResult::deterministic(false, true),
    }
}

/// Ejecuta Monte Carlo en paralelo con Rayon y early stopping
fn run_monte_carlo_parallel(
    hero: &[Card],
    villain: &[Card],
    board: &[Card],
    config: &MonteCarloConfig,
) -> EquityResult {
    // Cartas conocidas (remover del deck)
    let mut known_cards: Vec<Card> = Vec::with_capacity(9);
    known_cards.extend_from_slice(hero);
    known_cards.extend_from_slice(villain);
    known_cards.extend_from_slice(board);

    // Cartas restantes del deck
    let remaining_cards: Vec<Card> = CARDS
        .iter()
        .copied()
        .filter(|c| !known_cards.contains(c))
        .collect();

    let cards_needed = 5 - board.len();

    // Contadores atómicos para resultados
    let hero_wins = AtomicU64::new(0);
    let villain_wins = AtomicU64::new(0);
    let ties = AtomicU64::new(0);
    let total_simulations = AtomicU64::new(0);

    // Flag para early stopping
    let should_stop = std::sync::atomic::AtomicBool::new(false);

    // Número de chunks para paralelización
    let num_threads = if config.num_threads == 0 {
        rayon::current_num_threads()
    } else {
        config.num_threads
    };

    let sims_per_thread = config.num_simulations / num_threads as u32;
    let check_interval = config.convergence_check_interval / num_threads as u32;

    // Ejecutar en paralelo
    (0..num_threads).into_par_iter().for_each(|thread_id| {
        // RNG por thread con seed único
        let mut rng = ChaCha8Rng::seed_from_u64(thread_id as u64 * 12345 + 67890);
        let evaluator = SimdEvaluator::new();

        // Copia local del deck restante
        let mut deck = remaining_cards.clone();

        let mut local_hero_wins = 0u64;
        let mut local_villain_wins = 0u64;
        let mut local_ties = 0u64;
        let mut local_sims = 0u32;

        // Para tracking de convergencia
        let mut prev_equity = 0.5f64;

        // Buffers para manos
        let mut hero_hand = [Card::from_index(0).unwrap(); 7];
        let mut villain_hand = [Card::from_index(0).unwrap(); 7];

        // Copiar cartas conocidas
        hero_hand[0] = hero[0];
        hero_hand[1] = hero[1];
        villain_hand[0] = villain[0];
        villain_hand[1] = villain[1];

        for (i, &card) in board.iter().enumerate() {
            hero_hand[i + 2] = card;
            villain_hand[i + 2] = card;
        }

        for sim_idx in 0..sims_per_thread {
            // Check early stopping
            if should_stop.load(Ordering::Relaxed) {
                break;
            }

            // Fisher-Yates parcial para las cartas que necesitamos
            for i in 0..cards_needed {
                let j = rng.gen_range(i..deck.len());
                deck.swap(i, j);
            }

            // Completar las manos con las cartas del runout
            for (idx, card) in deck.iter().take(cards_needed).enumerate() {
                let card_idx = board.len() + 2 + idx;
                hero_hand[card_idx] = *card;
                villain_hand[card_idx] = *card;
            }

            // Evaluar manos
            let hero_rank = evaluator.evaluate_7cards(&hero_hand);
            let villain_rank = evaluator.evaluate_7cards(&villain_hand);

            // Comparar
            match hero_rank.cmp(&villain_rank) {
                std::cmp::Ordering::Greater => local_hero_wins += 1,
                std::cmp::Ordering::Less => local_villain_wins += 1,
                std::cmp::Ordering::Equal => local_ties += 1,
            }

            local_sims += 1;

            // Verificar convergencia periódicamente (solo thread 0)
            if thread_id == 0 && check_interval > 0 && sim_idx % check_interval == 0 && sim_idx > 0
            {
                let total_local =
                    local_hero_wins as f64 + local_villain_wins as f64 + local_ties as f64;
                if total_local > 0.0 {
                    let current_equity =
                        (local_hero_wins as f64 + local_ties as f64 * 0.5) / total_local;
                    let change = (current_equity - prev_equity).abs();

                    if change < config.convergence_threshold {
                        should_stop.store(true, Ordering::Relaxed);
                    }
                    prev_equity = current_equity;
                }
            }
        }

        // Agregar resultados al contador global
        hero_wins.fetch_add(local_hero_wins, Ordering::Relaxed);
        villain_wins.fetch_add(local_villain_wins, Ordering::Relaxed);
        ties.fetch_add(local_ties, Ordering::Relaxed);
        total_simulations.fetch_add(local_sims as u64, Ordering::Relaxed);
    });

    let total_hero = hero_wins.load(Ordering::Relaxed) as f64;
    let total_villain = villain_wins.load(Ordering::Relaxed) as f64;
    let total_ties = ties.load(Ordering::Relaxed) as f64;
    let total_sims = total_simulations.load(Ordering::Relaxed) as f64;
    let converged = should_stop.load(Ordering::Relaxed);

    let hero_equity = (total_hero + total_ties * 0.5) / total_sims;
    let villain_equity = (total_villain + total_ties * 0.5) / total_sims;
    let tie_equity = total_ties / total_sims;

    // Calcular error estándar
    let standard_error = (hero_equity * (1.0 - hero_equity) / total_sims).sqrt();

    EquityResult {
        hero_equity,
        villain_equity,
        tie_equity,
        simulations_run: total_sims as u32,
        converged_early: converged,
        standard_error,
    }
}

/// Simula una sola mano y devuelve el resultado
///
/// # Arguments
/// * `hero` - Cartas del héroe
/// * `villain` - Cartas del villano
/// * `board` - Cartas comunitarias (puede estar vacío o parcial)
/// * `rng` - Generador de números aleatorios
///
/// # Returns
/// * `1` si héroe gana, `-1` si villano gana, `0` si empate
pub fn simulate_single<R: Rng>(
    hero: &[Card; 2],
    villain: &[Card; 2],
    board: &[Card],
    remaining_deck: &mut [Card],
    rng: &mut R,
) -> i8 {
    let evaluator = SimdEvaluator::new();
    let cards_needed = 5 - board.len();

    // Fisher-Yates parcial
    for i in 0..cards_needed {
        let j = rng.gen_range(i..remaining_deck.len());
        remaining_deck.swap(i, j);
    }

    // Construir manos
    let mut hero_hand = [Card::from_index(0).unwrap(); 7];
    let mut villain_hand = [Card::from_index(0).unwrap(); 7];

    hero_hand[0] = hero[0];
    hero_hand[1] = hero[1];
    villain_hand[0] = villain[0];
    villain_hand[1] = villain[1];

    for (i, &card) in board.iter().enumerate() {
        hero_hand[i + 2] = card;
        villain_hand[i + 2] = card;
    }

    for (idx, card) in remaining_deck.iter().take(cards_needed).enumerate() {
        let card_idx = board.len() + 2 + idx;
        hero_hand[card_idx] = *card;
        villain_hand[card_idx] = *card;
    }

    let hero_rank = evaluator.evaluate_7cards(&hero_hand);
    let villain_rank = evaluator.evaluate_7cards(&villain_hand);

    match hero_rank.cmp(&villain_rank) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

/// Calcula equity en un escenario multiway (3+ jugadores)
///
/// # Arguments
/// * `hands` - Vector de manos (cada mano es un array de 2 cartas)
/// * `board` - Cartas comunitarias
/// * `num_simulations` - Número de simulaciones
///
/// # Returns
/// * Vector de equities para cada jugador
pub fn calculate_equity_multiway(
    hands: &[&[&str]],
    board: &[&str],
    num_simulations: u32,
) -> Vec<f64> {
    // Parsear todas las manos
    let parsed_hands: Vec<Vec<Card>> = hands
        .iter()
        .map(|h| h.iter().filter_map(|s| s.parse().ok()).collect())
        .collect();

    let board_cards: Vec<Card> = board.iter().filter_map(|s| s.parse().ok()).collect();

    // Validar
    for hand in &parsed_hands {
        if hand.len() != 2 {
            return vec![0.0; hands.len()];
        }
    }

    if board_cards.len() > 5 {
        return vec![0.0; hands.len()];
    }

    // Cartas conocidas
    let mut known_cards: Vec<Card> = Vec::with_capacity(hands.len() * 2 + 5);
    for hand in &parsed_hands {
        known_cards.extend_from_slice(hand);
    }
    known_cards.extend_from_slice(&board_cards);

    // Deck restante
    let remaining_cards: Vec<Card> = CARDS
        .iter()
        .copied()
        .filter(|c| !known_cards.contains(c))
        .collect();

    let cards_needed = 5 - board_cards.len();
    let num_players = parsed_hands.len();

    // Contadores atómicos
    let wins: Vec<AtomicU64> = (0..num_players).map(|_| AtomicU64::new(0)).collect();
    let ties_shared = AtomicU64::new(0);

    let num_threads = rayon::current_num_threads();
    let sims_per_thread = num_simulations / num_threads as u32;

    (0..num_threads).into_par_iter().for_each(|thread_id| {
        let mut rng = ChaCha8Rng::seed_from_u64(thread_id as u64 * 54321 + 98765);
        let evaluator = SimdEvaluator::new();
        let mut deck = remaining_cards.clone();

        let mut local_wins: Vec<u64> = vec![0; num_players];
        let mut local_ties = 0u64;

        // Buffers para manos
        let mut player_hands: Vec<[Card; 7]> = vec![[Card::from_index(0).unwrap(); 7]; num_players];

        // Copiar hole cards y board
        for (i, hand) in parsed_hands.iter().enumerate() {
            player_hands[i][0] = hand[0];
            player_hands[i][1] = hand[1];
            for (j, &card) in board_cards.iter().enumerate() {
                player_hands[i][j + 2] = card;
            }
        }

        for _ in 0..sims_per_thread {
            // Fisher-Yates parcial
            for i in 0..cards_needed {
                let j = rng.gen_range(i..deck.len());
                deck.swap(i, j);
            }

            // Completar manos con runout
            for player_hand in player_hands.iter_mut() {
                for i in 0..cards_needed {
                    player_hand[board_cards.len() + 2 + i] = deck[i];
                }
            }

            // Evaluar todas las manos
            let mut best_rank = HandRank::new(HandRank::WORST);
            let mut winner_idx = 0;
            let mut is_tie = false;

            for (i, hand) in player_hands.iter().enumerate() {
                let rank = evaluator.evaluate_7cards(hand);
                match rank.cmp(&best_rank) {
                    std::cmp::Ordering::Greater => {
                        best_rank = rank;
                        winner_idx = i;
                        is_tie = false;
                    }
                    std::cmp::Ordering::Equal => {
                        is_tie = true;
                    }
                    std::cmp::Ordering::Less => {}
                }
            }

            if is_tie {
                local_ties += 1;
            } else {
                local_wins[winner_idx] += 1;
            }
        }

        // Agregar a contadores globales
        for (i, &w) in local_wins.iter().enumerate() {
            wins[i].fetch_add(w, Ordering::Relaxed);
        }
        ties_shared.fetch_add(local_ties, Ordering::Relaxed);
    });

    // Calcular equities
    let total_ties = ties_shared.load(Ordering::Relaxed) as f64;
    let total_sims = sims_per_thread as f64 * num_threads as f64;
    let tie_share = total_ties / num_players as f64;

    wins.iter()
        .map(|w| {
            let player_wins = w.load(Ordering::Relaxed) as f64;
            (player_wins + tie_share) / total_sims
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aa_vs_kk_equity() {
        // AA vs KK es aproximadamente 82% vs 18%
        let result = calculate_equity(&["As", "Ah"], &["Ks", "Kh"], &[], 50_000);

        // Con 50K simulaciones, esperamos error < 1%
        assert!(
            result.hero_equity > 0.79 && result.hero_equity < 0.85,
            "AA equity should be ~82%, got {:.2}%",
            result.hero_equity * 100.0
        );
        assert!(
            result.villain_equity > 0.15 && result.villain_equity < 0.21,
            "KK equity should be ~18%, got {:.2}%",
            result.villain_equity * 100.0
        );
    }

    #[test]
    fn test_aksuited_vs_qq_equity() {
        // AKs vs QQ es aproximadamente 46% vs 54%
        let result = calculate_equity(&["As", "Ks"], &["Qh", "Qd"], &[], 50_000);

        assert!(
            result.hero_equity > 0.42 && result.hero_equity < 0.50,
            "AKs equity should be ~46%, got {:.2}%",
            result.hero_equity * 100.0
        );
    }

    #[test]
    fn test_coinflip_equity() {
        // 22 vs AKo es aproximadamente 52% vs 48% (coinflip)
        let result = calculate_equity(&["2s", "2h"], &["Ac", "Kd"], &[], 50_000);

        assert!(
            result.hero_equity > 0.48 && result.hero_equity < 0.56,
            "22 vs AK should be ~52%, got {:.2}%",
            result.hero_equity * 100.0
        );
    }

    #[test]
    fn test_dominated_equity() {
        // AK vs AQ es aproximadamente 70% vs 30% (AK domina)
        // Con cartas compartidas (ambos tienen As), el rango es más estrecho
        let result = calculate_equity(&["As", "Kh"], &["Ac", "Qd"], &[], 100_000);

        assert!(
            result.hero_equity > 0.65 && result.hero_equity < 0.80,
            "AK vs AQ should be ~70%, got {:.2}%",
            result.hero_equity * 100.0
        );
    }

    #[test]
    fn test_with_board_flop() {
        // AA vs KK con flop que no ayuda a nadie
        let result = calculate_equity(&["As", "Ah"], &["Ks", "Kh"], &["2c", "5d", "9h"], 50_000);

        assert!(
            result.hero_equity > 0.85,
            "AA should dominate on dry flop, got {:.2}%",
            result.hero_equity * 100.0
        );
    }

    #[test]
    fn test_with_board_turn() {
        // Test con board de 4 cartas
        let result = calculate_equity(
            &["As", "Ah"],
            &["Ks", "Kh"],
            &["2c", "5d", "9h", "3c"],
            50_000,
        );

        assert!(
            result.hero_equity > 0.90,
            "AA should strongly dominate on turn, got {:.2}%",
            result.hero_equity * 100.0
        );
    }

    #[test]
    fn test_deterministic_river() {
        // Board completo - resultado determinístico
        let result = calculate_equity(
            &["As", "Ah"],
            &["Ks", "Kh"],
            &["2c", "5d", "9h", "3c", "7d"],
            1,
        );

        // AA vs KK en este board: AA gana
        assert!(
            (result.hero_equity - 1.0).abs() < 0.01,
            "AA should win 100% on river, got {:.2}%",
            result.hero_equity * 100.0
        );
    }

    #[test]
    fn test_tie_scenario() {
        // Mismo par, mismo kicker -> empate
        let result = calculate_equity(
            &["As", "Kh"],
            &["Ad", "Kc"],
            &["2c", "5d", "9h", "3c", "7d"],
            1,
        );

        assert!(
            result.tie_equity > 0.99,
            "Should be a tie, got tie_equity {:.2}%",
            result.tie_equity * 100.0
        );
    }

    #[test]
    fn test_multiway_three_players() {
        // Test con 3 jugadores
        let equities =
            calculate_equity_multiway(&[&["As", "Ah"], &["Ks", "Kh"], &["Qs", "Qh"]], &[], 30_000);

        assert_eq!(equities.len(), 3);

        // AA debería tener la mayor equity
        assert!(
            equities[0] > equities[1] && equities[0] > equities[2],
            "AA should have highest equity"
        );

        // Suma debería ser ~1.0
        let sum: f64 = equities.iter().sum();
        assert!(
            (sum - 1.0).abs() < 0.05,
            "Equities should sum to 1.0, got {}",
            sum
        );
    }

    #[test]
    fn test_invalid_input() {
        // Input inválido debería devolver equity 0
        let result = calculate_equity(&["As"], &["Ks", "Kh"], &[], 1000);
        assert_eq!(result.simulations_run, 0);
    }

    #[test]
    fn test_standard_error_decreases() {
        // Más simulaciones = menor error estándar
        let result_small = calculate_equity(&["As", "Ah"], &["Ks", "Kh"], &[], 1_000);
        let result_large = calculate_equity(&["As", "Ah"], &["Ks", "Kh"], &[], 50_000);

        assert!(
            result_large.standard_error < result_small.standard_error,
            "More simulations should reduce standard error"
        );
    }
}
