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