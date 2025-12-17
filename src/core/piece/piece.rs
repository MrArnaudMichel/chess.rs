/// A simple container for piece state: position, side and movement flag.
///
/// Fields:
/// - `position`: the piece coordinates on the board.
/// - `side`: 0 for white, 1 for black.
/// - `has_moved`: whether the piece has moved (used for castling / pawn first move).
use crate::core::types::position::Position;

pub struct Piece {
    position: Position,
    side: u8,
    move_count: i8,
}

impl Piece {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            position,
            side,
            move_count: 0,
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn get_side(&self) -> u8 {
        self.side
    }

    pub fn mark_moved(&mut self) {
        self.move_count = self.move_count + 1;
    }

    pub fn has_moved(&self) -> bool {
        self.move_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::position::{A1, A2};
    use crate::core::types::color::WHITE;

    #[test]
    fn piece_getters_and_move_flag() {
        let mut piece = Piece::new(A1, WHITE);
        assert_eq!(piece.get_position(), &A1);
        assert_eq!(piece.get_side(), WHITE);
        assert!(!piece.has_moved());
        piece.mark_moved();
        assert!(piece.has_moved());
    }

    #[test]
    fn piece_position_mutation() {
        let mut piece = Piece::new(A1, WHITE);
        let pos = piece.get_position_mut();
        pos.x = A2.x;
        pos.y = A2.y;
        assert_eq!(piece.get_position(), &A2);
    }
}
