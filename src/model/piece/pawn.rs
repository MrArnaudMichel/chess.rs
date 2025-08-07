use crate::model::board::board::Board;
use crate::model::piece::chess_piece::ChessPiece;
use crate::model::structs::position::Position;
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


    fn display(&self) {
        println!("Pawn {}", self.piece.to_string());
    }

}