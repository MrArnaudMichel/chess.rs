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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::board::board::Board;
    use crate::core::types::color::{WHITE, BLACK};
    use crate::core::types::position::{B1, C3, A3, B3};

    fn setup_knight() -> (Board, Knight) {
        let board = Board::new();
        let knight = Knight::new(B1, WHITE);
        (board, knight)
    }

    #[test]
    fn knight_moves_l_shape() {
        let (board, knight) = setup_knight();
        assert!(knight.move_piece(&C3, &board).is_ok());
        assert!(knight.move_piece(&A3, &board).is_ok());
    }

    #[test]
    fn knight_ignores_blockers() {
        let (mut board, knight) = setup_knight();
        // place a blocker on B2 which would block other pieces but not the knight
        board.place_piece(Box::new(Knight::new(B3, BLACK)));
        assert!(knight.move_piece(&C3, &board).is_ok());
    }

    #[test]
    fn knight_captures_enemy() {
        let (mut board, knight) = setup_knight();
        board.place_piece(Box::new(Knight::new(C3, BLACK)));
        assert!(knight.move_piece(&C3, &board).is_ok());
    }

    #[test]
    fn knight_invalid_move() {
        let (board, knight) = setup_knight();
        assert!(!knight.move_piece(&B3, &board).is_ok());
    }
}
