use std::fmt;

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn from_algebraic(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }

        let chars: Vec<char> = s.chars().collect();
        let file = chars[0].to_ascii_lowercase();
        let rank = chars[1];

        let x = (file as u8).checked_sub(b'a')? as i8;
        let y = (rank.to_digit(10)? as i8) - 1;

        if x < 0 || x > 7 || y < 0 || y > 7 {
            return None;
        }

        Some(Position::new(x, y))
    }

    pub fn to_string(&self) -> String {
        let chars = to_hexadecimal(self);
        format!("({}, {})", chars.0, chars.1)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn to_hexadecimal(position: &Position) -> (char, char) {
    let x_char = (b'A' + position.x as u8) as char;
    (x_char, (b'1' + position.y as u8) as char)
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x_char, y_char) = to_hexadecimal(self);
        write!(f, "({}, {})", x_char, y_char)
    }
}


pub const A1: Position = Position { x: 0, y: 0 };
pub const B1: Position = Position { x: 1, y: 0 };
pub const C1: Position = Position { x: 2, y: 0 };
pub const D1: Position = Position { x: 3, y: 0 };
pub const E1: Position = Position { x: 4, y: 0 };
pub const F1: Position = Position { x: 5, y: 0 };
pub const G1: Position = Position { x: 6, y: 0 };
pub const H1: Position = Position { x: 7, y: 0 };
pub const A2: Position = Position { x: 0, y: 1 };
pub const B2: Position = Position { x: 1, y: 1 };
pub const C2: Position = Position { x: 2, y: 1 };
pub const D2: Position = Position { x: 3, y: 1 };
pub const E2: Position = Position { x: 4, y: 1 };
pub const F2: Position = Position { x: 5, y: 1 };
pub const G2: Position = Position { x: 6, y: 1 };
pub const H2: Position = Position { x: 7, y: 1 };
pub const A3: Position = Position { x: 0, y: 2 };
pub const B3: Position = Position { x: 1, y: 2 };
pub const C3: Position = Position { x: 2, y: 2 };
pub const D3: Position = Position { x: 3, y: 2 };
pub const E3: Position = Position { x: 4, y: 2 };
pub const F3: Position = Position { x: 5, y: 2 };
pub const G3: Position = Position { x: 6, y: 2 };
pub const H3: Position = Position { x: 7, y: 2 };
pub const A4: Position = Position { x: 0, y: 3 };
pub const B4: Position = Position { x: 1, y: 3 };
pub const C4: Position = Position { x: 2, y: 3 };
pub const D4: Position = Position { x: 3, y: 3 };
pub const E4: Position = Position { x: 4, y: 3 };
pub const F4: Position = Position { x: 5, y: 3 };
pub const G4: Position = Position { x: 6, y: 3 };
pub const H4: Position = Position { x: 7, y: 3 };
pub const A5: Position = Position { x: 0, y: 4 };
pub const B5: Position = Position { x: 1, y: 4 };
pub const C5: Position = Position { x: 2, y: 4 };
pub const D5: Position = Position { x: 3, y: 4 };
pub const E5: Position = Position { x: 4, y: 4 };
pub const F5: Position = Position { x: 5, y: 4 };
pub const G5: Position = Position { x: 6, y: 4 };
pub const H5: Position = Position { x: 7, y: 4 };
pub const A6: Position = Position { x: 0, y: 5 };
pub const B6: Position = Position { x: 1, y: 5 };
pub const C6: Position = Position { x: 2, y: 5 };
pub const D6: Position = Position { x: 3, y: 5 };
pub const E6: Position = Position { x: 4, y: 5 };
pub const F6: Position = Position { x: 5, y: 5 };
pub const G6: Position = Position { x: 6, y: 5 };
pub const H6: Position = Position { x: 7, y: 5 };
pub const A7: Position = Position { x: 0, y: 6 };
pub const B7: Position = Position { x: 1, y: 6 };
pub const C7: Position = Position { x: 2, y: 6 };
pub const D7: Position = Position { x: 3, y: 6 };
pub const E7: Position = Position { x: 4, y: 6 };
pub const F7: Position = Position { x: 5, y: 6 };
pub const G7: Position = Position { x: 6, y: 6 };
pub const H7: Position = Position { x: 7, y: 6 };
pub const A8: Position = Position { x: 0, y: 7 };
pub const B8: Position = Position { x: 1, y: 7 };
pub const C8: Position = Position { x: 2, y: 7 };
pub const D8: Position = Position { x: 3, y: 7 };
pub const E8: Position = Position { x: 4, y: 7 };
pub const F8: Position = Position { x: 5, y: 7 };
pub const G8: Position = Position { x: 6, y: 7 };
pub const H8: Position = Position { x: 7, y: 7 };