#[derive(Clone)]
pub struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

pub trait ChessPiece {
    fn get_position(&self) -> &Position;
    fn get_position_mut(&mut self) -> &mut Position;
    fn shift(&mut self, x: u8, y: u8);
    fn display(&self);
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
}

impl ChessPiece for Piece {
    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn shift(&mut self, x: u8, y: u8) {
        self.position.x = x;
        self.position.y = y;
    }

    fn display(&self) {
        println!("Position : ({}, {})", self.position.x, self.position.y);
    }
}