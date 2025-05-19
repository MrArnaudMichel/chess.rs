use crate::board::board::Board;
use super::piece::{ChessPiece, Piece, Position};

pub struct Rook {
    piece: Piece,
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

    fn get_side(&self) -> u8 {
        self.piece.get_side()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
    }

    fn is_valid_move(&self, destination: &Position, board: &Board) -> bool {
        let current_pos = self.get_position();
        let side = self.get_side();

        if current_pos == destination || (current_pos.x != destination.x && current_pos.y != destination.y) {
            println!("Mouvement impossible car case non joignable ou case = current");
            return false;
        }

        if board.is_occupied(destination) == side as i8 {
            println!("Destination occupée");
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
                println!("Chemin obstrué à {}", pos.to_string());
                return false;
            }
        }
        true
    }


    fn piece_to_hexa(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'R')
    }


    fn display(&self) {
        println!("Pawn {}", self.piece.to_string());
    }

}