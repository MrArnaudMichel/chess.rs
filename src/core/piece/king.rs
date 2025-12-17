//! King piece implementation and movement validation.

use std::any::Any;
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use crate::core::types::r#move::{MoveError, MoveOutcome};
use crate::core::types::r#move::MoveError::{BlockedPath, InvalidMove};
use crate::core::types::r#move::MoveOutcome::{Castling, Valid};
use super::piece::{Piece};

pub struct King {
    piece: Piece
}

impl King {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            piece: Piece::new(position, side)
        }
    }
}

impl ChessPiece for King {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_position(&self) -> &Position {
        self.piece.get_position()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
    }

    fn get_piece_mut(&mut self) -> &mut Piece {
        &mut self.piece
    }


    fn get_piece(&self) -> &Piece {
        &self.piece
    }


    fn get_side(&self) -> u8 {
        self.piece.get_side()
    }

    fn move_piece(&self, destination: &Position, board: &Board) -> Result<MoveOutcome, MoveError> {
        let current_pos = self.get_position();
        let side = self.get_side();

        if current_pos == destination || !board.is_within_bounds(destination){
            return Err(InvalidMove)
        }

        if !self.piece.has_moved(){
            let other_piece = board.get_piece(destination);
            if let Some(other_piece) = other_piece {
                if other_piece.get_name() == "rook" && !other_piece.get_piece().has_moved(){
                    if current_pos.y == destination.y {
                        let step = if destination.x > current_pos.x { 1 } else { -1 };
                        let mut x = current_pos.x + step;
                        while x != destination.x {
                            let pos_to_check = Position::new(x, current_pos.y);
                            if board.is_occupied(&pos_to_check) >= 0 {
                                return Err(BlockedPath)
                            }
                            x += step;
                        }
                        return Ok(Castling)
                    }
                }
            }
        }

        let dx = (destination.x - current_pos.x).abs();
        let dy = (destination.y - current_pos.y).abs();

        if dx <= 1 && dy <= 1 {
            if board.is_occupied(destination) == side as i8 {
                return Err(BlockedPath)
            }
            return Ok(Valid)
        }
        Err(InvalidMove)
    }

    fn get_name(&self) -> String {
        "king".to_string()
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'K')
    }
}