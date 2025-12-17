//! Queen piece movement and validation.

use std::any::Any;
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use crate::core::types::r#move::MoveError::{BlockedPath, InvalidMove};
use crate::core::types::r#move::{MoveError, MoveOutcome};
use crate::core::types::r#move::MoveOutcome::{Capture, Valid};
use super::piece::{Piece};
pub struct Queen {
    piece: Piece
}

impl Queen {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            piece: Piece::new(position, side)
        }
    }
}


impl ChessPiece for Queen {
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
        let current_position = self.get_position();
        let (current_x, current_y) = (current_position.x, current_position.y);
        let (dest_x, dest_y) = (destination.x, destination.y);

        if current_x == dest_x && current_y == dest_y  || !board.is_within_bounds(destination) {
            return Err(InvalidMove)
        }

        let dx = (dest_x - current_x).abs();
        let dy = (dest_y - current_y).abs();

        if dx != 0 && dy != 0 && dx != dy {
            return Err(InvalidMove)
        }

        let step_x = if dest_x > current_x { 1 } else if dest_x < current_x { -1 } else { 0 };
        let step_y = if dest_y > current_y { 1 } else if dest_y < current_y { -1 } else { 0 };

        let mut x = current_x + step_x;
        let mut y = current_y + step_y;

        while (x, y) != (dest_x, dest_y) {
            if board.is_occupied(&Position::new(x, y)) >= 0{
                return Err(BlockedPath);
            }

            x += step_x;
            y += step_y;
        }

        if let Some(piece) = board.get_piece(destination) {
            if piece.get_side() != self.get_side() {
                return Ok(Capture);
            } 
            return Err(InvalidMove);
        }
        Ok(Valid)
    }

    fn get_name(&self) -> String {
        "queen".to_string()
    }


    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'Q')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::board::board::Board;
    use crate::core::types::color::{BLACK, WHITE};
    use crate::core::types::position::{B2, C2, D2};


    fn setup_queen() -> (Board, Queen) {
        let board = Board::new();
        let queen = Queen::new(B2, WHITE);
        (board, queen)
    }

    #[test]
    fn queen_can_move_vertically() {
        let (board, queen) = setup_queen();
        assert!(queen.move_piece(&Position::new(1, 5), &board).is_ok());
        assert!(queen.move_piece(&Position::new(1, 0), &board).is_ok());
    }

    #[test]
    fn queen_can_move_horizontally() {
        let (board, queen) = setup_queen();
        assert!(queen.move_piece(&Position::new(4, 1), &board).is_ok());
        assert!(queen.move_piece(&Position::new(0, 1), &board).is_ok());
    }

    #[test]
    fn queen_can_move_diagonally() {
        let (board, queen) = setup_queen();
        assert!(queen.move_piece(&Position::new(4, 4), &board).is_ok());
        assert!(queen.move_piece(&Position::new(0, 0), &board).is_ok());
    }

    #[test]
    fn queen_cannot_move_invalidly() {
        let (board, queen) = setup_queen();
        assert!(!queen.move_piece(&Position::new(3, 4), &board).is_ok());
        assert!(!queen.move_piece(&Position::new(2, 3), &board).is_ok());
    }

    #[test]
    fn queen_cannot_move_if_blocked() {
        let (mut board, queen) = setup_queen();
        board.place_piece(Box::new(Queen::new(C2, BLACK)));
        assert!(!queen.move_piece(&D2, &board).is_ok());
    }

    #[test]
    fn queen_can_capture_enemy() {
        let (mut board, queen) = setup_queen();
        board.place_piece(Box::new(Queen::new(D2, BLACK)));
        assert!(queen.move_piece(&D2, &board).is_ok());
    }

    #[test]
    fn queen_cannot_capture_ally() {
        let (mut board, queen) = setup_queen();
        board.place_piece(Box::new(Queen::new(D2, WHITE)));
        assert!(!queen.move_piece(&D2, &board).is_ok());
    }
}