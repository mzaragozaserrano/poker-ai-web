//! # Ranking de Manos de Poker
//!
//! Define los tipos de manos y su ordenamiento para Texas Hold'em.
//! El ranking sigue el estándar de poker donde valores menores = manos más fuertes.

use std::cmp::Ordering;
use std::fmt;

/// Categorías de manos de poker (de mejor a peor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HandCategory {
    StraightFlush = 1,  // Incluye Royal Flush
    FourOfAKind = 2,    // Poker
    FullHouse = 3,      // Full
    Flush = 4,          // Color
    Straight = 5,       // Escalera
    ThreeOfAKind = 6,   // Trío
    TwoPair = 7,        // Doble pareja
    OnePair = 8,        // Pareja
    HighCard = 9,       // Carta alta
}

impl HandCategory {
    /// Nombre en español de la categoría
    pub const fn name_es(&self) -> &'static str {
        match self {
            HandCategory::StraightFlush => "Escalera de Color",
            HandCategory::FourOfAKind => "Poker",
            HandCategory::FullHouse => "Full",
            HandCategory::Flush => "Color",
            HandCategory::Straight => "Escalera",
            HandCategory::ThreeOfAKind => "Trío",
            HandCategory::TwoPair => "Doble Pareja",
            HandCategory::OnePair => "Pareja",
            HandCategory::HighCard => "Carta Alta",
        }
    }

    /// Nombre en inglés de la categoría
    pub const fn name_en(&self) -> &'static str {
        match self {
            HandCategory::StraightFlush => "Straight Flush",
            HandCategory::FourOfAKind => "Four of a Kind",
            HandCategory::FullHouse => "Full House",
            HandCategory::Flush => "Flush",
            HandCategory::Straight => "Straight",
            HandCategory::ThreeOfAKind => "Three of a Kind",
            HandCategory::TwoPair => "Two Pair",
            HandCategory::OnePair => "One Pair",
            HandCategory::HighCard => "High Card",
        }
    }

    /// Devuelve la categoría desde un valor de ranking (1-7462)
    pub fn from_rank_value(rank: u16) -> Self {
        match rank {
            1..=10 => HandCategory::StraightFlush,
            11..=166 => HandCategory::FourOfAKind,
            167..=322 => HandCategory::FullHouse,
            323..=1599 => HandCategory::Flush,
            1600..=1609 => HandCategory::Straight,
            1610..=2467 => HandCategory::ThreeOfAKind,
            2468..=3325 => HandCategory::TwoPair,
            3326..=6185 => HandCategory::OnePair,
            _ => HandCategory::HighCard,
        }
    }
}

impl fmt::Display for HandCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_en())
    }
}

/// Ranking completo de una mano de poker
///
/// El valor de ranking va de 1 (mejor: Royal Flush) a 7462 (peor: 7-5-4-3-2 offsuit).
/// Valores menores = manos más fuertes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandRank {
    /// Valor absoluto del ranking (1-7462, menor = mejor)
    value: u16,
}

impl HandRank {
    /// Valor del mejor ranking posible (Royal Flush)
    pub const BEST: u16 = 1;

    /// Valor del peor ranking posible (7-5-4-3-2 offsuit)
    pub const WORST: u16 = 7462;

    /// Número total de rankings únicos de 5 cartas
    pub const TOTAL_RANKINGS: u16 = 7462;

    /// Crea un nuevo HandRank desde un valor (1-7462)
    #[inline]
    pub const fn new(value: u16) -> Self {
        HandRank { value }
    }

    /// Devuelve el valor numérico del ranking
    #[inline]
    pub const fn value(&self) -> u16 {
        self.value
    }

    /// Devuelve la categoría de la mano
    #[inline]
    pub fn category(&self) -> HandCategory {
        HandCategory::from_rank_value(self.value)
    }

    /// Verifica si es Royal Flush (A-K-Q-J-T del mismo palo)
    #[inline]
    pub fn is_royal_flush(&self) -> bool {
        self.value == 1
    }

    /// Verifica si es Straight Flush (incluyendo Royal)
    #[inline]
    pub fn is_straight_flush(&self) -> bool {
        self.value <= 10
    }

