//! Máquina de Estados Finitos (FSM) para parsing de historiales Winamax.
//!
//! Este módulo implementa un parser eficiente que procesa líneas de historial
//! usando string slicing y prefijos, evitando Regex en hot loops para
//! maximizar el rendimiento en el Ryzen 3800X.

use crate::types::{
    Action, ActionType, Card, GameType, ParseResult, ParsedHand, ParserState, Player, Position,
    Street,
};

/// Hero por defecto según configuración del proyecto.
const HERO_NAME: &str = "thesmoy";

/// Parser FSM para historiales de Winamax.
#[derive(Debug)]
pub struct WinamaxParser {
    /// Estado actual del parser.
    state: ParserState,
    /// Mano actual en construcción.
    current_hand: ParsedHand,
    /// Calle actual para asignar acciones.
    current_street: Street,
    /// Manos completadas.
    hands: Vec<ParsedHand>,
    /// Errores encontrados durante el parsing.
    errors: Vec<String>,
}

impl Default for WinamaxParser {
    fn default() -> Self {
        Self::new()
    }
}

impl WinamaxParser {
    /// Crea un nuevo parser en estado inicial.
    pub fn new() -> Self {
        Self {
            state: ParserState::Initial,
            current_hand: ParsedHand::default(),
            current_street: Street::Preflop,
            hands: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Parsea un archivo completo de historial.
    pub fn parse(&mut self, content: &str) -> ParseResult {
        for line in content.lines() {
            self.process_line(line);
        }

        // Finalizar última mano si existe
        if !self.current_hand.hand_id.is_empty() {
            self.finalize_hand();
        }

        ParseResult {
            hands: std::mem::take(&mut self.hands),
            error_count: self.errors.len(),
            errors: std::mem::take(&mut self.errors),
        }
    }

    /// Procesa una línea individual según el estado actual.
    fn process_line(&mut self, line: &str) {
        let line = line.trim();

        // Líneas vacías indican fin de mano
        if line.is_empty() {
            if !self.current_hand.hand_id.is_empty() && self.state == ParserState::Summary {
                self.finalize_hand();
            }
            return;
        }

        // Detectar transiciones de estado por prefijos
        if line.starts_with("Winamax Poker - ") {
            self.start_new_hand(line);
            return;
        }

        if line.starts_with("Table: ") {
            self.parse_table_info(line);
            return;
        }

        if line.starts_with("Seat ") && line.contains(':') && !line.contains("won") {
            self.parse_seat_info(line);
            return;
        }

        if line.starts_with("*** ") {
            self.handle_section_marker(line);
            return;
        }

        if line.starts_with("Dealt to ") {
            self.parse_dealt_cards(line);
            return;
        }

        // Procesar acciones según el estado actual
        match self.state {
            ParserState::Blinds => self.parse_blind_action(line),
            ParserState::Preflop
            | ParserState::Flop
            | ParserState::Turn
            | ParserState::River
            | ParserState::Showdown => self.parse_action(line),
            ParserState::Summary => self.parse_summary_line(line),
            _ => {}
        }
    }

    /// Inicia el parsing de una nueva mano.
    fn start_new_hand(&mut self, line: &str) {
        // Si hay una mano anterior sin finalizar, guardarla
        if !self.current_hand.hand_id.is_empty() {
            self.finalize_hand();
        }

        self.current_hand = ParsedHand::default();
        self.current_street = Street::Preflop;
        self.state = ParserState::Header;

        // Detectar tipo de juego
        if line.contains("CashGame") {
            self.current_hand.game_type = GameType::CashGame;
        } else if line.contains("Tournament") {
            self.current_hand.game_type = GameType::Tournament;
        }

        // Extraer HandId: #XXXX-XXX-XXXXXXXXXX
        if let Some(start) = line.find("HandId: #") {
            let start = start + 9; // Saltar "HandId: #"
            if let Some(end) = line[start..].find(" - ") {
                self.current_hand.hand_id = line[start..start + end].to_string();
            }
        }

        // Extraer blinds: (0.01€/0.02€)
        if let Some(start) = line.find('(') {
            if let Some(end) = line[start..].find(')') {
                let blinds_str = &line[start + 1..start + end];
                self.parse_blinds_from_header(blinds_str);
            }
        }

        // Extraer timestamp: 2025/12/15 14:02:20 UTC
        if let Some(idx) = line.rfind(" - ") {
            let timestamp_part = &line[idx + 3..];
            self.current_hand.timestamp = timestamp_part.trim().to_string();
        }
    }

    /// Parsea información de blinds desde la cabecera.
    fn parse_blinds_from_header(&mut self, blinds_str: &str) {
        // Formato: "0.01€/0.02€" o "1€/2€"
        let parts: Vec<&str> = blinds_str.split('/').collect();
        if parts.len() == 2 {
            self.current_hand.small_blind_cents = self.parse_amount(parts[0]);
            self.current_hand.big_blind_cents = self.parse_amount(parts[1]);

            // Detectar play money
            if blinds_str.contains('v') || blinds_str.contains("chips") {
                self.current_hand.is_play_money = true;
            }
        }
    }

    /// Parsea información de la mesa.
    fn parse_table_info(&mut self, line: &str) {
        // Formato: Table: 'Nice 09' 5-max (real money) Seat #3 is the button
        self.state = ParserState::Seats;

        // Extraer nombre de mesa
        if let Some(start) = line.find('\'') {
            if let Some(end) = line[start + 1..].find('\'') {
                self.current_hand.table_name = line[start + 1..start + 1 + end].to_string();
            }
        }

        // Extraer max players (5-max, 6-max)
        if line.contains("5-max") {
            self.current_hand.max_players = 5;
        } else if line.contains("6-max") {
            self.current_hand.max_players = 6;
        }

        // Extraer posición del botón: Seat #X is the button
        if let Some(idx) = line.find("Seat #") {
            let start = idx + 6;
            if let Some(end_idx) = line[start..].find(' ') {
                if let Ok(seat) = line[start..start + end_idx].parse::<u8>() {
                    self.current_hand.button_seat = seat;
                }
            }
        }
    }

    /// Parsea información de un asiento/jugador.
    fn parse_seat_info(&mut self, line: &str) {
        // Formato: Seat 1: captainogue (1.76€)
        let parts: Vec<&str> = line.splitn(2, ": ").collect();
        if parts.len() != 2 {
            return;
        }

        // Extraer número de asiento
        let seat_str = parts[0].trim_start_matches("Seat ");
        let seat: u8 = match seat_str.parse() {
            Ok(s) => s,
            Err(_) => return,
        };

        // Separar nombre y stack
        let rest = parts[1];
        if let Some(paren_start) = rest.rfind('(') {
            let name = rest[..paren_start].trim().to_string();
            let stack_str = &rest[paren_start + 1..rest.len() - 1];
            let stack_cents = self.parse_amount(stack_str);
            let is_hero = name == HERO_NAME;

            let player = Player {
                name,
                seat,
                stack_cents,
                position: None, // Se calcula después
                hole_cards: None,
                is_hero,
            };

            self.current_hand.players.push(player);
        }
    }

    /// Maneja marcadores de sección (*** XXX ***).
    fn handle_section_marker(&mut self, line: &str) {
        if line.contains("ANTE/BLINDS") {
            self.state = ParserState::Blinds;
            self.calculate_positions();
        } else if line.contains("PRE-FLOP") {
            self.state = ParserState::Preflop;
            self.current_street = Street::Preflop;
        } else if line.contains("FLOP") && !line.contains("PRE") {
            self.state = ParserState::Flop;
            self.current_street = Street::Flop;
            self.parse_board_cards(line);
        } else if line.contains("TURN") {
            self.state = ParserState::Turn;
            self.current_street = Street::Turn;
            self.parse_board_cards(line);
        } else if line.contains("RIVER") {
            self.state = ParserState::River;
            self.current_street = Street::River;
            self.parse_board_cards(line);
        } else if line.contains("SHOW DOWN") {
            self.state = ParserState::Showdown;
        } else if line.contains("SUMMARY") {
            self.state = ParserState::Summary;
        }
    }

    /// Calcula las posiciones de los jugadores basado en el botón.
    fn calculate_positions(&mut self) {
        let button_seat = self.current_hand.button_seat;
        let num_players = self.current_hand.players.len();

        if num_players == 0 {
            return;
        }

        // Ordenar asientos a partir del botón
        let mut seats: Vec<u8> = self.current_hand.players.iter().map(|p| p.seat).collect();
        seats.sort();

        // Encontrar índice del botón en la lista ordenada
        let btn_idx = seats.iter().position(|&s| s == button_seat).unwrap_or(0);

        // Asignar posiciones en orden desde el botón
        let positions_6max = [
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
            Position::UTG,
            Position::MP,
            Position::CO,
        ];

        let positions_5max = [
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
            Position::MP,
            Position::CO,
        ];

        let positions: &[Position] = if num_players <= 5 {
            &positions_5max[..num_players.min(5)]
        } else {
            &positions_6max[..num_players.min(6)]
        };

        for (i, pos) in positions.iter().enumerate() {
            let seat_idx = (btn_idx + i) % num_players;
            let target_seat = seats[seat_idx];

            if let Some(player) = self
                .current_hand
                .players
                .iter_mut()
                .find(|p| p.seat == target_seat)
            {
                player.position = Some(*pos);
            }
        }
    }

    /// Parsea las cartas repartidas al héroe.
    fn parse_dealt_cards(&mut self, line: &str) {
        // Formato: Dealt to thesmoy [8d 8s]
        if let Some(bracket_start) = line.find('[') {
            if let Some(bracket_end) = line.find(']') {
                let cards_str = &line[bracket_start + 1..bracket_end];
                let cards: Vec<&str> = cards_str.split_whitespace().collect();

                if cards.len() == 2 {
                    if let (Some(c1), Some(c2)) = (Card::parse(cards[0]), Card::parse(cards[1])) {
                        self.current_hand.hero_cards = Some([c1.clone(), c2.clone()]);

                        // Asignar cartas al jugador héroe
                        if let Some(hero) =
                            self.current_hand.players.iter_mut().find(|p| p.is_hero)
                        {
                            hero.hole_cards = Some([c1, c2]);
                        }
                    }
                }
            }
        }
    }

    /// Parsea cartas del board desde marcadores de sección.
    fn parse_board_cards(&mut self, line: &str) {
        // Formato FLOP: *** FLOP *** [6d Qc 7s]
        // Formato TURN: *** TURN *** [6d Qc 7s][2c]
        // Formato RIVER: *** RIVER *** [6d Qc 7s 2c][Kh]

        // Buscar todos los grupos de cartas entre corchetes
        let mut i = 0;
        while let Some(start) = line[i..].find('[') {
            let abs_start = i + start + 1;
            if let Some(end) = line[abs_start..].find(']') {
                let cards_str = &line[abs_start..abs_start + end];

                // Solo agregar cartas nuevas (las del último corchete en TURN/RIVER)
                if line.contains("TURN") || line.contains("RIVER") {
                    // En TURN/RIVER, solo tomar el último corchete
                    let is_last = line[abs_start + end..].find('[').is_none();
                    if is_last {
                        for card_str in cards_str.split_whitespace() {
                            if let Some(card) = Card::parse(card_str) {
                                self.current_hand.board.push(card);
                            }
                        }
                    }
                } else {
                    // En FLOP, tomar todas las cartas
                    for card_str in cards_str.split_whitespace() {
                        if let Some(card) = Card::parse(card_str) {
                            self.current_hand.board.push(card);
                        }
                    }
                }

                i = abs_start + end + 1;
            } else {
                break;
            }
        }
    }

    /// Parsea acciones de ciegas.
    fn parse_blind_action(&mut self, line: &str) {
        if line.contains("posts small blind") || line.contains("poste la petite blinde") {
            self.parse_blind(line, ActionType::PostSmallBlind);
        } else if line.contains("posts big blind") || line.contains("poste la grosse blinde") {
            self.parse_blind(line, ActionType::PostBigBlind);
        } else if line.contains("posts ante") || line.contains("poste l'ante") {
            self.parse_blind(line, ActionType::PostAnte);
        }
    }

    /// Parsea una acción de ciega específica.
    fn parse_blind(&mut self, line: &str, action_type: ActionType) {
        let player_name = self.extract_player_name(line);
        let amount = self.extract_amount_from_line(line);

        let action = Action {
            player_name,
            action_type,
            amount_cents: Some(amount),
            is_all_in: line.contains("all-in"),
            street: Street::Preflop,
        };

        self.current_hand.actions.push(action);
    }

    /// Parsea una acción de juego.
    fn parse_action(&mut self, line: &str) {
        let player_name = self.extract_player_name(line);
        if player_name.is_empty() {
            // Puede ser línea de collected o shows
            if line.contains("collected") {
                self.parse_collect(line);
            } else if line.contains("shows") {
                self.parse_show(line);
            }
            return;
        }

        let is_all_in = line.contains("all-in");

        // Detectar tipo de acción (multi-idioma)
        let (action_type, amount) = if line.contains(" folds") || line.contains(" passe") {
            (ActionType::Fold, None)
        } else if line.contains(" checks") || line.contains(" parole") {
            (ActionType::Check, None)
        } else if line.contains(" calls ") || line.contains(" suit ") {
            let amt = self.extract_amount_from_line(line);
            (ActionType::Call, Some(amt))
        } else if line.contains(" bets ") || line.contains(" mise ") {
            let amt = self.extract_amount_from_line(line);
            (ActionType::Bet, Some(amt))
        } else if line.contains(" raises ") || line.contains(" relance ") {
            // Para raises, extraer el monto total (después de "to" o "à")
            let amt = self.extract_raise_amount(line);
            (ActionType::Raise, Some(amt))
        } else if line.contains("collected") {
            self.parse_collect(line);
            return;
        } else if line.contains("shows") {
            self.parse_show(line);
            return;
        } else {
            return;
        };

        let action = Action {
            player_name,
            action_type,
            amount_cents: amount,
            is_all_in,
            street: self.current_street,
        };

        self.current_hand.actions.push(action);
    }

    /// Parsea una acción de mostrar cartas.
    fn parse_show(&mut self, line: &str) {
        let player_name = self.extract_player_name_before_keyword(line, "shows");
        if player_name.is_empty() {
            return;
        }

        // Extraer cartas mostradas
        if let Some(bracket_start) = line.find('[') {
            if let Some(bracket_end) = line.find(']') {
                let cards_str = &line[bracket_start + 1..bracket_end];
                let cards: Vec<&str> = cards_str.split_whitespace().collect();

                if cards.len() == 2 {
                    if let (Some(c1), Some(c2)) = (Card::parse(cards[0]), Card::parse(cards[1])) {
                        // Asignar cartas al jugador correspondiente
                        if let Some(player) = self
                            .current_hand
                            .players
                            .iter_mut()
                            .find(|p| p.name == player_name)
                        {
                            player.hole_cards = Some([c1, c2]);
                        }
                    }
                }
            }
        }

        let action = Action {
            player_name,
            action_type: ActionType::Show,
            amount_cents: None,
            is_all_in: false,
            street: self.current_street,
        };

        self.current_hand.actions.push(action);
    }

    /// Parsea una acción de recolectar bote.
    fn parse_collect(&mut self, line: &str) {
        let player_name = self.extract_player_name_before_keyword(line, "collected");
        if player_name.is_empty() {
            return;
        }

        let amount = self.extract_amount_from_line(line);

        // Agregar a ganadores
        self.current_hand.pot.winners.push((player_name.clone(), amount));

        let action = Action {
            player_name,
            action_type: ActionType::Collect,
            amount_cents: Some(amount),
            is_all_in: false,
            street: self.current_street,
        };

        self.current_hand.actions.push(action);
    }

    /// Parsea una línea de la sección SUMMARY.
    fn parse_summary_line(&mut self, line: &str) {
        if line.starts_with("Total pot") {
            // Formato: Total pot 0.91€ | Rake 0.03€
            // O: Total pot 0.08€ | No rake
            let parts: Vec<&str> = line.split('|').collect();

            if !parts.is_empty() {
                // Extraer total pot
                let pot_part = parts[0];
                if let Some(idx) = pot_part.find("Total pot") {
                    let amount_str = pot_part[idx + 9..].trim();
                    self.current_hand.pot.total_cents = self.parse_amount(amount_str);
                }
            }

            if parts.len() > 1 {
                let rake_part = parts[1].trim();
                if rake_part.starts_with("Rake") {
                    let amount_str = rake_part.trim_start_matches("Rake").trim();
                    self.current_hand.pot.rake_cents = self.parse_amount(amount_str);
                }
            }
        } else if line.starts_with("Board:") {
            // Board ya parseado desde marcadores de sección, pero validar
            // Formato: Board: [6d Qc 7s 2c]
        }
        // Las líneas "Seat X: player won/showed..." se ignoran (ya tenemos info de collected)
    }

    /// Extrae el nombre del jugador desde el inicio de una línea de acción.
    fn extract_player_name(&self, line: &str) -> String {
        // El nombre termina en el primer espacio seguido de una palabra clave de acción
        let keywords = [
            " folds",
            " checks",
            " calls ",
            " bets ",
            " raises ",
            " passe",
            " parole",
            " suit ",
            " mise ",
            " relance ",
            " posts ",
            " poste ",
            " collected",
            " shows",
        ];

        for keyword in keywords {
            if let Some(idx) = line.find(keyword) {
                return line[..idx].to_string();
            }
        }

        String::new()
    }

    /// Extrae el nombre del jugador antes de una palabra clave específica.
    fn extract_player_name_before_keyword(&self, line: &str, keyword: &str) -> String {
        if let Some(idx) = line.find(keyword) {
            return line[..idx].trim().to_string();
        }
        String::new()
    }

    /// Extrae el monto de una línea de acción.
    fn extract_amount_from_line(&self, line: &str) -> i64 {
        // Buscar el primer número después de una palabra clave de cantidad
        let keywords = [
            "calls ", "bets ", "raises ", "blind ", "collected ", "suit ", "mise ", "relance ",
            "blinde ",
        ];

        for keyword in keywords {
            if let Some(idx) = line.find(keyword) {
                let start = idx + keyword.len();
                let rest = &line[start..];
                return self.parse_first_amount(rest);
            }
        }

        0
    }

    /// Extrae el monto total de un raise (después de "to" o "à").
    fn extract_raise_amount(&self, line: &str) -> i64 {
        // Formato: "raises 0.04€ to 0.06€" -> extraer 0.06€
        // Formato FR: "relance 0.04€ à 0.06€" -> extraer 0.06€
        if let Some(idx) = line.find(" to ") {
            let rest = &line[idx + 4..];
            return self.parse_first_amount(rest);
        }
        if let Some(idx) = line.find(" à ") {
            let rest = &line[idx + 4..];
            return self.parse_first_amount(rest);
        }

        // Fallback: extraer cualquier monto
        self.extract_amount_from_line(line)
    }

    /// Parsea el primer monto encontrado en un string.
    fn parse_first_amount(&self, s: &str) -> i64 {
        let mut num_str = String::new();
        let mut found_digit = false;

        for c in s.chars() {
            if c.is_ascii_digit() || c == '.' || c == ',' {
                num_str.push(if c == ',' { '.' } else { c });
                found_digit = true;
            } else if found_digit {
                break;
            }
        }

        self.parse_amount(&num_str)
    }

    /// Convierte un string de cantidad a centavos.
    fn parse_amount(&self, s: &str) -> i64 {
        // Limpiar símbolo de moneda y espacios
        let cleaned: String = s
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
            .map(|c| if c == ',' { '.' } else { c })
            .collect();

        if let Ok(amount) = cleaned.parse::<f64>() {
            (amount * 100.0).round() as i64
        } else {
            0
        }
    }

    /// Finaliza la mano actual y la agrega a la lista de manos completadas.
    fn finalize_hand(&mut self) {
        if self.current_hand.hand_id.is_empty() {
            return;
        }

        // Validaciones básicas
        if self.current_hand.players.is_empty() {
            self.errors.push(format!(
                "Hand {} has no players",
                self.current_hand.hand_id
            ));
        }

        self.hands.push(std::mem::take(&mut self.current_hand));
        self.state = ParserState::Initial;
        self.current_hand = ParsedHand::default();
    }

    /// Resetea el parser para procesar un nuevo archivo.
    pub fn reset(&mut self) {
        self.state = ParserState::Initial;
        self.current_hand = ParsedHand::default();
        self.current_street = Street::Preflop;
        self.hands.clear();
        self.errors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amount() {
        let parser = WinamaxParser::new();

        assert_eq!(parser.parse_amount("0.02€"), 2);
        assert_eq!(parser.parse_amount("1.50€"), 150);
        assert_eq!(parser.parse_amount("100€"), 10000);
        assert_eq!(parser.parse_amount("0.01"), 1);
        assert_eq!(parser.parse_amount("2,50€"), 250);
    }

    #[test]
    fn test_parse_simple_hand() {
        let content = r#"Winamax Poker - CashGame - HandId: #21819158-393-1765807340 - Holdem no limit (0.01€/0.02€) - 2025/12/15 14:02:20 UTC
Table: 'Nice 09' 5-max (real money) Seat #3 is the button
Seat 1: captainogue (1.76€)
Seat 2: verlan4 (2.24€)
Seat 3: CucleBen (1.82€)
Seat 4: D0LLIPRANE (2.93€)
Seat 5: thesmoy (2€)
*** ANTE/BLINDS ***
D0LLIPRANE posts small blind 0.01€
thesmoy posts big blind 0.02€
Dealt to thesmoy [8d 8s]
*** PRE-FLOP ***
captainogue folds
verlan4 calls 0.02€
CucleBen raises 0.04€ to 0.06€
D0LLIPRANE folds
thesmoy calls 0.04€
verlan4 calls 0.04€
*** FLOP *** [6d Qc 7s]
thesmoy checks
verlan4 checks
CucleBen bets 0.19€
thesmoy folds
verlan4 calls 0.19€
*** TURN *** [6d Qc 7s][2c]
verlan4 checks
CucleBen bets 0.37€
verlan4 folds
CucleBen collected 0.91€ from pot
*** SUMMARY ***
Total pot 0.91€ | Rake 0.03€
Board: [6d Qc 7s 2c]
Seat 3: CucleBen (button) won 0.91€

"#;

        let mut parser = WinamaxParser::new();
        let result = parser.parse(content);

        assert_eq!(result.hands.len(), 1);
        assert_eq!(result.error_count, 0);

        let hand = &result.hands[0];
        assert_eq!(hand.hand_id, "21819158-393-1765807340");
        assert_eq!(hand.game_type, GameType::CashGame);
        assert_eq!(hand.table_name, "Nice 09");
        assert_eq!(hand.max_players, 5);
        assert_eq!(hand.small_blind_cents, 1);
        assert_eq!(hand.big_blind_cents, 2);
        assert_eq!(hand.button_seat, 3);
        assert_eq!(hand.players.len(), 5);

        // Verificar cartas del héroe
        assert!(hand.hero_cards.is_some());
        let hero_cards = hand.hero_cards.as_ref().unwrap();
        assert_eq!(hero_cards[0].rank, '8');
        assert_eq!(hero_cards[0].suit, 'd');
        assert_eq!(hero_cards[1].rank, '8');
        assert_eq!(hero_cards[1].suit, 's');

        // Verificar board
        assert_eq!(hand.board.len(), 4);

        // Verificar pot y rake
        assert_eq!(hand.pot.total_cents, 91);
        assert_eq!(hand.pot.rake_cents, 3);
        assert_eq!(hand.pot.winners.len(), 1);
        assert_eq!(hand.pot.winners[0].0, "CucleBen");
        assert_eq!(hand.pot.winners[0].1, 91);
    }

    #[test]
    fn test_parse_showdown_hand() {
        let content = r#"Winamax Poker - CashGame - HandId: #21819158-400-1765807600 - Holdem no limit (0.01€/0.02€) - 2025/12/15 14:10:00 UTC
Table: 'Nice 09' 5-max (real money) Seat #2 is the button
Seat 1: Player1 (2€)
Seat 2: thesmoy (2€)
*** ANTE/BLINDS ***
Player1 posts small blind 0.01€
thesmoy posts big blind 0.02€
Dealt to thesmoy [Ah Kh]
*** PRE-FLOP ***
Player1 raises 0.04€ to 0.06€
thesmoy calls 0.04€
*** FLOP *** [As Kd 7c]
Player1 checks
thesmoy bets 0.10€
Player1 calls 0.10€
*** TURN *** [As Kd 7c][2h]
Player1 checks
thesmoy checks
*** RIVER *** [As Kd 7c 2h][3s]
Player1 checks
thesmoy checks
*** SHOW DOWN ***
thesmoy shows [Ah Kh] (Two pairs : Aces and Kings)
Player1 shows [Qd Qs] (One pair : Queens)
thesmoy collected 0.32€ from pot
*** SUMMARY ***
Total pot 0.32€ | Rake 0.02€
Board: [As Kd 7c 2h 3s]
Seat 2: thesmoy (button) won 0.32€

"#;

        let mut parser = WinamaxParser::new();
        let result = parser.parse(content);

        assert_eq!(result.hands.len(), 1);
        let hand = &result.hands[0];

        // Verificar que el showdown registró las cartas de ambos jugadores
        let player1 = hand.players.iter().find(|p| p.name == "Player1").unwrap();
        assert!(player1.hole_cards.is_some());
        let p1_cards = player1.hole_cards.as_ref().unwrap();
        assert_eq!(p1_cards[0].rank, 'Q');

        // Verificar board completo
        assert_eq!(hand.board.len(), 5);
    }

    #[test]
    fn test_card_parse() {
        assert!(Card::parse("Ah").is_some());
        assert!(Card::parse("Td").is_some());
        assert!(Card::parse("2c").is_some());
        assert!(Card::parse("Ks").is_some());

        let card = Card::parse("Ah").unwrap();
        assert_eq!(card.rank, 'A');
        assert_eq!(card.suit, 'h');
    }
}
