//! Tipos de datos para el parser de Winamax.
//!
//! Define todas las estructuras y enumeraciones necesarias para representar
//! una mano de poker parseada, incluyendo acciones, jugadores y metadatos.

use serde::{Deserialize, Serialize};

/// Estado actual del parser FSM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParserState {
    /// Esperando inicio de una nueva mano.
    #[default]
    Initial,
    /// Parseando cabecera (formato, mesa, timestamp).
    Header,
    /// Procesando información de asientos.
    Seats,
    /// Procesando ciegas y antes.
    Blinds,
    /// Procesando acciones preflop.
    Preflop,
    /// Procesando acciones en el flop.
    Flop,
    /// Procesando acciones en el turn.
    Turn,
    /// Procesando acciones en el river.
    River,
    /// Procesando showdown.
    Showdown,
    /// Extrayendo resultados y resumen.
    Summary,
}

/// Tipo de juego detectado en el historial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameType {
    /// Cash Game (dinero real o play money).
    CashGame,
    /// Torneo o Expresso.
    Tournament,
}

/// Calle de la mano de poker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

/// Tipo de acción de poker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// Poner ciega pequeña.
    PostSmallBlind,
    /// Poner ciega grande.
    PostBigBlind,
    /// Poner ante.
    PostAnte,
    /// No apostar (check).
    Check,
    /// Igualar apuesta (call).
    Call,
    /// Apostar (bet).
    Bet,
    /// Subir (raise).
    Raise,
    /// Retirarse (fold).
    Fold,
    /// All-in.
    AllIn,
    /// Mostrar cartas en showdown.
    Show,
    /// Ganar bote.
    Collect,
}

/// Posición en la mesa (6-max).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    /// Botón (dealer).
    Button,
    /// Ciega pequeña.
    SmallBlind,
    /// Ciega grande.
    BigBlind,
    /// Under The Gun (primera posición).
    UTG,
    /// Middle Position.
    MP,
    /// Cut-Off (antes del botón).
    CO,
}

/// Representa una carta de poker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    /// Rango: 2-9, T, J, Q, K, A.
    pub rank: char,
    /// Palo: h (hearts), d (diamonds), c (clubs), s (spades).
    pub suit: char,
}

impl Card {
    /// Parsea una carta desde un string como "Ah" o "Td".
    pub fn parse(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 2 {
            Some(Card {
                rank: chars[0],
                suit: chars[1],
            })
        } else {
            None
        }
    }
}

/// Información de un jugador en la mano.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Nombre del jugador.
    pub name: String,
    /// Número de asiento (1-6).
    pub seat: u8,
    /// Stack inicial en centavos.
    pub stack_cents: i64,
    /// Posición calculada en la mesa.
    pub position: Option<Position>,
    /// Cartas del jugador (si son conocidas).
    pub hole_cards: Option<[Card; 2]>,
    /// Si es el héroe (thesmoy).
    pub is_hero: bool,
}

/// Una acción individual en la mano.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Nombre del jugador que realiza la acción.
    pub player_name: String,
    /// Tipo de acción.
    pub action_type: ActionType,
    /// Cantidad en centavos (si aplica).
    pub amount_cents: Option<i64>,
    /// Si el jugador queda all-in con esta acción.
    pub is_all_in: bool,
    /// Calle donde ocurre la acción.
    pub street: Street,
}

/// Información del bote y resultados.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotInfo {
    /// Bote total en centavos.
    pub total_cents: i64,
    /// Rake en centavos.
    pub rake_cents: i64,
    /// Ganadores y sus ganancias (nombre, cantidad en centavos).
    pub winners: Vec<(String, i64)>,
}

/// Representa una mano de poker completa parseada.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedHand {
    /// ID único de la mano (del historial de Winamax).
    pub hand_id: String,
    /// Tipo de juego (Cash/Tournament).
    pub game_type: GameType,
    /// Nombre de la mesa.
    pub table_name: String,
    /// Número máximo de jugadores (5-max, 6-max).
    pub max_players: u8,
    /// Ciega pequeña en centavos.
    pub small_blind_cents: i64,
    /// Ciega grande en centavos.
    pub big_blind_cents: i64,
    /// Timestamp de la mano (formato UTC).
    pub timestamp: String,
    /// Asiento del botón.
    pub button_seat: u8,
    /// Lista de jugadores en la mano.
    pub players: Vec<Player>,
    /// Cartas del héroe (thesmoy).
    pub hero_cards: Option<[Card; 2]>,
    /// Cartas comunitarias en el board.
    pub board: Vec<Card>,
    /// Acciones de la mano organizadas por calle.
    pub actions: Vec<Action>,
    /// Información del bote y resultados.
    pub pot: PotInfo,
    /// Si es dinero de juego (play money).
    pub is_play_money: bool,
}

impl Default for ParsedHand {
    fn default() -> Self {
        Self {
            hand_id: String::new(),
            game_type: GameType::CashGame,
            table_name: String::new(),
            max_players: 6,
            small_blind_cents: 0,
            big_blind_cents: 0,
            timestamp: String::new(),
            button_seat: 0,
            players: Vec::new(),
            hero_cards: None,
            board: Vec::new(),
            actions: Vec::new(),
            pot: PotInfo {
                total_cents: 0,
                rake_cents: 0,
                winners: Vec::new(),
            },
            is_play_money: false,
        }
    }
}

/// Resultado del parsing de un archivo completo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    /// Manos parseadas exitosamente.
    pub hands: Vec<ParsedHand>,
    /// Número de manos con errores.
    pub error_count: usize,
    /// Mensajes de error (si los hay).
    pub errors: Vec<String>,
}

