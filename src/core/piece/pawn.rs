//! Pawn piece implementation and tests.
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use super::piece::{Piece};

pub struct Pawn {
    piece: Piece
}

impl Pawn {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            piece: Piece::new(x, y, side)
        }
    }
}

impl ChessPiece for Pawn {
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

    fn is_valid_move(&self, destination: &Position, board: &Board) -> bool {
        let current_pos = self.get_position();
        let side = self.get_side();
        let dir: i8 = if side == 0 { 1 } else if side == 1 { -1 } else {
            println!("Côté invalide");
            return false;
        };

        let dx = destination.x - current_pos.x;
        let dy = destination.y - current_pos.y;

        if dx.abs() == 1 && dy == dir {
            if board.is_occupied(destination) == ((side ^ 1) as i8) {
                return true;
            }
            return false;
        }

        if dx == 0 && dy == dir {
            if board.is_occupied(destination) >= 0 {
                return false;
            }
            return true;
        }

        if dx == 0 && dy == 2 * dir {
            if self.piece.has_moved() {
                return false;
            }

            let intermediate = Position::new(current_pos.x, current_pos.y + dir);
            if board.is_occupied(&intermediate) >= 0{
                return false;
            }
            if board.is_occupied(destination) >= 0{
                return false;
            }
            return true;
        }
        false
    }

    fn get_name(&self) -> String {
        "pawn".to_string()
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'P')
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::board::board::Board;
    use crate::core::types::position::Position;

    fn empty_board() -> Board {
        Board::new()
    }

    #[test]
    fn test_pawn_valid_moves() {
        let mut board = empty_board();
        let mut white_pawn = Pawn::new(4, 1, 0);
        board.add_piece(Box::new(white_pawn));

        let mut black_pawn = Pawn::new(3, 6, 1);
        board.add_piece(Box::new(black_pawn));

        assert!(board.get_piece(&Position::new(4, 1)).unwrap().is_valid_move(&Position::new(4, 3), &board));
        assert!(board.get_piece(&Position::new(4, 1)).unwrap().is_valid_move(&Position::new(4, 2), &board));
        assert!(board.get_piece(&Position::new(3, 6)).unwrap().is_valid_move(&Position::new(3, 4), &board));
        assert!(board.get_piece(&Position::new(3, 6)).unwrap().is_valid_move(&Position::new(3, 5), &board));
        assert!(!board.get_piece(&Position::new(4, 1)).unwrap().is_valid_move(&Position::new(5, 3), &board));
    }

    #[test]
    fn test_pawn_capture() {
        let mut board = empty_board();
        let white_pawn = Pawn::new(4, 4, 0);
        board.add_piece(Box::new(white_pawn));
        let black_pawn = Pawn::new(5, 5, 1);
        board.add_piece(Box::new(black_pawn));
        assert!(board.get_piece(&Position::new(4, 4)).unwrap().is_valid_move(&Position::new(5, 5), &board));
    }

    #[test]
    fn test_pawn_blocked_move() {
        let mut board = empty_board();
        let white_pawn = Pawn::new(4, 1, 0);
        board.add_piece(Box::new(white_pawn));
        let blocking_piece = Pawn::new(4, 2, 1);
        board.add_piece(Box::new(blocking_piece));
        assert!(!board.get_piece(&Position::new(4, 1)).unwrap().is_valid_move(&Position::new(4, 2), &board));
    }

    #[test]
    fn test_pawn_en_passant() {
        let mut board = empty_board();
        let white_pawn = Pawn::new(4, 4, 0);
        board.add_piece(Box::new(white_pawn));
        let black_pawn = Pawn::new(5, 6, 1);
        board.add_piece(Box::new(black_pawn));
        assert!(board.get_piece(&Position::new(5, 6)).unwrap().is_valid_move(&Position::new(5, 4), &board));

        assert!(board.get_piece(&Position::new(4, 4)).unwrap().is_valid_move(&Position::new(5, 5), &board));
    }
}