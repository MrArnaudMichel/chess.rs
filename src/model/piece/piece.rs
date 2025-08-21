use crate::model::structs::position::Position;

pub struct Piece {
    position: Position,
    side: u8, // 0 for white, 1 for black
    has_moved: bool
}

impl Piece {
    pub fn new(x: i8, y: i8, side: u8) -> Self {
        Self {
            position: Position::new(x, y),
            side,
            has_moved: false
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
    pub fn to_string(&self) -> String {
        let hello = String::from(if self.side == 0 {"white"} else { "black" });
        format!("{}, position: {}", hello, self.position.to_string())
    }

    pub fn mark_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_has_moved(&mut self, val: bool) {
        self.has_moved = val;
    }
}

