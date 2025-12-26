//! # Hand Evaluator Module
//!
//! Evaluador de manos de poker de alto rendimiento para Texas Hold'em.
//!
//! ## Características
//! - Evaluación O(1) para 5 cartas usando lookup tables
//! - Soporte para 5, 6 y 7 cartas
//! - Rankings compatibles con el estándar de poker (1-7462)
//!
//! ## Uso
//!
//! ```rust
//! use poker_math::hand_evaluator::{Card, evaluate_5cards, evaluate_7cards};
//!
//! // Crear cartas
//! let cards: [Card; 5] = [
//!     "As".parse().unwrap(),
//!     "Ks".parse().unwrap(),
//!     "Qs".parse().unwrap(),
//!     "Js".parse().unwrap(),
//!     "Ts".parse().unwrap(),
//! ];
//!
//! // Evaluar
//! let rank = evaluate_5cards(&cards);
//! assert!(rank.is_royal_flush());
//! ```
//!
//! ## Performance
//!
//! Objetivos de rendimiento en Ryzen 7 3800X:
//! - 5 cartas: < 50ns
//! - 7 cartas: < 200ns

mod cards;
mod evaluator;
mod hand_rank;
mod lookup;

// Re-exports públicos
pub use cards::{Card, Deck, Rank, Suit, CARDS, PRIMES};
pub use evaluator::{evaluate, evaluate_5cards, evaluate_6cards, evaluate_7cards};
pub use hand_rank::{HandCategory, HandRank};

/// Evalúa una mano desde strings de cartas
///
/// # Arguments
/// * `cards` - Strings de cartas separadas por espacios ("As Ks Qs Js Ts")
///
/// # Returns
/// * `Option<HandRank>` - El ranking si las cartas son válidas
///
/// # Example
/// ```rust
/// use poker_math::hand_evaluator::evaluate_from_strings;
///
/// let rank = evaluate_from_strings("As Ks Qs Js Ts").unwrap();
/// assert!(rank.is_royal_flush());
/// ```
pub fn evaluate_from_strings(cards_str: &str) -> Option<HandRank> {
    let cards: Vec<Card> = cards_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    evaluate(&cards)
}

/// Compara dos manos y devuelve el resultado
///
/// # Returns
/// * `std::cmp::Ordering` - Greater si hand1 gana, Less si hand2 gana, Equal si empate
pub fn compare_hands(hand1: &[Card], hand2: &[Card]) -> Option<std::cmp::Ordering> {
    let rank1 = evaluate(hand1)?;
    let rank2 = evaluate(hand2)?;
    Some(rank1.cmp(&rank2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_from_strings() {
        let rank = evaluate_from_strings("As Ks Qs Js Ts").unwrap();
        assert!(rank.is_royal_flush());
    }

    #[test]
    fn test_compare_hands() {
        let royal = cards_from_str("As Ks Qs Js Ts");
        let pair = cards_from_str("Ah Ad Kc Qh Jd");

        let result = compare_hands(&royal, &pair).unwrap();
        assert_eq!(result, std::cmp::Ordering::Greater);
    }

    fn cards_from_str(s: &str) -> Vec<Card> {
        s.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
    }
}
