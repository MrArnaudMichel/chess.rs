//! Bishop piece movement validation.

use std::any::Any;
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use crate::core::types::r#move::{MoveError, MoveOutcome};
use crate::core::types::r#move::MoveError::{BlockedPath, InvalidMove};
use crate::core::types::r#move::MoveOutcome::{Capture, Valid};
use super::piece::{Piece};

pub struct Bishop {
    piece: Piece
}

impl Bishop {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            piece: Piece::new(position, side)
        }
    }
}

impl ChessPiece for Bishop {
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

        let dx = destination.x - current_pos.x;
        let dy = destination.y - current_pos.y;

        if dx.abs() != dy.abs() || !board.is_within_bounds(destination) {
            return Err(InvalidMove);
        }

        let step_x = if dx > 0 { 1 } else { -1 };
        let step_y = if dy > 0 { 1 } else { -1 };

        let mut x = current_pos.x + step_x;
        let mut y = current_pos.y + step_y;

        while x != destination.x && y != destination.y && board.is_within_bounds(&Position::new(x, y)) {
            if board.is_occupied(&Position::new(x, y)) >= 0 {
                return Err(BlockedPath)
            }
            x += step_x;
            y += step_y;
        }

        if destination.x < 0 || destination.x >= 8 || destination.y < 0 || destination.y >= 8 {
            return Err(InvalidMove)
        }
        let value = board.is_occupied(&Position::new(destination.x, destination.y));
        if value == ((side ^ 1) as i8) {
            return Ok(Capture)
        } else if value == -1 {
            return Ok(Valid)
        }
        Err(InvalidMove)
    }

    fn get_name(&self) -> String {
        "bishop".to_string()
    }


    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'B')
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::board::board::Board;
    use crate::core::types::color::{BLACK, WHITE};
    use crate::core::types::position::{C1, D2, E3, C4};
    use crate::core::piece::queen::Queen;

    fn setup_bishop() -> (Board, Bishop) {
        let board = Board::new();
        let bishop = Bishop::new(C1, WHITE);
        (board, bishop)
    }

    #[test]
    fn bishop_moves_diagonally() {
        let (board, bishop) = setup_bishop();
        assert!(bishop.move_piece(&E3, &board).is_ok());
    }

    #[test]
    fn bishop_blocked_path() {
        let (mut board, bishop) = setup_bishop();
        board.place_piece(Box::new(Queen::new(D2, BLACK)));
        assert!(!bishop.move_piece(&E3, &board).is_ok());
    }

    #[test]
    fn bishop_captures_enemy() {
        let (mut board, bishop) = setup_bishop();
        board.place_piece(Box::new(Bishop::new(E3, BLACK)));
        assert!(bishop.move_piece(&E3, &board).is_ok());
    }

    #[test]
    fn bishop_invalid_straight_move() {
        let (board, bishop) = setup_bishop();
        assert!(!bishop.move_piece(&C4, &board).is_ok());
    }
}
