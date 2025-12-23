//! # Schema Module
//!
//! Definiciones de estructuras Rust que mapean al Star Schema de DuckDB.
//!
//! ## Arquitectura
//! - **Dimension Tables**: `Player`, `HandMetadata`
//! - **Fact Tables**: `HandAction`
//! - **Result Tables**: `CashSession`, `Tournament`, `TournamentResult`
//!
//! ## Tipos de Datos
//! - Cantidades monetarias: `i64` (centavos enteros)
//! - Identificadores: `String` para IDs externos, `uuid::Uuid` para IDs internos
//! - Timestamps: `chrono::NaiveDateTime` (UTC)

use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// 1. IDENTIDAD Y JUGADORES
// ============================================================================

/// Tabla: players
/// Consolidación de identidad única para el usuario y oponentes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String, // UUID as String
    pub display_name: String,
    pub is_hero: bool,
    pub notes: Option<String>,
    pub created_at: String, // ISO 8601 timestamp
    pub updated_at: String,
}

impl Player {
    /// Crea un nuevo jugador
    pub fn new(display_name: String, is_hero: bool) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            player_id: uuid::Uuid::new_v4().to_string(),
            display_name,
            is_hero,
            notes: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Crea el jugador Hero (thesmoy)
    pub fn create_hero(display_name: String) -> Self {
        Self::new(display_name, true)
    }
}

/// Tabla: player_aliases
/// Gestión de múltiples nicknames y cuentas (multi-sala)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAlias {
    pub alias_id: String,
    pub player_id: String,
    pub site_name: SiteName,
    pub site_nickname: String,
    pub created_at: String,
}

impl PlayerAlias {
    /// Crea un nuevo alias para un jugador
    pub fn new(player_id: String, site_name: SiteName, site_nickname: String) -> Self {
        Self {
            alias_id: uuid::Uuid::new_v4().to_string(),
            player_id,
            site_name,
            site_nickname,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Enum: Sitios de poker soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiteName {
    Winamax,
    PokerStars,
    GGPoker,
    PartyPoker,
}

impl fmt::Display for SiteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiteName::Winamax => write!(f, "Winamax"),
            SiteName::PokerStars => write!(f, "PokerStars"),
            SiteName::GGPoker => write!(f, "GGPoker"),
            SiteName::PartyPoker => write!(f, "PartyPoker"),
        }
    }
}

// ============================================================================
// 2. ESTRUCTURA ANALÍTICA (STAR SCHEMA)
// ============================================================================

/// Tabla: hands_metadata (DIMENSION TABLE)
/// Metadata de cada mano para análisis dimensional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandMetadata {
    pub hand_id: String,
    pub session_id: Option<String>,
    pub tournament_id: Option<String>,
    pub timestamp: String, // ISO 8601
    pub stake: String,
    pub format: GameFormat,
    pub table_name: String,
    pub blind_level: i64, // SB en centavos
    pub button_seat: u8,   // 0-5 para 6-max
    pub created_at: String,
}

impl HandMetadata {
    /// Crea metadata para una mano de cash game
    pub fn new_cash(
        hand_id: String,
        session_id: String,
        timestamp: String,
        stake: String,
        table_name: String,
        blind_level: i64,
        button_seat: u8,
    ) -> Self {
        Self {
            hand_id,
            session_id: Some(session_id),
            tournament_id: None,
            timestamp,
            stake,
            format: GameFormat::Cash,
            table_name,
            blind_level,
            button_seat,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Crea metadata para una mano de torneo
    pub fn new_tournament(
        hand_id: String,
        tournament_id: String,
        timestamp: String,
        stake: String,
        format: GameFormat,
        table_name: String,
        blind_level: i64,
        button_seat: u8,
    ) -> Self {
        Self {
            hand_id,
            session_id: None,
            tournament_id: Some(tournament_id),
            timestamp,
            stake,
            format,
            table_name,
            blind_level,
            button_seat,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Enum: Formato de juego
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameFormat {
    Cash,
    MTT,
    SNG,
    Expresso,
}

impl fmt::Display for GameFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameFormat::Cash => write!(f, "CASH"),
            GameFormat::MTT => write!(f, "MTT"),
            GameFormat::SNG => write!(f, "SNG"),
            GameFormat::Expresso => write!(f, "EXPRESSO"),
        }
    }
}

/// Tabla: hands_actions (FACT TABLE)
/// Tabla de hechos con todas las acciones de todas las manos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandAction {
    pub action_id: String,
    pub hand_id: String,
    pub player_id: String,
    pub street: Street,
    pub action_type: ActionType,
    pub amount_cents: i64,
    pub is_hero_action: bool,
    pub ev_cents: Option<i64>,
    pub action_sequence: i32,
    pub created_at: String,
}

impl HandAction {
    /// Crea una nueva acción
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hand_id: String,
        player_id: String,
        street: Street,
        action_type: ActionType,
        amount_cents: i64,
        is_hero_action: bool,
        action_sequence: i32,
    ) -> Self {
        Self {
            action_id: uuid::Uuid::new_v4().to_string(),
            hand_id,
            player_id,
            street,
            action_type,
            amount_cents,
            is_hero_action,
            ev_cents: None,
            action_sequence,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Enum: Street (calle del juego)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Street::Preflop => write!(f, "PREFLOP"),
            Street::Flop => write!(f, "FLOP"),
            Street::Turn => write!(f, "TURN"),
            Street::River => write!(f, "RIVER"),
        }
    }
}

