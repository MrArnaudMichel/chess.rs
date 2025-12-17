//! Pawn piece implementation and tests.

use std::any::Any;
use super::piece::Piece;
use crate::core::board::board::Board;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::position::Position;
use crate::core::types::r#move::{ MoveError, MoveOutcome};
use crate::core::types::r#move::MoveError::{BlockedPath, InvalidMove};
use crate::core::types::r#move::MoveOutcome::{Capture, EnPassant, Promotion, Valid};

pub struct Pawn {
    piece: Piece,
    en_passant_vulnerable: bool,
}

impl Pawn {
    pub fn new(position: Position, side: u8) -> Self {
        Self {
            piece: Piece::new(position, side),
            en_passant_vulnerable: false,
        }
    }
}

impl ChessPiece for Pawn {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_position(&self) -> &Position {
        self.piece.get_position()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
    }

    fn get_piece_mut(&mut self) -> &mut Piece {
        &mut self.piece
    }

    fn get_piece(&self) -> &Piece {
        &self.piece
    }

    fn get_side(&self) -> u8 {
        self.piece.get_side()
    }

    fn move_piece(&self, destination: &Position, board: &Board) -> Result<MoveOutcome, MoveError> {
        let current_pos = self.get_position();
        let side = self.get_side();
        let dir: i8 = if side == 0 { 1 } else if side == 1 { -1 } else {
            return Err(InvalidMove);
        };

        let dx = destination.x - current_pos.x;
        let dy = destination.y - current_pos.y;

        if dx.abs() == 1 && dy == dir {

            if board.is_occupied(destination) == ((side ^ 1) as i8){
                if is_promotion_rank(destination, side) {
                    return Ok(Promotion);
                }
                return Ok(Capture);
            }

            if board.get_piece(destination).is_none() {
                let adjacent_pos = Position::new(destination.x, current_pos.y);
                if let Some(piece) = board.get_piece(&adjacent_pos) {
                    if let Some(pawn) = piece.as_any().downcast_ref::<Pawn>() {
                        if pawn.get_side() == (side ^ 1) && pawn.en_passant_vulnerable {
                            return Ok(EnPassant {captured: adjacent_pos});
                        }
                    }
                }
            }
        }


        if dx == 0 && dy == dir {
            if board.is_occupied(destination) >= 0 {
                return Err(BlockedPath)
            }
            if (side == 0 && destination.y == 7) || (side == 1 && destination.y == 0) {
                return Ok(Promotion)
            }
            return Ok(Valid)
        }

        if dx == 0 && dy == 2 * dir {
            if self.piece.has_moved() {
                return Err(InvalidMove)
            }

            let intermediate = Position::new(current_pos.x, current_pos.y + dir);
            if board.is_occupied(&intermediate) >= 0{
                return Err(BlockedPath)
            }
            if board.is_occupied(destination) >= 0{
                return Err(BlockedPath)
            }
            return Ok(Valid)
        }
        Err(InvalidMove)
    }

    fn get_name(&self) -> String {
        "pawn".to_string()
    }

    fn piece_to_hex(&self) -> String {
        format!("{}{}", if self.get_side() == 0 {'W'} else {'B'}, 'P')
    }

    fn shift(&mut self, x: i8, y: i8) {
        let dy = y - self.get_position().y;
        if dy.abs() == 2 {
            self.en_passant_vulnerable = true;
        } else {
            self.en_passant_vulnerable = false;
        }
        let pos: &mut Position = self.get_position_mut();
        pos.x = x;
        pos.y = y;
        self.get_piece_mut().mark_moved();
    }
}

fn is_promotion_rank(position: &Position, side: u8) -> bool {
    (side == 0 && position.y == 7) || (side == 1 && position.y == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::board::board::Board;
    use crate::core::types::color::{BLACK, WHITE};
    use crate::core::types::position::{B2, B3, B4, C3, G7, G8, B5, C7, C6};
    use crate::core::piece::rook::Rook;
    use crate::core::types::r#move::MoveOutcome::EnPassant;

    fn setup_pawn() -> (Board, Pawn) {
        let board = Board::new();
        let pawn = Pawn::new(B2, WHITE);
        (board, pawn)
    }

    #[test]
    fn pawn_moves_one_forward() {
        let (board, pawn) = setup_pawn();
        assert!(pawn.move_piece(&B3, &board).is_ok());
    }

    #[test]
    fn pawn_moves_two_forward_first_move() {
        let (board, pawn) = setup_pawn();
        assert!(pawn.move_piece(&B4, &board).is_ok());
    }

    #[test]
    fn pawn_blocked_forward() {
        let (mut board, pawn) = setup_pawn();
        board.place_piece(Box::new(Rook::new(B3, BLACK)));
        assert!(!pawn.move_piece(&B4, &board).is_ok());
    }

    #[test]
    fn pawn_can_capture_diagonally() {
        let (mut board, pawn) = setup_pawn();
        board.place_piece(Box::new(Pawn::new(C3, BLACK)));
        assert!(pawn.move_piece(&C3, &board).is_ok());
    }

    #[test]
    fn pawn_promotion_detected() {
        let board = Board::new();
        let pawn = Pawn::new(G7, WHITE);
        let outcome = pawn.move_piece(&G8, &board).unwrap();
        // promotion variant should be returned
        match outcome {
            MoveOutcome::Promotion => {}
            _ => panic!("Expected Promotion outcome"),
        }
    }

    #[test]
    fn pawn_en_passant_capture() {
        let mut board = Board::new();
        // white pawn that will perform en-passant
        let pawn = Pawn::new(B5, WHITE);

        // simulate a black pawn that moved two squares from C7 to C5 and is vulnerable
        let mut enemy = Pawn::new(C7, BLACK);
        enemy.shift(2, 4); // now at C5 and en_passant_vulnerable == true
        board.place_piece(Box::new(enemy));

        // attempt en-passant: white pawn moves from B5 to C6 capturing pawn at C5
        let outcome = pawn.move_piece(&C6, &board).unwrap();
        match outcome {
            EnPassant { captured } => assert_eq!(captured, crate::core::types::position::Position::new(2, 4)),
            _ => panic!("Expected EnPassant outcome"),
        }
    }
}
