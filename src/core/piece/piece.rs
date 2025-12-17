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