/// Enum: Tipo de acción
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Fold,
    Call,
    Raise,
    Bet,
    Check,
    AllIn,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Fold => write!(f, "FOLD"),
            ActionType::Call => write!(f, "CALL"),
            ActionType::Raise => write!(f, "RAISE"),
            ActionType::Bet => write!(f, "BET"),
            ActionType::Check => write!(f, "CHECK"),
            ActionType::AllIn => write!(f, "ALL_IN"),
        }
    }
}

// ============================================================================
// 3. ECONOMÍA Y RESULTADOS
// ============================================================================

/// Tabla: cash_sessions
/// Sesiones de cash game con métricas de rendimiento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashSession {
    pub session_id: String,
    pub player_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub stake: String,
    pub net_won_cents: i64,
    pub ev_won_cents: i64,
    pub rake_cents: i64,
    pub rakeback_cents: i64,
    pub bb_100: Option<f64>,
    pub ev_bb_100: Option<f64>,
    pub hands_played: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl CashSession {
    /// Crea una nueva sesión de cash game
    pub fn new(player_id: String, start_time: String, stake: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            player_id,
            start_time,
            end_time: None,
            stake,
            net_won_cents: 0,
            ev_won_cents: 0,
            rake_cents: 0,
            rakeback_cents: 0,
            bb_100: None,
            ev_bb_100: None,
            hands_played: 0,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Calcula el bb/100 basado en el blind level
    pub fn calculate_bb_100(&mut self, blind_level: i64) {
        if self.hands_played > 0 && blind_level > 0 {
            let bb = blind_level * 2;
            self.bb_100 = Some((self.net_won_cents as f64 / bb as f64) / self.hands_played as f64 * 100.0);
            self.ev_bb_100 = Some((self.ev_won_cents as f64 / bb as f64) / self.hands_played as f64 * 100.0);
        }
    }
}

/// Tabla: tournaments
/// Información de torneos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub tournament_id: String,
    pub tournament_name: String,
    pub format: GameFormat,
    pub buyin_cents: i64,
    pub rake_cents: i64,
    pub total_entries: Option<i32>,
    pub prize_pool_cents: Option<i64>,
    pub start_time: String,
    pub created_at: String,
}

