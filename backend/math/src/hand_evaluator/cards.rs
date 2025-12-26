//! # Representación de Cartas
//!
//! Sistema de representación de cartas optimizado para evaluación rápida.
//! Usa representación de 32 bits compatible con el algoritmo Cactus Kev.
//!
//! ## Formato de Carta (32 bits)
//! ```text
//! +--------+--------+--------+--------+
//! |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
//! +--------+--------+--------+--------+
//!
//! p = número primo del rank (2=2, 3=3, 5=5, 7=7, 11=B, 13=D, 17=Q, 19=K, 23=A, etc.)
//! r = rank de la carta (2-14, donde 14=As)
//! cdhs = bits de suit (exactamente uno activo)
//! b = bit pattern para detección de straight/flush
//! ```

use std::fmt;

/// Los 13 números primos asignados a cada rank (2-A)
/// Usados para multiplicación rápida en detección de manos
pub const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// Representación de un rank de carta (2-A)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Rank {
    /// Crea un Rank desde un valor numérico (0-12)
    #[inline]
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Rank::Two),
            1 => Some(Rank::Three),
            2 => Some(Rank::Four),
            3 => Some(Rank::Five),
            4 => Some(Rank::Six),
            5 => Some(Rank::Seven),
            6 => Some(Rank::Eight),
            7 => Some(Rank::Nine),
            8 => Some(Rank::Ten),
            9 => Some(Rank::Jack),
            10 => Some(Rank::Queen),
            11 => Some(Rank::King),
            12 => Some(Rank::Ace),
            _ => None,
        }
    }

    /// Crea un Rank desde un caracter ('2'-'9', 'T', 'J', 'Q', 'K', 'A')
    #[inline]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            '9' => Some(Rank::Nine),
            'T' | 't' => Some(Rank::Ten),
            'J' | 'j' => Some(Rank::Jack),
            'Q' | 'q' => Some(Rank::Queen),
            'K' | 'k' => Some(Rank::King),
            'A' | 'a' => Some(Rank::Ace),
            _ => None,
        }
    }

    /// Devuelve el índice numérico (0-12)
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Devuelve el número primo asociado
    #[inline]
    pub const fn prime(self) -> u32 {
        PRIMES[self as usize]
    }

    /// Devuelve el caracter representativo
    #[inline]
    pub const fn to_char(self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Representación de un palo de carta
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Suit {
    Clubs = 0,    // Tréboles
    Diamonds = 1, // Diamantes
    Hearts = 2,   // Corazones
    Spades = 3,   // Picas
}

impl Suit {
    /// Crea un Suit desde un índice (0-3)
    #[inline]
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Suit::Clubs),
            1 => Some(Suit::Diamonds),
            2 => Some(Suit::Hearts),
            3 => Some(Suit::Spades),
            _ => None,
        }
    }

    /// Crea un Suit desde un caracter ('c', 'd', 'h', 's')
    #[inline]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'c' | 'C' => Some(Suit::Clubs),
            'd' | 'D' => Some(Suit::Diamonds),
            'h' | 'H' => Some(Suit::Hearts),
            's' | 'S' => Some(Suit::Spades),
            _ => None,
        }
    }

    /// Devuelve el índice (0-3)
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Devuelve el bit mask para este palo (usado en representación de 32 bits)
    #[inline]
    pub const fn bit_mask(self) -> u32 {
        1 << (self as u32 + 12)
    }

    /// Devuelve el caracter representativo
    #[inline]
    pub const fn to_char(self) -> char {
        match self {
            Suit::Clubs => 'c',
            Suit::Diamonds => 'd',
            Suit::Hearts => 'h',
            Suit::Spades => 's',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Representación de una carta de poker (32 bits)
///
/// Formato interno optimizado para evaluación rápida:
/// - Bits 0-5: Número primo del rank
/// - Bits 8-11: Rank (0-12)
/// - Bits 12-15: Suit bit (exactamente uno activo)
/// - Bits 16-28: Bit pattern para detección de straights
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card(u32);

impl Card {
    /// Crea una nueva carta desde rank y suit
    #[inline]
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let prime = PRIMES[rank as usize];
        let rank_bits = (rank as u32) << 8;
        let suit_bit = 1u32 << (suit as u32 + 12);
        let rank_bit = 1u32 << (rank as u32 + 16);

        Card(prime | rank_bits | suit_bit | rank_bit)
    }

    /// Crea una carta desde un string ("As", "Kh", "2c", etc.)
    pub fn from_str(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let rank_char = chars.next()?;
        let suit_char = chars.next()?;

        let rank = Rank::from_char(rank_char)?;
        let suit = Suit::from_char(suit_char)?;

        Some(Card::new(rank, suit))
    }

    /// Crea una carta desde un índice (0-51)
    #[inline]
    pub const fn from_index(index: u8) -> Option<Card> {
        if index >= 52 {
            return None;
        }
        let rank_idx = index % 13;
        let suit_idx = index / 13;

        // Necesitamos unwrap manual porque no podemos usar ? en const fn
        let rank = match Rank::from_index(rank_idx) {
            Some(r) => r,
            None => return None,
        };
        let suit = match Suit::from_index(suit_idx) {
            Some(s) => s,
            None => return None,
        };

        Some(Card::new(rank, suit))
    }

    /// Devuelve el índice único (0-51)
    #[inline]
    pub const fn index(self) -> u8 {
        let rank = self.rank().index();
        let suit = self.suit_index();
        suit * 13 + rank
    }

    /// Devuelve el rank de la carta
    #[inline]
    pub const fn rank(self) -> Rank {
        let rank_bits = ((self.0 >> 8) & 0x0F) as u8;
        // Safe porque sabemos que rank_bits está en 0-12
        match Rank::from_index(rank_bits) {
            Some(r) => r,
            None => Rank::Two, // Fallback, nunca debería ocurrir
        }
    }

    /// Devuelve el índice del suit (0-3)
    #[inline]
    const fn suit_index(self) -> u8 {
        let suit_bits = (self.0 >> 12) & 0x0F;
        match suit_bits {
            0x1 => 0, // Clubs
            0x2 => 1, // Diamonds
            0x4 => 2, // Hearts
            0x8 => 3, // Spades
            _ => 0,   // Fallback
        }
    }

    /// Devuelve el suit de la carta
    #[inline]
    pub const fn suit(self) -> Suit {
        match Suit::from_index(self.suit_index()) {
            Some(s) => s,
            None => Suit::Clubs, // Fallback
        }
    }

    /// Devuelve el valor interno de 32 bits (para evaluación)
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Devuelve el número primo del rank
    #[inline]
    pub const fn prime(self) -> u32 {
        self.0 & 0x3F
    }

    /// Devuelve el bit de rank (para detección de straights)
    #[inline]
    pub const fn rank_bit(self) -> u32 {
        (self.0 >> 16) & 0x1FFF
    }

    /// Devuelve el bit de suit
    #[inline]
    pub const fn suit_bit(self) -> u32 {
        (self.0 >> 12) & 0x0F
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card({}{})", self.rank(), self.suit())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

/// Una baraja de 52 cartas
#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
    position: usize,
}

impl Deck {
    /// Crea una baraja nueva ordenada
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit_idx in 0..4 {
            for rank_idx in 0..13 {
                if let (Some(rank), Some(suit)) =
                    (Rank::from_index(rank_idx), Suit::from_index(suit_idx))
                {
                    cards.push(Card::new(rank, suit));
                }
            }
        }
        Deck { cards, position: 0 }
    }

    /// Baraja las cartas usando Fisher-Yates
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        self.position = 0;
    }

    /// Baraja con un generador específico (para reproducibilidad)
    pub fn shuffle_with_rng<R: rand::Rng>(&mut self, rng: &mut R) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(rng);
        self.position = 0;
    }

    /// Reparte una carta
    pub fn deal(&mut self) -> Option<Card> {
        if self.position < 52 {
            let card = self.cards[self.position];
            self.position += 1;
            Some(card)
        } else {
            None
        }
    }

    /// Reparte múltiples cartas
    pub fn deal_n(&mut self, n: usize) -> Vec<Card> {
        let mut cards = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(card) = self.deal() {
                cards.push(card);
            }
        }
        cards
    }

    /// Reinicia la baraja (sin barajar)
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Cartas restantes
    pub fn remaining(&self) -> usize {
        52 - self.position
    }

    /// Elimina cartas específicas de la baraja (para simulaciones)
    pub fn remove_cards(&mut self, cards_to_remove: &[Card]) {
        self.cards.retain(|card| !cards_to_remove.contains(card));
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

/// Lookup table de todas las 52 cartas pre-calculadas
pub static CARDS: [Card; 52] = {
    let mut cards = [Card(0); 52];
    let mut i = 0u8;
    while i < 52 {
        if let Some(card) = Card::from_index(i) {
            cards[i as usize] = card;
        }
        i += 1;
    }
    cards
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_from_char() {
        assert_eq!(Rank::from_char('A'), Some(Rank::Ace));
        assert_eq!(Rank::from_char('K'), Some(Rank::King));
        assert_eq!(Rank::from_char('T'), Some(Rank::Ten));
        assert_eq!(Rank::from_char('2'), Some(Rank::Two));
        assert_eq!(Rank::from_char('X'), None);
    }

    #[test]
    fn test_suit_from_char() {
        assert_eq!(Suit::from_char('s'), Some(Suit::Spades));
        assert_eq!(Suit::from_char('h'), Some(Suit::Hearts));
        assert_eq!(Suit::from_char('d'), Some(Suit::Diamonds));
        assert_eq!(Suit::from_char('c'), Some(Suit::Clubs));
        assert_eq!(Suit::from_char('x'), None);
    }

    #[test]
    fn test_card_creation() {
        let ace_spades = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(ace_spades.rank(), Rank::Ace);
        assert_eq!(ace_spades.suit(), Suit::Spades);
    }

    #[test]
    fn test_card_from_str() {
        let card = Card::from_str("As").unwrap();
        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Spades);

        let card = Card::from_str("2c").unwrap();
        assert_eq!(card.rank(), Rank::Two);
        assert_eq!(card.suit(), Suit::Clubs);

        let card = Card::from_str("Th").unwrap();
        assert_eq!(card.rank(), Rank::Ten);
        assert_eq!(card.suit(), Suit::Hearts);
    }

    #[test]
    fn test_card_index_roundtrip() {
        for i in 0..52 {
            let card = Card::from_index(i).unwrap();
            assert_eq!(card.index(), i);
        }
    }

    #[test]
    fn test_card_display() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(format!("{}", card), "As");

        let card = Card::new(Rank::Ten, Suit::Hearts);
        assert_eq!(format!("{}", card), "Th");
    }

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deck_deal() {
        let mut deck = Deck::new();
        let card = deck.deal().unwrap();
        assert_eq!(deck.remaining(), 51);
        // Primera carta debe ser 2c (índice 0)
        assert_eq!(card.rank(), Rank::Two);
        assert_eq!(card.suit(), Suit::Clubs);
    }

    #[test]
    fn test_cards_lookup() {
        // Verificar que la lookup table tiene todas las cartas
        for i in 0..52 {
            let card = CARDS[i];
            assert_eq!(card.index(), i as u8);
        }
    }

    #[test]
    fn test_card_primes() {
        // Verificar que cada carta tiene el prime correcto
        let two_clubs = Card::new(Rank::Two, Suit::Clubs);
        assert_eq!(two_clubs.prime(), 2);

        let ace_spades = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(ace_spades.prime(), 41);
    }
}

