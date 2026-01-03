//! # Synthetic Hand Generator
//!
//! Generador de manos sinteticas de poker para pruebas de carga.
//!
//! ## Caracteristicas
//! - Generacion determinista con semilla (reproducible)
//! - Paralelizacion con Rayon (16 threads en Ryzen 3800X)
//! - Distribucion realista de acciones (inspirado en GTO)
//! - Soporte para multiples stakes (NL2, NL5, NL10, NL25, NL50)
//! - Compatible con esquema DuckDB (hands_metadata, hands_actions)
//!
//! ## Objetivo de Rendimiento
//! - 1M manos en < 60 segundos
//!
//! ## Uso
//!
//! ```rust,no_run
//! use poker_parsers::synthetic_generator::{SyntheticConfig, SyntheticGenerator};
//!
//! let config = SyntheticConfig::new(1_000_000)
//!     .with_seed(42)
//!     .with_stakes(vec!["NL10".to_string()]);
//!
//! let generator = SyntheticGenerator::new(config);
//! let hands = generator.generate_parallel();
//!
//! println!("Generated {} hands", hands.len());
//! ```

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

use crate::types::{
    Action, ActionType, Card, GameType, ParsedHand, Player, Position, PotInfo, Street,
};

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuracion para el generador de manos sinteticas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntheticConfig {
    /// Numero total de manos a generar
    pub num_hands: usize,

    /// Semilla para reproducibilidad (None = aleatorio)
    pub seed: Option<u64>,

    /// Stakes disponibles para generar
    pub stakes: Vec<StakeLevel>,

    /// Rango de fechas para las manos
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

    /// Numero de jugadores por mesa (default: 6)
    pub max_players: u8,

    /// Nombre del heroe (default: "thesmoy")
    pub hero_name: String,

    /// Probabilidad de que el heroe este en la mano (0.0 - 1.0)
    pub hero_presence_rate: f64,
}

impl Default for SyntheticConfig {
    fn default() -> Self {
        Self {
            num_hands: 10_000,
            seed: None,
            stakes: vec![
                StakeLevel::NL2,
                StakeLevel::NL5,
                StakeLevel::NL10,
                StakeLevel::NL25,
                StakeLevel::NL50,
            ],
            start_date: Utc::now() - Duration::days(365),
            end_date: Utc::now(),
            max_players: 6,
            hero_name: "thesmoy".to_string(),
            hero_presence_rate: 0.85,
        }
    }
}

impl SyntheticConfig {
    /// Crea una nueva configuracion con numero de manos especificado
    pub fn new(num_hands: usize) -> Self {
        Self {
            num_hands,
            ..Default::default()
        }
    }

    /// Establece la semilla para reproducibilidad
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Establece los stakes a generar
    pub fn with_stakes(mut self, stakes: Vec<String>) -> Self {
        self.stakes = stakes
            .iter()
            .filter_map(|s| StakeLevel::from_str(s))
            .collect();
        if self.stakes.is_empty() {
            self.stakes = vec![StakeLevel::NL10];
        }
        self
    }

    /// Establece el rango de fechas
    pub fn with_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_date = start;
        self.end_date = end;
        self
    }

    /// Establece el nombre del heroe
    pub fn with_hero_name(mut self, name: String) -> Self {
        self.hero_name = name;
        self
    }
}

// ============================================================================
// STAKE LEVELS
// ============================================================================

/// Niveles de stake soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StakeLevel {
    NL2,   // 0.01/0.02
    NL5,   // 0.02/0.05
    NL10,  // 0.05/0.10
    NL25,  // 0.10/0.25
    NL50,  // 0.25/0.50
    NL100, // 0.50/1.00
}

