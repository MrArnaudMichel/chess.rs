use crate::model::board::board::Board;
use super::piece::{ChessPiece, Piece, Position};

pub struct Bishop {
    piece: Piece
}

impl Bishop {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            piece: Piece::new(x, y, side)
        }
    }
}

impl ChessPiece for Bishop {
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

        let dx = destination.x - current_pos.x;
        let dy = destination.y - current_pos.y;

        if dx.abs() != dy.abs() {
            return false;
        }

        let step_x = if dx > 0 { 1 } else { -1 };
        let step_y = if dy > 0 { 1 } else { -1 };

        let mut x = current_pos.x + step_x;
        let mut y = current_pos.y + step_y;

        while x != destination.x && y != destination.y {
            if board.is_occupied(&Position::new(x, y)) >= 0 {
                return false;
            }

            x += step_x;
            y += step_y;
        }

        let value = board.is_occupied(&Position::new(destination.x, destination.y));
        if value == ((side ^ 1) as i8) || value == -1 {
            return true;
        }
        false
    }

    fn get_name(&self) -> String {
        "bishop".to_string()
    }


    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'B')
    }


    fn display(&self) {
        println!("Bishop {}", self.piece.to_string());
    }

}