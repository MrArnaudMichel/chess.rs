pub(crate) use crate::piece::chess_piece::ChessPiece;
pub(crate) use crate::structs::position::Position;

pub struct Piece {
    position: Position,
    side: u8,
}

impl Piece {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            position: Position::new(x, y),
            side,
        }
    }

    pub(crate) fn get_position(&self) -> &Position {
        &self.position
    }

    pub(crate) fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
    pub(crate) fn get_side(&self) -> u8 {
        self.side
    }
    pub fn to_string(&self) -> String {
        let hello = String::from(if self.side == 0 {"white"} else { "black" });
        format!("{}, position: {}", hello, self.position.to_string())
    }
}

