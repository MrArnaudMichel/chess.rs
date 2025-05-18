use super::piece::{ChessPiece, Piece, Position};

pub struct Pawn {
    piece: Piece,
    has_moved: bool,
}

impl Pawn {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            piece: Piece::new(x, y, 0),
            has_moved: false,
        }
    }
}

// Implémentation du trait ChessPiece pour Pawn
impl ChessPiece for Pawn {
    fn get_position(&self) -> &Position {
        self.piece.get_position()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
    }

    fn is_valid_move(&self, new_x: u8, new_y: u8) -> bool {
        let current_pos = self.get_position();

        if new_x != current_pos.x {
            return false;
        }

        let forward_distance = new_y as i16 - current_pos.y as i16;

        if !self.has_moved {
            forward_distance == 1 || forward_distance == 2
        } else {
            forward_distance == 1
        }
    }
}
