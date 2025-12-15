//! King piece implementation and movement validation.
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use super::piece::{Piece};

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

        // Check castling
        if !self.piece.has_moved(){
            let other_piece = board.get_piece(destination);
            if let Some(other_piece) = other_piece {
                if other_piece.get_name() == "rook" && !other_piece.get_piece().has_moved(){
                    if current_pos.y == destination.y {
                        let step = if destination.x > current_pos.x { 1 } else { -1 };
                        let mut x = current_pos.x + step;
                        while x != destination.x {
                            let pos_to_check = Position::new(x, current_pos.y);
                            if board.is_occupied(&pos_to_check) != -1 {
                                return false;
                            }
                            x += step;
                        }
                        return true;
                    }
                }
            }
        }

        let dx = (destination.x - current_pos.x).abs();
        let dy = (destination.y - current_pos.y).abs();

        if dx <= 1 && dy <= 1 {
            if board.is_occupied(destination) == side as i8 {
                return false;
            }
            return true;
        }
        false
    }

    fn get_name(&self) -> String {
        "king".to_string()
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'K')
    }


    fn display(&self) {
        println!("King {}", self.piece.to_string());
    }
}