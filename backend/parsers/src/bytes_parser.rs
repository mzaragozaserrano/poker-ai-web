//! Parser optimizado basado en bytes sin uso de Regex.
//!
//! Este módulo implementa funciones de parsing de bajo nivel que operan
//! directamente sobre bytes (`&[u8]`) en lugar de strings UTF-8, evitando
//! validaciones costosas y usando comparaciones directas de bytes.
//!
//! ## Optimizaciones
//!
//! - **Lookup tables**: Tokens comunes pre-computados como constantes
//! - **Comparación de bytes**: Sin overhead de validación UTF-8
//! - **Aritmética de enteros**: Parser de centavos sin floats
//! - **Zero allocations**: Todas las funciones operan sobre slices

/// Tokens comunes en historiales de Winamax (como bytes).
pub mod tokens {
    pub const WINAMAX_POKER: &[u8] = b"Winamax Poker";
    pub const CASHGAME: &[u8] = b"CashGame";
    pub const TOURNAMENT: &[u8] = b"Tournament";
    pub const TABLE: &[u8] = b"Table: ";
    pub const SEAT: &[u8] = b"Seat ";
    pub const ANTE_BLINDS: &[u8] = b"*** ANTE/BLINDS ***";
    pub const PRE_FLOP: &[u8] = b"*** PRE-FLOP ***";
    pub const FLOP: &[u8] = b"*** FLOP ***";
    pub const TURN: &[u8] = b"*** TURN ***";
    pub const RIVER: &[u8] = b"*** RIVER ***";
    pub const SHOW_DOWN: &[u8] = b"*** SHOW DOWN ***";
    pub const SUMMARY: &[u8] = b"*** SUMMARY ***";
    pub const DEALT_TO: &[u8] = b"Dealt to ";
    pub const TOTAL_POT: &[u8] = b"Total pot";
    pub const BOARD: &[u8] = b"Board:";

    // Acciones (multi-idioma)
    pub const FOLDS: &[u8] = b" folds";
    pub const PASSE: &[u8] = b" passe"; // FR
    pub const CHECKS: &[u8] = b" checks";
    pub const PAROLE: &[u8] = b" parole"; // FR
    pub const CALLS: &[u8] = b" calls ";
    pub const SUIT: &[u8] = b" suit "; // FR
    pub const BETS: &[u8] = b" bets ";
    pub const MISE: &[u8] = b" mise "; // FR
    pub const RAISES: &[u8] = b" raises ";
    pub const RELANCE: &[u8] = b" relance "; // FR
    pub const POSTS_SMALL: &[u8] = b"posts small blind";
    pub const POSTS_BIG: &[u8] = b"posts big blind";
    pub const POSTS_ANTE: &[u8] = b"posts ante";
    pub const POSTE_PETITE: &[u8] = b"poste la petite blinde"; // FR
    pub const POSTE_GROSSE: &[u8] = b"poste la grosse blinde"; // FR
    pub const POSTE_ANTE: &[u8] = b"poste l'ante"; // FR
    pub const COLLECTED: &[u8] = b"collected";
    pub const SHOWS: &[u8] = b"shows";
    pub const ALL_IN: &[u8] = b"all-in";
}

/// Verifica si un slice de bytes comienza con un prefijo específico.
///
/// Esta función es más rápida que `str::starts_with()` porque:
/// 1. No requiere validación UTF-8
/// 2. Usa comparación directa de bytes
/// 3. Puede ser optimizada por el compilador a instrucciones SIMD
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::{starts_with_bytes, tokens};
///
/// let line = b"Winamax Poker - CashGame";
/// assert!(starts_with_bytes(line, tokens::WINAMAX_POKER));
/// ```
#[inline]
pub fn starts_with_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.len() >= needle.len() && &haystack[..needle.len()] == needle
}

/// Busca un patrón de bytes dentro de un slice.
///
/// Retorna la posición del primer match o `None` si no se encuentra.
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::find_bytes;
///
/// let line = b"Player1 raises 0.04 to 0.06";
/// assert_eq!(find_bytes(line, b"raises"), Some(8));
/// ```
#[inline]
pub fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Extrae un monto en centavos desde un slice de bytes.
///
/// Parsea números en formato "0.02€", "1.50", "2,50€" y los convierte
/// a centavos usando aritmética de enteros (sin floats).
///
/// # Algoritmo
///
/// 1. Extrae dígitos y separador decimal (. o ,)
/// 2. Separa parte entera y decimal
/// 3. Calcula: `centavos = parte_entera * 100 + parte_decimal`
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::parse_amount_cents;
///
/// assert_eq!(parse_amount_cents(b"0.02"), 2);
/// assert_eq!(parse_amount_cents(b"1.50"), 150);
/// assert_eq!(parse_amount_cents(b"2,50"), 250);
/// assert_eq!(parse_amount_cents(b"100"), 10000);
/// ```
pub fn parse_amount_cents(bytes: &[u8]) -> i64 {
    let mut integer_part = 0i64;
    let mut decimal_part = 0i64;
    let mut in_decimal = false;
    let mut decimal_digits = 0u32;

    for &byte in bytes {
        match byte {
            b'0'..=b'9' => {
                let digit = (byte - b'0') as i64;
                if in_decimal {
                    decimal_part = decimal_part * 10 + digit;
                    decimal_digits += 1;
                    if decimal_digits >= 2 {
                        break; // Solo necesitamos 2 decimales
                    }
                } else {
                    integer_part = integer_part * 10 + digit;
                }
            }
            b'.' | b',' => {
                in_decimal = true;
            }
            b'\xE2' | b'\x80' | b'\xAC' => {
                // Símbolo € en UTF-8 (E2 82 AC), ignorar
                continue;
            }
            b' ' | b'\t' => {
                // Ignorar espacios
                continue;
            }
            _ => {
                // Cualquier otro carácter termina el número
                if integer_part > 0 || decimal_part > 0 {
                    break;
                }
            }
        }
    }

    // Ajustar decimales si hay menos de 2 dígitos
    let decimal_cents = match decimal_digits {
        0 => 0,
        1 => decimal_part * 10, // 0.5 -> 50 centavos
        _ => decimal_part,      // 0.05 -> 5 centavos
    };

    integer_part * 100 + decimal_cents
}

