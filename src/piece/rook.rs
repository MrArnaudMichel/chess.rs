use std::cmp::PartialEq;
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

        if (current_pos.x != destination.x && current_pos.y != current_pos.y) || current_pos == destination {
            println!("Mouvement impossible car case non joiniable ou case = current");
            return false;
        }

        if board.is_occupied(destination) == side as i8 {
            println!("Destination occupée");
            return false;
        }

        if current_pos.x != destination.x {

            let range = if current_pos.x > destination.x {
                destination.x..current_pos.x
            } else {
                (current_pos.x + 1)..destination.x
            };
            for i in range {
                let test_pos = Position::new(i, current_pos.y);
                print!("Test position  {}", test_pos.to_string());
                if board.is_occupied(&test_pos) >= 0 {
                    return false;
                }
            }
            return true;
        }
        let range = if current_pos.y > destination.y {
            (destination.y + 1)..current_pos.y
        } else {
            (current_pos.y + 1)..(destination.y - 1)
        };
        for i in range {
            let test_pos = Position::new(current_pos.x, i);
            if board.is_occupied(&test_pos) >= 0 {
                return false;
            }
        }
        return true;
        
        false
    }

    fn piece_to_hexa(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'R')
    }


    fn display(&self) {
        println!("Pawn {}", self.piece.to_string());
    }

}