#[derive(Clone)]
pub struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}