/// Extrae el primer número encontrado en un slice de bytes.
///
/// Busca el primer dígito y extrae el número completo (incluyendo decimales).
/// Ignora dígitos que forman parte de nombres (ej: "Player1").
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::extract_first_amount;
///
/// let line = b"Player1 calls 0.02";
/// assert_eq!(extract_first_amount(line), 2);
/// ```
pub fn extract_first_amount(bytes: &[u8]) -> i64 {
    let mut i = 0;

    // Buscar el primer número que esté precedido por espacio o al inicio
    while i < bytes.len() {
        // Saltar hasta encontrar un dígito
        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }

        if i >= bytes.len() {
            return 0;
        }

        // Verificar si el dígito está precedido por espacio (no es parte de un nombre)
        let is_valid_start = i == 0 || bytes[i - 1] == b' ' || bytes[i - 1] == b'\t';

        if is_valid_start {
            // Encontrar el final del número
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b',')
            {
                i += 1;
            }

            return parse_amount_cents(&bytes[start..i]);
        }

        // Saltar este dígito y continuar buscando
        i += 1;
    }

    0
}

/// Extrae un nombre de jugador desde el inicio de una línea hasta una palabra clave.
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::extract_player_name;
///
/// let line = b"Player1 folds";
/// let name = extract_player_name(line, &[b" folds", b" calls "]);
/// assert_eq!(name, b"Player1");
/// ```
pub fn extract_player_name<'a>(line: &'a [u8], keywords: &[&[u8]]) -> &'a [u8] {
    for keyword in keywords {
        if let Some(pos) = find_bytes(line, keyword) {
            return &line[..pos];
        }
    }
    b""
}

/// Parsea un número de asiento desde una línea "Seat X: ...".
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::parse_seat_number;
///
/// let line = b"Seat 3: Player1 (2.00)";
/// assert_eq!(parse_seat_number(line), Some(3));
/// ```
pub fn parse_seat_number(line: &[u8]) -> Option<u8> {
    if !starts_with_bytes(line, tokens::SEAT) {
        return None;
    }

    let start = tokens::SEAT.len();
    let rest = &line[start..];

    // Extraer dígitos hasta encontrar ':'
    let mut seat = 0u8;
    for &byte in rest {
        if byte.is_ascii_digit() {
            seat = seat * 10 + (byte - b'0');
        } else if byte == b':' {
            return Some(seat);
        } else {
            break;
        }
    }

    None
}

/// Extrae cartas desde un string entre corchetes [Ah Kd].
///
/// Retorna un vector de pares (rank, suit) como bytes.
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::extract_cards;
///
/// let line = b"Dealt to thesmoy [8d 8s]";
/// let cards = extract_cards(line);
/// assert_eq!(cards.len(), 2);
/// assert_eq!(cards[0], (b'8', b'd'));
/// assert_eq!(cards[1], (b'8', b's'));
/// ```
pub fn extract_cards(line: &[u8]) -> Vec<(u8, u8)> {
    let mut cards = Vec::new();

    // Buscar apertura de corchete
    let start = match line.iter().position(|&b| b == b'[') {
        Some(pos) => pos + 1,
        None => return cards,
    };

    // Buscar cierre de corchete
    let end = match line[start..].iter().position(|&b| b == b']') {
        Some(pos) => start + pos,
        None => return cards,
    };

    let cards_section = &line[start..end];

    // Parsear cartas (formato: "Ah Kd" o "8d 8s")
    let mut i = 0;
    while i < cards_section.len() {
        // Saltar espacios
        while i < cards_section.len() && cards_section[i] == b' ' {
            i += 1;
        }

        // Leer rank
        if i >= cards_section.len() {
            break;
        }
        let rank = cards_section[i];
        i += 1;

        // Leer suit
        if i >= cards_section.len() {
            break;
        }
        let suit = cards_section[i];
        i += 1;

        // Validar que rank y suit sean válidos
        if is_valid_rank(rank) && is_valid_suit(suit) {
            cards.push((rank, suit));
        }
    }

    cards
}

