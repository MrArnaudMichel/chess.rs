#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    NoPiece,
    WrongTurn,
    InvalidMove,
    KingInCheck,
}
