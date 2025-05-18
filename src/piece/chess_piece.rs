use crate::piece::piece::Position;

pub trait ChessPiece {
    fn get_position(&self) -> &Position;
    fn get_position_mut(&mut self) -> &mut Position;

    fn is_valid_move(&self, destination: &Position) -> bool;

    fn shift(&mut self, x: u8, y: u8) {
        let pos = self.get_position_mut();
        pos.x = x;
        pos.y = y;
    }

    fn display(&self) {
        let pos = self.get_position();
        println!("Position : ({}, {})", pos.x, pos.y);
    }
}