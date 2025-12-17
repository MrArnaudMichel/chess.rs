use std::any::Any;

use crate::core::board::board::Board;
use crate::core::piece::piece::{Piece};
use crate::core::types::r#move::{MoveError, MoveOutcome};
use crate::core::types::position::Position;

pub trait ChessPiece: Any {
    fn as_any(&self) -> &dyn Any;
    fn get_position(&self) -> &Position;
    fn get_position_mut(&mut self) -> &mut Position;
    fn get_piece_mut(&mut self) -> &mut Piece;

    fn get_piece(&self) -> &Piece;
    fn get_side(&self) -> u8;
    fn move_piece(&self, destination: &Position, board: &Board) -> Result<MoveOutcome, MoveError>;

    #[allow(dead_code)]
    fn all_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                let destination = Position::new(x as i8, y as i8);
                if self.move_piece(&destination, board).is_ok() {
                    valid_moves.push(destination);
                }
            }
        }
        valid_moves
    }
    fn get_name(&self) -> String;
    fn piece_to_hex(&self) -> String;

    #[allow(dead_code)]
    fn get_path_image(&self) -> String {
        format!("assets/images/{}.png", self.piece_to_hex().to_lowercase())
    }

    fn shift(&mut self, x: i8, y: i8) {
        // debug
        println!("CHESS_PIECE DEBUG: shift called for piece {} to ({}, {})", self.get_name(), x, y);
        let pos: &mut Position = self.get_position_mut();
        pos.x = x;
        pos.y = y;
        (&mut *self).get_piece_mut().mark_moved();
    }
}