impl StakeLevel {
    /// Convierte string a StakeLevel
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NL2" => Some(StakeLevel::NL2),
            "NL5" => Some(StakeLevel::NL5),
            "NL10" => Some(StakeLevel::NL10),
            "NL25" => Some(StakeLevel::NL25),
            "NL50" => Some(StakeLevel::NL50),
            "NL100" => Some(StakeLevel::NL100),
            _ => None,
        }
    }

    /// Small blind en centavos
    pub fn small_blind_cents(&self) -> i64 {
        match self {
            StakeLevel::NL2 => 1,
            StakeLevel::NL5 => 2,
            StakeLevel::NL10 => 5,
            StakeLevel::NL25 => 10,
            StakeLevel::NL50 => 25,
            StakeLevel::NL100 => 50,
        }
    }

    /// Big blind en centavos
    pub fn big_blind_cents(&self) -> i64 {
        self.small_blind_cents() * 2
    }

    /// Stack tipico en centavos (100BB)
    pub fn typical_stack_cents(&self) -> i64 {
        self.big_blind_cents() * 100
    }

    /// Nombre del stake
    pub fn name(&self) -> &'static str {
        match self {
            StakeLevel::NL2 => "NL2",
            StakeLevel::NL5 => "NL5",
            StakeLevel::NL10 => "NL10",
            StakeLevel::NL25 => "NL25",
            StakeLevel::NL50 => "NL50",
            StakeLevel::NL100 => "NL100",
        }
    }
}

// ============================================================================
// ACTION DISTRIBUTIONS (GTO-inspired)
// ============================================================================

/// Distribucion de acciones preflop
#[derive(Debug, Clone)]
struct PreflopDistribution {
    // Pesos: [Fold, Call, Raise]
    rfi_weights: [u32; 3],      // Raise First In
    vs_raise_weights: [u32; 3], // Facing raise
    vs_3bet_weights: [u32; 3],  // Facing 3-bet
}

impl Default for PreflopDistribution {
    fn default() -> Self {
        Self {
            // RFI: Open 25%, Limp 5%, Fold 70%
            rfi_weights: [70, 5, 25],
            // vs Raise: Fold 55%, Call 25%, 3bet 20%
            vs_raise_weights: [55, 25, 20],
            // vs 3bet: Fold 60%, Call 25%, 4bet 15%
            vs_3bet_weights: [60, 25, 15],
        }
    }
}

/// Distribucion de acciones postflop
#[derive(Debug, Clone)]
struct PostflopDistribution {
    // Pesos: [Check, Bet, Call, Raise, Fold]
    ip_no_bet_weights: [u32; 5],      // In position, no bet
    ip_facing_bet_weights: [u32; 5],  // In position, facing bet
    oop_no_bet_weights: [u32; 5],     // Out of position, no bet
    oop_facing_bet_weights: [u32; 5], // Out of position, facing bet
}

impl Default for PostflopDistribution {
    fn default() -> Self {
        Self {
            // IP sin bet: Check 55%, Bet 45%
            ip_no_bet_weights: [55, 45, 0, 0, 0],
            // IP facing bet: Fold 35%, Call 45%, Raise 20%
            ip_facing_bet_weights: [0, 0, 45, 20, 35],
            // OOP sin bet: Check 65%, Bet 35%
            oop_no_bet_weights: [65, 35, 0, 0, 0],
            // OOP facing bet: Fold 40%, Call 42%, Raise 18%
            oop_facing_bet_weights: [0, 0, 42, 18, 40],
        }
    }
}

// ============================================================================
// SYNTHETIC GENERATOR
// ============================================================================

/// Generador de manos sinteticas de poker
pub struct SyntheticGenerator {
    config: SyntheticConfig,
    preflop_dist: PreflopDistribution,
    postflop_dist: PostflopDistribution,
}

impl SyntheticGenerator {
    /// Crea un nuevo generador con la configuracion especificada
    pub fn new(config: SyntheticConfig) -> Self {
        Self {
            config,
            preflop_dist: PreflopDistribution::default(),
            postflop_dist: PostflopDistribution::default(),
        }
    }

