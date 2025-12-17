use std::any::Any;
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use crate::core::types::r#move::{MoveError, MoveOutcome};
use crate::core::types::r#move::MoveError::InvalidMove;
use crate::core::types::r#move::MoveOutcome::{Capture, Valid};
use super::piece::{Piece};

pub struct Knight {
    piece: Piece
}

impl Knight {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            piece: Piece::new(position, side)
        }
    }
}

impl ChessPiece for Knight {
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
        
        if current_pos == destination || !board.is_within_bounds(destination) {
            return Err(InvalidMove)
        }
        
        let dx = (destination.x - current_pos.x).abs();
        let dy = (destination.y - current_pos.y).abs();
        
        if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
            if board.is_occupied(destination) == side as i8 {
                return Err(InvalidMove)
            } else if board.is_occupied(destination) == !side as i8 { 
                return Ok(Capture)
            }
            return Ok(Valid)
        }
        Err(InvalidMove)
    }

    fn get_name(&self) -> String {
        "knight".to_string()
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'N')
    }
}