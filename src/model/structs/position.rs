use std::fmt;

#[derive(Clone)]
pub struct Position {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
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