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

impl ChessPiece for Pawn {
    fn get_position(&self) -> &Position {
        self.piece.get_position()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
    }

    fn is_valid_move(&self, destination: &Position) -> bool {
        let current_pos = self.get_position();

        if destination.x != current_pos.x {
            return false;
        }

        let forward_distance = destination.y - current_pos.y;

        if !self.has_moved {
            forward_distance == 1 || forward_distance == 2
        } else {
            forward_distance == 1
        }
    }

    fn display(&self) {
        println!("Pawn {}", self.piece.to_string());
    }

}
