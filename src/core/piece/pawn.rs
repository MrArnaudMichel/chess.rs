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