    /// Genera manos en paralelo usando Rayon
    ///
    /// Cada thread genera un batch independiente con su propia semilla derivada
    pub fn generate_parallel(&self) -> Vec<ParsedHand> {
        let num_threads = rayon::current_num_threads();
        let hands_per_thread = self.config.num_hands / num_threads;
        let remainder = self.config.num_hands % num_threads;

        let base_seed = self.config.seed.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen()
        });

        let counter = AtomicUsize::new(0);

        // Generar en paralelo
        let hands: Vec<ParsedHand> = (0..num_threads)
            .into_par_iter()
            .flat_map(|thread_id| {
                let thread_hands = if thread_id == 0 {
                    hands_per_thread + remainder
                } else {
                    hands_per_thread
                };

                // Semilla unica por thread
                let thread_seed = base_seed.wrapping_add(thread_id as u64 * 1_000_000);
                let mut rng = ChaCha8Rng::seed_from_u64(thread_seed);

                let mut batch = Vec::with_capacity(thread_hands);
                for _ in 0..thread_hands {
                    let hand_num = counter.fetch_add(1, Ordering::Relaxed);
                    let hand = self.generate_single_hand(&mut rng, hand_num);
                    batch.push(hand);
                }
                batch
            })
            .collect();

        hands
    }

    /// Genera manos secuencialmente (para testing/debug)
    pub fn generate_sequential(&self) -> Vec<ParsedHand> {
        let seed = self.config.seed.unwrap_or(12345);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..self.config.num_hands)
            .map(|i| self.generate_single_hand(&mut rng, i))
            .collect()
    }

    /// Genera una mano individual
    fn generate_single_hand(&self, rng: &mut ChaCha8Rng, hand_num: usize) -> ParsedHand {
        // Seleccionar stake aleatorio
        let stake = self
            .config
            .stakes
            .choose(rng)
            .copied()
            .unwrap_or(StakeLevel::NL10);

        // Generar timestamp aleatorio en el rango
        let timestamp = self.generate_timestamp(rng);

        // Generar ID unico
        let hand_id = format!("SYN-{}-{:010}", stake.name(), hand_num);

        // Generar jugadores
        let (players, button_seat) = self.generate_players(rng, stake);

        // Determinar posiciones
        let players_with_positions = self.assign_positions(players, button_seat);

        // Generar cartas
        let mut deck = self.create_shuffled_deck(rng);
        let (players_with_cards, hero_cards) =
            self.deal_cards(&mut deck, players_with_positions, rng);

        // Generar acciones
        let (actions, pot_total, winners) = self.generate_actions(rng, &players_with_cards, stake);

        // Generar board (si hay acciones postflop)
        let board = self.generate_board(&mut deck, &actions);

        // Calcular rake (5% del bote, max 3BB)
        let rake = (pot_total * 5 / 100).min(stake.big_blind_cents() * 3);

        ParsedHand {
            hand_id,
            game_type: GameType::CashGame,
            table_name: format!("Synthetic Table {}", rng.gen_range(1..=100)),
            max_players: self.config.max_players,
            small_blind_cents: stake.small_blind_cents(),
            big_blind_cents: stake.big_blind_cents(),
            timestamp,
            button_seat,
            players: players_with_cards,
            hero_cards,
            board,
            actions,
            pot: PotInfo {
                total_cents: pot_total,
                rake_cents: rake,
                winners,
            },
            is_play_money: false,
        }
    }

    /// Genera un timestamp aleatorio en el rango configurado
    fn generate_timestamp(&self, rng: &mut ChaCha8Rng) -> String {
        let start_ts = self.config.start_date.timestamp();
        let end_ts = self.config.end_date.timestamp();
        let random_ts = rng.gen_range(start_ts..=end_ts);

        DateTime::from_timestamp(random_ts, 0)
            .map(|dt| dt.format("%Y/%m/%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| Utc::now().format("%Y/%m/%d %H:%M:%S UTC").to_string())
    }

    /// Genera jugadores para la mano
    fn generate_players(&self, rng: &mut ChaCha8Rng, stake: StakeLevel) -> (Vec<Player>, u8) {
        let num_players = rng.gen_range(2..=self.config.max_players);
        let button_seat = rng.gen_range(1..=num_players);

        // Lista de nombres de oponentes
        let opponent_names = [
            "fish123",
            "shark_pro",
            "donkey42",
            "nit_master",
            "lag_wizard",
            "tight_tiger",
            "loose_lucy",
            "aggro_andy",
            "passive_pete",
            "random_rx",
            "grinder99",
            "bluffer_bob",
            "value_vic",
            "pot_control",
            "check_raise_cr",
        ];

        let mut players = Vec::with_capacity(num_players as usize);
        let mut used_seats: Vec<u8> = Vec::new();

        // Decidir si el heroe esta en la mano
        let hero_present = rng.gen_bool(self.config.hero_presence_rate);
        let hero_seat = if hero_present {
            Some(rng.gen_range(1..=num_players))
        } else {
            None
        };

        for i in 0..num_players {
            let seat = loop {
                let s = rng.gen_range(1..=self.config.max_players);
                if !used_seats.contains(&s) {
                    used_seats.push(s);
                    break s;
                }
            };

            let (name, is_hero) =
                if Some(i + 1) == hero_seat.map(|_| i + 1) && hero_present && i == 0 {
                    (self.config.hero_name.clone(), true)
                } else {
                    let name = opponent_names
                        .choose(rng)
                        .unwrap_or(&"opponent")
                        .to_string();
                    (name, false)
                };

            // Stack aleatorio entre 50BB y 200BB
            let stack_bbs = rng.gen_range(50..=200) as i64;
            let stack_cents = stack_bbs * stake.big_blind_cents();

            players.push(Player {
                name,
                seat,
                stack_cents,
                position: None,
                hole_cards: None,
                is_hero,
            });
        }

        // Ordenar por asiento
        players.sort_by_key(|p| p.seat);

        (players, button_seat)
    }

    /// Asigna posiciones a los jugadores (6-max)
    fn assign_positions(&self, mut players: Vec<Player>, button_seat: u8) -> Vec<Player> {
        let num_players = players.len();
        if num_players == 0 {
            return players;
        }

        // Encontrar indice del button
        let btn_idx = players
            .iter()
            .position(|p| p.seat == button_seat)
            .unwrap_or(0);

        // Asignar posiciones en orden desde el button
        let positions_6max = [
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
            Position::UTG,
            Position::MP,
            Position::CO,
        ];

        for (i, player) in players.iter_mut().enumerate() {
            let pos_idx = (i + num_players - btn_idx) % num_players;
            player.position = Some(positions_6max[pos_idx.min(5)]);
        }

        players
    }

    /// Crea y baraja un mazo de cartas
    fn create_shuffled_deck(&self, rng: &mut ChaCha8Rng) -> Vec<Card> {
        let ranks = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        let suits = ['h', 'd', 'c', 's'];

        let mut deck: Vec<Card> = ranks
            .iter()
            .flat_map(|&rank| suits.iter().map(move |&suit| Card { rank, suit }))
            .collect();

        deck.shuffle(rng);
        deck
    }

    /// Reparte cartas a los jugadores
    fn deal_cards(
        &self,
        deck: &mut Vec<Card>,
        mut players: Vec<Player>,
        _rng: &mut ChaCha8Rng,
    ) -> (Vec<Player>, Option<[Card; 2]>) {
        let mut hero_cards = None;

        for player in players.iter_mut() {
            if deck.len() >= 2 {
                let card1 = deck.pop().unwrap();
                let card2 = deck.pop().unwrap();
                let cards = [card1.clone(), card2.clone()];

                if player.is_hero {
                    hero_cards = Some(cards.clone());
                    player.hole_cards = Some(cards);
                }
                // Solo el heroe conoce sus cartas (realismo)
            }
        }

        (players, hero_cards)
    }

    /// Genera el board basado en las acciones
    fn generate_board(&self, deck: &mut Vec<Card>, actions: &[Action]) -> Vec<Card> {
        let mut board = Vec::new();

        // Verificar si llegamos a cada street
        let has_flop = actions.iter().any(|a| a.street == Street::Flop);
        let has_turn = actions.iter().any(|a| a.street == Street::Turn);
        let has_river = actions.iter().any(|a| a.street == Street::River);

        if has_flop && deck.len() >= 3 {
            board.push(deck.pop().unwrap());
            board.push(deck.pop().unwrap());
            board.push(deck.pop().unwrap());
        }

        if has_turn && deck.len() >= 1 {
            board.push(deck.pop().unwrap());
        }

        if has_river && deck.len() >= 1 {
            board.push(deck.pop().unwrap());
        }

        board
    }

    /// Genera acciones realistas para la mano
    fn generate_actions(
        &self,
        rng: &mut ChaCha8Rng,
        players: &[Player],
        stake: StakeLevel,
    ) -> (Vec<Action>, i64, Vec<(String, i64)>) {
        let mut actions = Vec::new();
        let mut pot = 0i64;
        let mut player_bets: Vec<i64> = vec![0; players.len()];
        let mut player_folded: Vec<bool> = vec![false; players.len()];
        let bb = stake.big_blind_cents();
        let sb = stake.small_blind_cents();

        // Encontrar SB y BB
        let sb_idx = players
            .iter()
            .position(|p| p.position == Some(Position::SmallBlind));
        let bb_idx = players
            .iter()
            .position(|p| p.position == Some(Position::BigBlind));

        // Postear ciegas
        if let Some(idx) = sb_idx {
            actions.push(Action {
                player_name: players[idx].name.clone(),
                action_type: ActionType::PostSmallBlind,
                amount_cents: Some(sb),
                is_all_in: false,
                street: Street::Preflop,
            });
            pot += sb;
            player_bets[idx] = sb;
        }

        if let Some(idx) = bb_idx {
            actions.push(Action {
                player_name: players[idx].name.clone(),
                action_type: ActionType::PostBigBlind,
                amount_cents: Some(bb),
                is_all_in: false,
                street: Street::Preflop,
            });
            pot += bb;
            player_bets[idx] = bb;
        }

        // Simular preflop
        let (preflop_actions, preflop_pot, preflop_folded) =
            self.simulate_preflop(rng, players, &player_bets, &player_folded, stake);

        actions.extend(preflop_actions);
        pot += preflop_pot;
        player_folded = preflop_folded;

        // Contar jugadores activos
        let active_players: Vec<usize> = player_folded
            .iter()
            .enumerate()
            .filter(|(_, &folded)| !folded)
            .map(|(i, _)| i)
            .collect();

        // Si hay mas de 1 jugador, simular postflop
        if active_players.len() > 1 {
            for street in [Street::Flop, Street::Turn, Street::River] {
                let active_count = player_folded.iter().filter(|&&f| !f).count();
                if active_count <= 1 {
                    break;
                }

                let (street_actions, street_pot, street_folded) =
                    self.simulate_postflop(rng, players, &player_folded, stake, street);

                actions.extend(street_actions);
                pot += street_pot;
                player_folded = street_folded;
            }
        }

        // Determinar ganador
        let active_players: Vec<&Player> = players
            .iter()
            .zip(player_folded.iter())
            .filter(|(_, &folded)| !folded)
            .map(|(p, _)| p)
            .collect();

        let winner = if active_players.len() == 1 {
            active_players[0].name.clone()
        } else if !active_players.is_empty() {
            // Showdown: elegir ganador aleatorio
            active_players
                .choose(rng)
                .map(|p| p.name.clone())
                .unwrap_or_default()
        } else {
            players.first().map(|p| p.name.clone()).unwrap_or_default()
        };

        let winners = vec![(winner, pot)];

        (actions, pot, winners)
    }

    /// Simula acciones preflop
    fn simulate_preflop(
        &self,
        rng: &mut ChaCha8Rng,
        players: &[Player],
        initial_bets: &[i64],
        initial_folded: &[bool],
        stake: StakeLevel,
    ) -> (Vec<Action>, i64, Vec<bool>) {
        let mut actions = Vec::new();
        let mut pot = 0i64;
        let mut player_bets = initial_bets.to_vec();
        let mut player_folded = initial_folded.to_vec();
        let bb = stake.big_blind_cents();
        let mut current_bet = bb;
        let mut raises_count = 0;

        // Orden de accion preflop: UTG -> MP -> CO -> BTN -> SB -> BB
        let action_order = [
            Position::UTG,
            Position::MP,
            Position::CO,
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
        ];

        for position in action_order.iter() {
            if let Some((idx, player)) = players
                .iter()
                .enumerate()
                .find(|(_, p)| p.position == Some(*position))
            {
                if player_folded[idx] {
                    continue;
                }

                // Determinar accion
                let action_type = if current_bet == bb && raises_count == 0 {
                    // RFI situation
                    self.choose_preflop_action(rng, &self.preflop_dist.rfi_weights)
                } else if raises_count == 1 {
                    // Facing raise
                    self.choose_preflop_action(rng, &self.preflop_dist.vs_raise_weights)
                } else {
                    // Facing 3bet+
                    self.choose_preflop_action(rng, &self.preflop_dist.vs_3bet_weights)
                };

                match action_type {
                    ActionType::Fold => {
                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Fold,
                            amount_cents: None,
                            is_all_in: false,
                            street: Street::Preflop,
                        });
                        player_folded[idx] = true;
                    }
                    ActionType::Call => {
                        let call_amount = current_bet - player_bets[idx];
                        if call_amount > 0 {
                            actions.push(Action {
                                player_name: player.name.clone(),
                                action_type: ActionType::Call,
                                amount_cents: Some(call_amount),
                                is_all_in: false,
                                street: Street::Preflop,
                            });
                            pot += call_amount;
                            player_bets[idx] = current_bet;
                        }
                    }
                    ActionType::Raise => {
                        let raise_size = if raises_count == 0 {
                            // Open raise: 2.5-3x BB
                            bb * rng.gen_range(25..=30) / 10
                        } else {
                            // 3bet/4bet: 3x previous
                            current_bet * 3
                        };
                        let total_bet = raise_size.min(player.stack_cents);
                        let to_add = total_bet - player_bets[idx];

                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Raise,
                            amount_cents: Some(total_bet),
                            is_all_in: total_bet >= player.stack_cents,
                            street: Street::Preflop,
                        });
                        pot += to_add;
                        player_bets[idx] = total_bet;
                        current_bet = total_bet;
                        raises_count += 1;
                    }
                    _ => {}
                }

                // Limitar acciones
                if raises_count >= 4 {
                    break;
                }
            }
        }

        (actions, pot, player_folded)
    }

    /// Simula acciones postflop
    fn simulate_postflop(
        &self,
        rng: &mut ChaCha8Rng,
        players: &[Player],
        initial_folded: &[bool],
        stake: StakeLevel,
        street: Street,
    ) -> (Vec<Action>, i64, Vec<bool>) {
        let mut actions = Vec::new();
        let mut pot = 0i64;
        let mut player_folded = initial_folded.to_vec();
        let bb = stake.big_blind_cents();
        let mut current_bet = 0i64;

        // Orden de accion postflop: SB -> BB -> UTG -> MP -> CO -> BTN
        let action_order = [
            Position::SmallBlind,
            Position::BigBlind,
            Position::UTG,
            Position::MP,
            Position::CO,
            Position::Button,
        ];

        for position in action_order.iter() {
            if let Some((idx, player)) = players
                .iter()
                .enumerate()
                .find(|(_, p)| p.position == Some(*position))
            {
                if player_folded[idx] {
                    continue;
                }

                // Determinar si esta IP u OOP
                let is_ip = matches!(position, Position::Button | Position::CO);

                let action_type = if current_bet == 0 {
                    // Sin apuesta
                    let weights = if is_ip {
                        &self.postflop_dist.ip_no_bet_weights
                    } else {
                        &self.postflop_dist.oop_no_bet_weights
                    };
                    self.choose_postflop_action(rng, weights)
                } else {
                    // Facing bet
                    let weights = if is_ip {
                        &self.postflop_dist.ip_facing_bet_weights
                    } else {
                        &self.postflop_dist.oop_facing_bet_weights
                    };
                    self.choose_postflop_action(rng, weights)
                };

                match action_type {
                    ActionType::Check => {
                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Check,
                            amount_cents: None,
                            is_all_in: false,
                            street,
                        });
                    }
                    ActionType::Bet => {
                        // Bet size: 50-75% pot
                        let bet_size = bb * rng.gen_range(3..=6); // Simplified
                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Bet,
                            amount_cents: Some(bet_size),
                            is_all_in: false,
                            street,
                        });
                        pot += bet_size;
                        current_bet = bet_size;
                    }
                    ActionType::Call => {
                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Call,
                            amount_cents: Some(current_bet),
                            is_all_in: false,
                            street,
                        });
                        pot += current_bet;
                    }
                    ActionType::Raise => {
                        let raise_size = current_bet * 3;
                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Raise,
                            amount_cents: Some(raise_size),
                            is_all_in: false,
                            street,
                        });
                        pot += raise_size;
                        current_bet = raise_size;
                    }
                    ActionType::Fold => {
                        actions.push(Action {
                            player_name: player.name.clone(),
                            action_type: ActionType::Fold,
                            amount_cents: None,
                            is_all_in: false,
                            street,
                        });
                        player_folded[idx] = true;
                    }
                    _ => {}
                }
            }
        }

        (actions, pot, player_folded)
    }

    /// Elige accion preflop basada en pesos
    fn choose_preflop_action(&self, rng: &mut ChaCha8Rng, weights: &[u32; 3]) -> ActionType {
        let dist = WeightedIndex::new(weights).unwrap();
        match dist.sample(rng) {
            0 => ActionType::Fold,
            1 => ActionType::Call,
            2 => ActionType::Raise,
            _ => ActionType::Fold,
        }
    }

    /// Elige accion postflop basada en pesos
    fn choose_postflop_action(&self, rng: &mut ChaCha8Rng, weights: &[u32; 5]) -> ActionType {
        let dist = WeightedIndex::new(weights).unwrap();
        match dist.sample(rng) {
            0 => ActionType::Check,
            1 => ActionType::Bet,
            2 => ActionType::Call,
            3 => ActionType::Raise,
            4 => ActionType::Fold,
            _ => ActionType::Check,
        }
    }
}

