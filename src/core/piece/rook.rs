use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use super::piece::{Piece};


pub struct Rook {
    piece: Piece
}

impl Rook {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            piece: Piece::new(position, side)
        }
    }
}

impl ChessPiece for Rook {
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



        if current_pos == destination || (current_pos.x != destination.x && current_pos.y != destination.y) {
            return false;
        }

        if board.is_occupied(destination) == side as i8 {
            return false;
        }

        let positions_to_check = if current_pos.x != destination.x {
            let range = if current_pos.x < destination.x {
                (current_pos.x + 1)..destination.x
            } else {
                (destination.x + 1)..current_pos.x
            };
            range.map(|x| Position::new(x, current_pos.y)).collect::<Vec<_>>()
        } else {
            let range = if current_pos.y < destination.y {
                (current_pos.y + 1)..destination.y
            } else {
                (destination.y + 1)..current_pos.y
            };
            range.map(|y| Position::new(current_pos.x, y)).collect::<Vec<_>>()
        };

        for pos in positions_to_check {
            if board.is_occupied(&pos) >= 0 {
                return false;
            }
        }
        true
    }

    fn get_name(&self) -> String {
        "rook".to_string()
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'R')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::board::board::Board;
    use crate::core::types::color::{BLACK, WHITE};
    use crate::core::types::position::{A1, A2, A4, D4, F1};

    fn setup_rook() -> (Board, Rook) {
        let board = Board::new();
        let rook = Rook::new(A1, WHITE);
        (board, rook)
    }

    #[test]
    fn rook_can_move_vertically() {
        let (board, rook) = setup_rook();
        assert!(rook.is_valid_move(&A4, &board));
    }

    #[test]
    fn rook_can_move_horizontally() {
        let (board, rook) = setup_rook();
        assert!(rook.is_valid_move(&F1, &board));
    }

    #[test]
    fn rook_cannot_move_diagonally() {
        let (board, rook) = setup_rook();
        assert!(!rook.is_valid_move(&D4, &board));
    }

    #[test]
    fn rook_cannot_move_if_blocked() {
        let (mut board, rook) = setup_rook();

        board.place_piece(Box::new(Rook::new(A2, BLACK)));

        assert!(!rook.is_valid_move(&A4, &board));
    }

    #[test]
    fn rook_can_capture_enemy() {
        let (mut board, rook) = setup_rook();

        board.place_piece(Box::new(Rook::new(A4, BLACK)));

        assert!(rook.is_valid_move(&A4, &board));
    }

    #[test]
    fn rook_cannot_capture_ally() {
        let (mut board, rook) = setup_rook();

        board.place_piece(Box::new(Rook::new(A4, WHITE)));

        assert!(!rook.is_valid_move(&A4, &board));
    }
}

