use crate::core::types::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveOutcome {
    Valid,
    Capture,
    EnPassant { captured: Position },
    Castling,
    Promotion,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    NoPiece,
    BlockedPath,
    WrongTurn,
    InvalidMove,
    KingInCheck,
}
