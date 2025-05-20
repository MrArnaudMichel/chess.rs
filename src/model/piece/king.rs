use crate::model::board::board::Board;
use super::piece::{ChessPiece, Piece, Position};

pub struct King {
    piece: Piece
}

impl King {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            piece: Piece::new(x, y, side)
        }
    }
}

impl ChessPiece for King {
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

        if current_pos == destination {
            return false;
        }

        let dx = (destination.x - current_pos.x).abs();
        let dy = (destination.y - current_pos.y).abs();

        if dx <= 1 || dy <= 1 {
            if board.is_occupied(destination) == side as i8 {
                println!("Destination occupied");
                return false;
            }
            return true;
        }
        println!("Movement impossible car case non assignable");
        false
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'T')
    }


    fn display(&self) {
        println!("King {}", self.piece.to_string());
    }
}