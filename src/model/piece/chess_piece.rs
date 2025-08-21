use crate::model::board::board::Board;
use crate::model::piece::piece::{Piece};
use crate::model::structs::position::Position;

pub trait ChessPiece {
    fn get_position(&self) -> &Position;
    fn get_position_mut(&mut self) -> &mut Position;
    fn get_piece_mut(&mut self) -> &mut Piece;

    #[allow(dead_code)]
    fn get_piece(&self) -> &Piece;
    fn get_side(&self) -> u8;
    fn is_valid_move(&self, destination: &Position, board: &Board) -> bool;

    fn all_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                let destination = Position::new(x as i8, y as i8);
                if self.is_valid_move(&destination, board) {
                    valid_moves.push(destination);
                }
            }
        }
        valid_moves
    }
    fn get_name(&self) -> String;
    fn piece_to_hex(&self) -> String;

    fn get_path_image(&self) -> String {
        // format!("assets/images/{}{}.png", if self.get_side() == 0 {'w'} else {'b'}, self.get_name())
        format!("assets/images/{}.png", self.piece_to_hex().to_lowercase())
    }

    fn shift(&mut self, x: i8, y: i8) {
        let pos: &mut Position = self.get_position_mut();
        pos.x = x;
        pos.y = y;
        self.get_piece_mut().mark_moved();
    }

    fn display(&self) {
        let pos = self.get_position();
        println!("Position : ({}, {})", pos.x, pos.y);
    }
}