// ============================================================================
// RESULT TYPES
// ============================================================================

/// Resultado de la generacion sintetica
#[derive(Debug, Clone)]
pub struct GenerationResult {
    pub hands: Vec<ParsedHand>,
    pub elapsed_ms: u128,
    pub hands_per_second: f64,
}

impl GenerationResult {
    pub fn new(hands: Vec<ParsedHand>, elapsed_ms: u128) -> Self {
        let hands_per_second = if elapsed_ms > 0 {
            (hands.len() as f64) / (elapsed_ms as f64 / 1000.0)
        } else {
            0.0
        };
        Self {
            hands,
            elapsed_ms,
            hands_per_second,
        }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Genera manos sinteticas con la configuracion dada
pub fn generate_synthetic_hands(config: SyntheticConfig) -> GenerationResult {
    let start = std::time::Instant::now();
    let generator = SyntheticGenerator::new(config);
    let hands = generator.generate_parallel();
    let elapsed = start.elapsed().as_millis();
    GenerationResult::new(hands, elapsed)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stake_level_conversions() {
        assert_eq!(StakeLevel::NL10.small_blind_cents(), 5);
        assert_eq!(StakeLevel::NL10.big_blind_cents(), 10);
        assert_eq!(StakeLevel::NL10.typical_stack_cents(), 1000);
    }

    #[test]
    fn test_stake_from_str() {
        assert_eq!(StakeLevel::from_str("NL10"), Some(StakeLevel::NL10));
        assert_eq!(StakeLevel::from_str("nl50"), Some(StakeLevel::NL50));
        assert_eq!(StakeLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_config_builder() {
        let config = SyntheticConfig::new(1000)
            .with_seed(42)
            .with_stakes(vec!["NL10".to_string(), "NL25".to_string()]);

        assert_eq!(config.num_hands, 1000);
        assert_eq!(config.seed, Some(42));
        assert_eq!(config.stakes.len(), 2);
    }

    #[test]
    fn test_generate_small_batch() {
        let config = SyntheticConfig::new(100).with_seed(42);
        let generator = SyntheticGenerator::new(config);
        let hands = generator.generate_sequential();

        assert_eq!(hands.len(), 100);

        // Verificar estructura basica
        for hand in &hands {
            assert!(!hand.hand_id.is_empty());
            assert!(!hand.players.is_empty());
            assert!(!hand.actions.is_empty());
            assert!(hand.pot.total_cents > 0);
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let config1 = SyntheticConfig::new(10).with_seed(12345);
        let config2 = SyntheticConfig::new(10).with_seed(12345);

        let gen1 = SyntheticGenerator::new(config1);
        let gen2 = SyntheticGenerator::new(config2);

        let hands1 = gen1.generate_sequential();
        let hands2 = gen2.generate_sequential();

        // Misma semilla = mismos resultados
        for (h1, h2) in hands1.iter().zip(hands2.iter()) {
            assert_eq!(h1.hand_id, h2.hand_id);
            assert_eq!(h1.players.len(), h2.players.len());
        }
    }

    #[test]
    fn test_parallel_generation() {
        let config = SyntheticConfig::new(1000).with_seed(42);
        let result = generate_synthetic_hands(config);

        assert_eq!(result.hands.len(), 1000);
        assert!(result.elapsed_ms > 0);
        assert!(result.hands_per_second > 0.0);
    }

    #[test]
    fn test_hero_presence() {
        let config = SyntheticConfig::new(100)
            .with_seed(42)
            .with_hero_name("test_hero".to_string());

        let generator = SyntheticGenerator::new(config);
        let hands = generator.generate_sequential();

        // Al menos algunas manos deben tener al heroe
        let hero_hands = hands
            .iter()
            .filter(|h| h.players.iter().any(|p| p.is_hero))
            .count();

        assert!(hero_hands > 0, "Hero should be present in some hands");
    }

    #[test]
    fn test_positions_assigned() {
        let config = SyntheticConfig::new(10).with_seed(42);
        let generator = SyntheticGenerator::new(config);
        let hands = generator.generate_sequential();

        for hand in &hands {
            for player in &hand.players {
                assert!(
                    player.position.is_some(),
                    "Every player should have a position"
                );
            }
        }
    }
}

