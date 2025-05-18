pub(crate) use crate::piece::chess_piece::ChessPiece;
pub(crate) use crate::piece::position::Position;

pub struct Piece {
    position: Position,
    side: u8,
}

impl Piece {
    pub fn new(x: u8, y: u8, side: u8) -> Self {
        Self {
            position: Position::new(x, y),
            side,
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

    fn is_valid_move(&self, new_x: u8, new_y: u8) -> bool {
        todo!()
    }

    fn shift(&mut self, x: u8, y: u8) {
        self.position.x = x;
        self.position.y = y;
    }

    fn display(&self) {
        println!("Position : ({}, {})", self.position.x, self.position.y);
    }
}
