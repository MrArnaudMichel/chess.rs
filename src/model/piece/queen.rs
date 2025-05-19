use crate::model::board::board::Board;
use super::piece::{ChessPiece, Piece, Position};

pub struct Queen {
    piece: Piece
}

impl Queen {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            piece: Piece::new(x, y, side)
        }
    }
}


impl ChessPiece for Queen {
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
        let current_position = self.get_position();
        let (current_x, current_y) = (current_position.x, current_position.y);
        let (dest_x, dest_y) = (destination.x, destination.y);

        if current_x == dest_x && current_y == dest_y {
            return false;
        }

        let dx = (dest_x - current_x).abs();
        let dy = (dest_y - current_y).abs();

        if dx != 0 && dy != 0 && dx != dy {
            return false;
        }

        // Check if the path is clear
        let step_x = if dest_x > current_x { 1 } else if dest_x < current_x { -1 } else { 0 };
        let step_y = if dest_y > current_y { 1 } else if dest_y < current_y { -1 } else { 0 };

        let mut x = current_x + step_x;
        let mut y = current_y + step_y;

        while (x, y) != (dest_x, dest_y) {
            if board.is_occupied(&Position::new(x, y)) >= 0{
                return false;
            }

            x += step_x;
            y += step_y;
        }

        if let Some(piece) = board.get_piece(destination) {
            return piece.get_side() != self.get_side();
        }

        true
    }


    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'Q')
    }


    fn display(&self) {
        println!("Pawn {}", self.piece.to_string());
    }
}