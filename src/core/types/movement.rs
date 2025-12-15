use crate::core::types::position::Position;

#[derive(Clone)]
pub struct Movement {
    from: Position,
    to: Position
}


impl Movement {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.from.to_string(), self.to.to_string())
    }

    pub fn initial_position(&self) -> Position {
        self.from.clone()
    }

    pub fn finish_position(&self) -> Position {
        self.to.clone()
    }
}