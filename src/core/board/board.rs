//! Board representation and utilities: stores pieces and provides movement/check detection.
use crate::core::types::position::Position;
use crate::core::piece::chess_piece::ChessPiece;
use crate::core::types::r#move::{MoveError, MoveOutcome};
use crate::core::types::r#move::MoveError::{KingInCheck, NoPiece, WrongTurn};

pub struct Board {
    pieces: [[Option<Box<dyn ChessPiece>>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }

    pub fn place_piece(&mut self, piece: Box<dyn ChessPiece>) -> &mut Self{
        let pos = piece.get_position().clone();

        if pos.x < 8 && pos.y < 8 {
            self.pieces[pos.y as usize][pos.x as usize] = Some(piece);
        } else {
            println!("Position invalide : la pièce doit être dans un tableau 8x8");
        }
        self
    }

    pub fn get_piece(&self, position: &Position) -> Option<&Box<dyn ChessPiece>> {
        if position.x < 8 && position.y < 8 {
            self.pieces[position.y as usize][position.x as usize].as_ref()
        } else {
            None
        }
    }

    /// Returns:
    /// - 0 if occupied by side WHITE
    /// - 1 if occupied by side BLACK
    /// - -1 if not occupied
    /// - -2 if out of bounds
    pub fn is_occupied(&self, position: &Position) -> i8 {
        if position.x < 8 && position.y < 8 {
            if let Some(piece) = self.pieces[position.y as usize][position.x as usize].as_ref() {
                piece.get_side() as i8
            } else {
                -1
            }
        } else {
            -2
        }
    }


    pub fn display_all(&self) {
        for y in (0..8).rev() {
            print!("{} ", y + 1);
            for x in 0..8 {
                match &self.pieces[y][x] {
                    Some(piece) => print!("{} ", piece.piece_to_hex()),
                    None => print!("-- "),
                }
            }
            println!();
        }
        println!("  A  B  C  D  E  F  G  H");
    }


    pub fn move_piece(
        &mut self,
        from: Position,
        to: Position,
        turn: u8,
    ) -> Result<MoveOutcome, MoveError> {
        let piece = self.get_piece(&from).ok_or(NoPiece)?;

        if piece.get_side() != turn {
            return Err(WrongTurn);
        }

        let outcome = piece.move_piece(&to, self)?;

        let captured_piece = self._move_piece_internal(from.clone(), to.clone())?;

        let mut en_passant_captured: Option<Box<dyn ChessPiece>> = None;
        if let MoveOutcome::EnPassant { captured } = &outcome {
            let (x, y) = (captured.x as usize, captured.y as usize);
            en_passant_captured = self.pieces[y][x].take();
        }

        if self.is_checked(turn, None) {
            let _ = self._move_piece_internal(to.clone(), from.clone());

            self.pieces[to.y as usize][to.x as usize] = captured_piece;

            if let MoveOutcome::EnPassant { captured } = outcome {
                let (x, y) = (captured.x as usize, captured.y as usize);
                self.pieces[y][x] = en_passant_captured;
            }

            return Err(KingInCheck);
        }

        Ok(outcome)
    }



    /// Internal move function: Moves piece and returns the captured piece (if any)
    /// This allows us to "Undo" a move.
    fn _move_piece_internal(&mut self, from: Position, to: Position) -> Result<Option<Box<dyn ChessPiece>>, MoveError> {
        let (from_x, from_y) = (from.x as usize, from.y as usize);
        let (to_x, to_y) = (to.x as usize, to.y as usize);

        let mut piece = self.pieces[from_y][from_x].take().ok_or(NoPiece)?;

        piece.shift(to.x, to.y);

        let captured = self.pieces[to_y][to_x].take();

        self.pieces[to_y][to_x] = Some(piece);

        Ok(captured)
    }

    pub fn is_checked(&self, side: u8, position: Option<Position>) -> bool {
        let mut king_position: Option<Position> = position;

        if king_position.is_none() {
            'outer: for y in 0..8 {
                for x in 0..8 {
                    if let Some(piece) = &self.pieces[y][x] {
                        if piece.get_side() == side && piece.get_name().to_lowercase() == "king" {
                            king_position = Some(Position::new(x as i8, y as i8));
                            break 'outer;
                        }
                    }
                }
            }
        }

        let king_pos = match king_position {
            Some(pos) => pos,
            None => {
                return false;
            }
        };

        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = &self.pieces[y][x] {
                    if piece.get_side() != side {
                        if piece.move_piece(&king_pos, self).is_ok() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn has_legal_moves(&mut self, side: u8) -> bool {

        for y in 0..8 {
            for x in 0..8 {
                let is_own_piece = if let Some(p) = &self.pieces[y][x] {
                    p.get_side() == side
                } else {
                    false
                };

                if is_own_piece {
                    let from = Position::new(x as i8, y as i8);

                    for target_y in 0..8 {
                        for target_x in 0..8 {
                            let to = Position::new(target_x, target_y);

                            if let Some(p) = &self.pieces[y][x] {
                                if !p.move_piece(&to, self).is_ok() {
                                    continue;
                                }
                            }

                            if self.move_piece(from.clone(), to.clone(), side).is_ok() {
                                self._move_piece_internal(to.clone(), from.clone()).unwrap();
                                let captured = self._move_piece_internal(from.clone(), to.clone()).unwrap();
                                let still_checked = self.is_checked(side, None);
                                let _ = self._move_piece_internal(to.clone(), from.clone());
                                self.pieces[target_y as usize][target_x as usize] = captured;

                                if !still_checked {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_within_bounds(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < 8 && position.y >= 0 && position.y < 8
    }
}