use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use super::piece::{Piece};


pub struct Rook {
    piece: Piece
}

impl Rook {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            piece: Piece::new(x, y, side)
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
    use crate::core::types::position::Position;

    fn empty_board() -> Board {
        Board::new()
    }

    #[test]
    fn rook_can_move_vertically() {
        let board = empty_board();
        let rook = Rook::new(0, 0, 0);

        let destination = Position::new(0, 5);

        assert!(rook.is_valid_move(&destination, &board));
    }

    #[test]
    fn rook_can_move_horizontally() {
        let board = empty_board();
        let rook = Rook::new(0, 0, 0);

        let destination = Position::new(5, 0);

        assert!(rook.is_valid_move(&destination, &board));
    }

    #[test]
    fn rook_cannot_move_diagonally() {
        let board = empty_board();
        let rook = Rook::new(0, 0, 0);

        let destination = Position::new(3, 3);

        assert!(!rook.is_valid_move(&destination, &board));
    }

    #[test]
    fn rook_cannot_move_if_blocked() {
        let mut board = empty_board();
        let rook = Rook::new(0, 0, 0);

        board.add_piece(Box::new(Rook::new(0, 3, 1)));

        let destination = Position::new(0, 5);

        assert!(!rook.is_valid_move(&destination, &board));
    }

    #[test]
    fn rook_can_capture_enemy() {
        let mut board = empty_board();
        let rook = Rook::new(0, 0, 0);

        board.add_piece(Box::new(Rook::new(0, 5, 1)));
        let destination = Position::new(0, 5);
        assert!(rook.is_valid_move(&destination, &board));
    }

    #[test]
    fn rook_cannot_capture_ally() {
        let mut board = empty_board();
        let rook = Rook::new(0, 0, 0);

        board.add_piece(Box::new(Rook::new(0, 5, 0)));

        let destination = Position::new(0, 5);

        assert!(!rook.is_valid_move(&destination, &board));
    }
}