impl Tournament {
    /// Crea un nuevo torneo
    pub fn new(
        tournament_id: String,
        tournament_name: String,
        format: GameFormat,
        buyin_cents: i64,
        rake_cents: i64,
        start_time: String,
    ) -> Self {
        Self {
            tournament_id,
            tournament_name,
            format,
            buyin_cents,
            rake_cents,
            total_entries: None,
            prize_pool_cents: None,
            start_time,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Tabla: tournament_results
/// Resultados de torneos para cálculo de ROI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentResult {
    pub result_id: String,
    pub tournament_id: String,
    pub player_id: String,
    pub finish_position: i32,
    pub prize_won_cents: i64,
    pub bounty_won_cents: i64,
    pub total_won_cents: i64,
    pub roi_real: Option<f64>,
    pub created_at: String,
}

impl TournamentResult {
    /// Crea un nuevo resultado de torneo
    pub fn new(
        tournament_id: String,
        player_id: String,
        finish_position: i32,
        prize_won_cents: i64,
        bounty_won_cents: i64,
    ) -> Self {
        let total_won_cents = prize_won_cents + bounty_won_cents;
        Self {
            result_id: uuid::Uuid::new_v4().to_string(),
            tournament_id,
            player_id,
            finish_position,
            prize_won_cents,
            bounty_won_cents,
            total_won_cents,
            roi_real: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Calcula el ROI real considerando bounties
    pub fn calculate_roi(&mut self, buyin_cents: i64, rake_cents: i64) {
        let total_invested = buyin_cents + rake_cents;
        if total_invested > 0 {
            self.roi_real = Some(
                ((self.total_won_cents - total_invested) as f64 / total_invested as f64) * 100.0,
            );
        }
    }
}

// ============================================================================
// 4. BUILDERS Y HELPERS
// ============================================================================

/// Builder para HandMetadata
pub struct HandMetadataBuilder {
    hand_id: String,
    session_id: Option<String>,
    tournament_id: Option<String>,
    timestamp: String,
    stake: String,
    format: GameFormat,
    table_name: String,
    blind_level: i64,
    button_seat: u8,
}

impl HandMetadataBuilder {
    pub fn new(hand_id: String, timestamp: String) -> Self {
        Self {
            hand_id,
            session_id: None,
            tournament_id: None,
            timestamp,
            stake: String::new(),
            format: GameFormat::Cash,
            table_name: String::new(),
            blind_level: 0,
            button_seat: 0,
        }
    }

    pub fn session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn tournament_id(mut self, tournament_id: String) -> Self {
        self.tournament_id = Some(tournament_id);
        self
    }

    pub fn stake(mut self, stake: String) -> Self {
        self.stake = stake;
        self
    }

    pub fn format(mut self, format: GameFormat) -> Self {
        self.format = format;
        self
    }

    pub fn table_name(mut self, table_name: String) -> Self {
        self.table_name = table_name;
        self
    }

    pub fn blind_level(mut self, blind_level: i64) -> Self {
        self.blind_level = blind_level;
        self
    }

    pub fn button_seat(mut self, button_seat: u8) -> Self {
        self.button_seat = button_seat;
        self
    }

    pub fn build(self) -> HandMetadata {
        HandMetadata {
            hand_id: self.hand_id,
            session_id: self.session_id,
            tournament_id: self.tournament_id,
            timestamp: self.timestamp,
            stake: self.stake,
            format: self.format,
            table_name: self.table_name,
            blind_level: self.blind_level,
            button_seat: self.button_seat,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hero_player() {
        let hero = Player::create_hero("thesmoy".to_string());
        assert!(hero.is_hero);
        assert_eq!(hero.display_name, "thesmoy");
    }

    #[test]
    fn test_create_hand_metadata_cash() {
        let metadata = HandMetadata::new_cash(
            "HAND123".to_string(),
            "SESSION456".to_string(),
            "2024-01-15T10:30:00Z".to_string(),
            "NL10".to_string(),
            "Table 1".to_string(),
            5,
            2,
        );
        assert_eq!(metadata.format, GameFormat::Cash);
        assert!(metadata.session_id.is_some());
        assert!(metadata.tournament_id.is_none());
    }

    #[test]
    fn test_create_hand_action() {
        let action = HandAction::new(
            "HAND123".to_string(),
            "PLAYER456".to_string(),
            Street::Preflop,
            ActionType::Raise,
            30,
            true,
            1,
        );
        assert_eq!(action.street, Street::Preflop);
        assert_eq!(action.action_type, ActionType::Raise);
        assert!(action.is_hero_action);
    }

    #[test]
    fn test_calculate_bb_100() {
        let mut session = CashSession::new(
            "PLAYER123".to_string(),
            "2024-01-15T10:00:00Z".to_string(),
            "NL10".to_string(),
        );
        session.net_won_cents = 1000;
        session.hands_played = 100;
        session.calculate_bb_100(5); // 5 cents SB = 10 cents BB
        
        assert!(session.bb_100.is_some());
        // (1000 cents / 10 BB) / 100 hands * 100 = 100 BB / 100 hands * 100 = 100.0 bb/100
        assert_eq!(session.bb_100.unwrap(), 100.0);
    }

    #[test]
    fn test_calculate_roi() {
        let mut result = TournamentResult::new(
            "TOURNEY123".to_string(),
            "PLAYER456".to_string(),
            1,
            5000,
            1000,
        );
        result.calculate_roi(1000, 100); // 10€ buyin + 1€ rake
        
        assert!(result.roi_real.is_some());
        // (6000 - 1100) / 1100 * 100 = 445.45%
        assert!((result.roi_real.unwrap() - 445.45).abs() < 0.1);
    }

    #[test]
    fn test_hand_metadata_builder() {
        let metadata = HandMetadataBuilder::new(
            "HAND789".to_string(),
            "2024-01-15T12:00:00Z".to_string(),
        )
        .stake("NL50".to_string())
        .format(GameFormat::Cash)
        .table_name("High Stakes".to_string())
        .blind_level(25)
        .button_seat(3)
        .build();

        assert_eq!(metadata.stake, "NL50");
        assert_eq!(metadata.blind_level, 25);
        assert_eq!(metadata.button_seat, 3);
    }
}

