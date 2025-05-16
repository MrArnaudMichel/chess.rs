use crate::piece::piece::Piece;

pub struct Pawn {
    piece: Piece
}

impl Pawn {
    pub fn new() -> Self {
        Self {
            piece: Piece::new(0, 0)
        }
    }

    pub fn shift(&mut self, x: u8, y: u8) {
        self.piece.shift(x, y);
    }

    pub fn display(&self) {
        self.piece.display()
    }
}