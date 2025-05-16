pub struct Position {
    x: u8,
    y: u8
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y
        }
    }
}

pub struct Piece {
    position: Position
}

impl Piece {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            position: Position::new(x, y)
        }
    }

    pub fn shift(&mut self, x: u8, y: u8) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn display(&self) {
        println!("Position : ({}, {})", self.position.x, self.position.y);
    }
}