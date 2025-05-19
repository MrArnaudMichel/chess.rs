use crate::board::board::Board;
use super::piece::{ChessPiece, Piece, Position};

pub struct Pawn {
    piece: Piece,
    has_moved: bool,
}

impl Pawn {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            piece: Piece::new(x, y, side),
            has_moved: false,
        }
    }
}

impl ChessPiece for Pawn {
    fn get_position(&self) -> &Position {
        self.piece.get_position()
    }

    fn get_side(&self) -> u8 {
        self.piece.get_side()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
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
            if board.is_occupied(destination) ==  ((side ^ 1) as i8) {
                return true;
            } else if board.is_occupied(destination) == side as i8{
                println!("Impossible de capturer une pièce alliée.");
            } else {
                println!("Aucune pièce à capturer sur la diagonale.");
            }
            return false;
        }

        if dx == 0 && dy == dir {
            if board.is_occupied(destination) >= 0 {
                println!("Case devant occupée.");
                return false;
            }
            return true;
        }

        if dx == 0 && dy == 2 * dir {
            if self.has_moved {
                println!("Ce pion a déjà bougé, il ne peut avancer de deux cases.");
                return false;
            }

            let intermediate = Position::new(current_pos.x, current_pos.y + dir);
            if board.is_occupied(&intermediate) >= 0{
                println!("La case intermédiaire est occupée.");
                return false;
            }
            if board.is_occupied(destination) >= 0{
                println!("La case de destination est occupée.");
                return false;
            }
            return true;
        }

        println!("Mouvement invalide pour un pion.");
        false
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'P')
    }


    fn display(&self) {
        println!("Pawn {}", self.piece.to_string());
    }

}