/// Verifica si un byte es un rank válido (2-9, T, J, Q, K, A).
#[inline]
fn is_valid_rank(byte: u8) -> bool {
    matches!(byte, b'2'..=b'9' | b'T' | b'J' | b'Q' | b'K' | b'A')
}

/// Verifica si un byte es un suit válido (h, d, c, s).
#[inline]
fn is_valid_suit(byte: u8) -> bool {
    matches!(byte, b'h' | b'd' | b'c' | b's')
}

/// Convierte bytes ASCII a string (sin validación UTF-8).
///
/// # Safety
///
/// UNSAFE: Solo usar si estás seguro de que los bytes son ASCII válido.
/// Si los bytes contienen UTF-8 inválido, este comportamiento es undefined.
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::bytes_to_string_unchecked;
///
/// let bytes = b"Player1";
/// let s = unsafe { bytes_to_string_unchecked(bytes) };
/// assert_eq!(s, "Player1");
/// ```
#[inline]
pub unsafe fn bytes_to_string_unchecked(bytes: &[u8]) -> String {
    String::from_utf8_unchecked(bytes.to_vec())
}

/// Convierte bytes a string de forma segura (con validación UTF-8).
///
/// # Ejemplo
///
/// ```
/// use poker_parsers::bytes_parser::bytes_to_string;
///
/// let bytes = b"Player1";
/// assert_eq!(bytes_to_string(bytes), "Player1");
/// ```
#[inline]
pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with_bytes() {
        assert!(starts_with_bytes(
            b"Winamax Poker - CashGame",
            tokens::WINAMAX_POKER
        ));
        assert!(starts_with_bytes(b"Seat 1: Player", tokens::SEAT));
        assert!(!starts_with_bytes(b"Player folds", tokens::WINAMAX_POKER));
    }

    #[test]
    fn test_find_bytes() {
        assert_eq!(find_bytes(b"Player1 raises 0.04", b"raises"), Some(8));
        assert_eq!(find_bytes(b"Player1 folds", b"folds"), Some(8));
        assert_eq!(find_bytes(b"No match here", b"xyz"), None);
    }

    #[test]
    fn test_parse_amount_cents() {
        assert_eq!(parse_amount_cents(b"0.02"), 2);
        assert_eq!(parse_amount_cents(b"1.50"), 150);
        assert_eq!(parse_amount_cents(b"100"), 10000);
        assert_eq!(parse_amount_cents(b"2,50"), 250);
        assert_eq!(parse_amount_cents(b"0.01\xE2\x82\xAC"), 1); // 0.01€
        assert_eq!(parse_amount_cents(b"10.5"), 1050);
    }

    #[test]
    fn test_extract_first_amount() {
        assert_eq!(extract_first_amount(b"Player1 calls 0.02"), 2);
        assert_eq!(extract_first_amount(b"raises 0.04 to 0.06"), 4);
        assert_eq!(extract_first_amount(b"collected 1.50 from pot"), 150);
    }

    #[test]
    fn test_extract_player_name() {
        let keywords = &[tokens::FOLDS, tokens::CALLS, tokens::RAISES];
        assert_eq!(extract_player_name(b"Player1 folds", keywords), b"Player1");
        assert_eq!(
            extract_player_name(b"thesmoy calls 0.02", keywords),
            b"thesmoy"
        );
    }

    #[test]
    fn test_parse_seat_number() {
        assert_eq!(parse_seat_number(b"Seat 1: Player1"), Some(1));
        assert_eq!(parse_seat_number(b"Seat 5: thesmoy (2.00)"), Some(5));
        assert_eq!(parse_seat_number(b"Not a seat line"), None);
    }

    #[test]
    fn test_extract_cards() {
        let cards = extract_cards(b"Dealt to thesmoy [8d 8s]");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0], (b'8', b'd'));
        assert_eq!(cards[1], (b'8', b's'));

        let cards = extract_cards(b"shows [Ah Kh]");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0], (b'A', b'h'));
        assert_eq!(cards[1], (b'K', b'h'));

        let cards = extract_cards(b"*** FLOP *** [6d Qc 7s]");
        assert_eq!(cards.len(), 3);
    }

    #[test]
    fn test_is_valid_rank() {
        assert!(is_valid_rank(b'A'));
        assert!(is_valid_rank(b'K'));
        assert!(is_valid_rank(b'2'));
        assert!(is_valid_rank(b'T'));
        assert!(!is_valid_rank(b'X'));
        assert!(!is_valid_rank(b'1'));
    }

    #[test]
    fn test_is_valid_suit() {
        assert!(is_valid_suit(b'h'));
        assert!(is_valid_suit(b'd'));
        assert!(is_valid_suit(b'c'));
        assert!(is_valid_suit(b's'));
        assert!(!is_valid_suit(b'x'));
    }
}

