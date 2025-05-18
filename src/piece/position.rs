#[derive(Clone)]
pub struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    
    pub fn to_string(&self) -> String {
        let chars = to_hexadecimal(self);
        format!("({}, {})", chars.0, chars.1)
    }
}

pub fn to_hexadecimal(position: &Position) -> (char, char) {
    let x_char = (b'A' + position.x) as char;
    (x_char, (b'0' + position.y) as char)
}
