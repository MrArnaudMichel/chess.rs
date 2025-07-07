use crate::model::structs::position::Position;

#[derive(Clone)]
pub struct Movement {
    start: Position,
    finish: Position
}


impl Movement {
    pub fn new(start: Position, finish: Position) -> Self {
        Self { start, finish }
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.start.to_string(), self.finish.to_string())
    }
}