    /// Verifica si es Four of a Kind
    #[inline]
    pub fn is_four_of_a_kind(&self) -> bool {
        self.value >= 11 && self.value <= 166
    }

    /// Verifica si es Full House
    #[inline]
    pub fn is_full_house(&self) -> bool {
        self.value >= 167 && self.value <= 322
    }

    /// Verifica si es Flush
    #[inline]
    pub fn is_flush(&self) -> bool {
        self.value >= 323 && self.value <= 1599
    }

    /// Verifica si es Straight
    #[inline]
    pub fn is_straight(&self) -> bool {
        self.value >= 1600 && self.value <= 1609
    }

    /// Verifica si es Three of a Kind
    #[inline]
    pub fn is_three_of_a_kind(&self) -> bool {
        self.value >= 1610 && self.value <= 2467
    }

    /// Verifica si es Two Pair
    #[inline]
    pub fn is_two_pair(&self) -> bool {
        self.value >= 2468 && self.value <= 3325
    }

    /// Verifica si es One Pair
    #[inline]
    pub fn is_one_pair(&self) -> bool {
        self.value >= 3326 && self.value <= 6185
    }

    /// Verifica si es High Card
    #[inline]
    pub fn is_high_card(&self) -> bool {
        self.value > 6185
    }

    /// Calcula el percentil de fuerza (0.0 = peor, 1.0 = mejor)
    #[inline]
    pub fn percentile(&self) -> f64 {
        1.0 - (self.value as f64 - 1.0) / (Self::TOTAL_RANKINGS as f64 - 1.0)
    }

    /// Descripción textual de la mano
    pub fn description(&self) -> String {
        let category = self.category();
        format!("{} (rank {})", category.name_en(), self.value)
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> Ordering {
        // Menor valor = mejor mano, así que invertimos la comparación
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandRank({}, {})", self.value, self.category())
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_category_order() {
        // Royal Flush debe ser el mejor
        let royal = HandRank::new(1);
        assert_eq!(royal.category(), HandCategory::StraightFlush);
        assert!(royal.is_royal_flush());

        // High Card debe ser el peor
        let worst = HandRank::new(7462);
        assert_eq!(worst.category(), HandCategory::HighCard);
    }

    #[test]
    fn test_hand_rank_comparison() {
        let royal = HandRank::new(1);
        let pair = HandRank::new(4000);
        let high_card = HandRank::new(7000);

        // Royal > Pair > High Card
        assert!(royal > pair);
        assert!(pair > high_card);
        assert!(royal > high_card);
    }

    #[test]
    fn test_percentile() {
        let best = HandRank::new(1);
        assert!((best.percentile() - 1.0).abs() < 0.001);

        let worst = HandRank::new(7462);
        assert!(worst.percentile() < 0.001);
    }

    #[test]
    fn test_category_boundaries() {
        // Straight Flush: 1-10
        assert_eq!(
            HandRank::new(10).category(),
            HandCategory::StraightFlush
        );
        assert_eq!(HandRank::new(11).category(), HandCategory::FourOfAKind);

        // Four of a Kind: 11-166
        assert_eq!(HandRank::new(166).category(), HandCategory::FourOfAKind);
        assert_eq!(HandRank::new(167).category(), HandCategory::FullHouse);

        // Full House: 167-322
        assert_eq!(HandRank::new(322).category(), HandCategory::FullHouse);
        assert_eq!(HandRank::new(323).category(), HandCategory::Flush);

        // Flush: 323-1599
        assert_eq!(HandRank::new(1599).category(), HandCategory::Flush);
        assert_eq!(HandRank::new(1600).category(), HandCategory::Straight);

        // Straight: 1600-1609
        assert_eq!(HandRank::new(1609).category(), HandCategory::Straight);
        assert_eq!(HandRank::new(1610).category(), HandCategory::ThreeOfAKind);
    }

    #[test]
    fn test_category_names() {
        assert_eq!(HandCategory::StraightFlush.name_es(), "Escalera de Color");
        assert_eq!(HandCategory::StraightFlush.name_en(), "Straight Flush");
        assert_eq!(HandCategory::FourOfAKind.name_es(), "Poker");
        assert_eq!(HandCategory::FullHouse.name_es(), "Full");
    }
}

