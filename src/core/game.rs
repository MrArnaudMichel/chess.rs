//! Core game utilities: setup functions to initialize the board and manage turns.
use crate::core::board::board::Board;
use crate::core::piece::{chess_piece::ChessPiece, pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use crate::core::types::color::{BLACK, WHITE};
use crate::core::types::position::{*};

pub struct Game {
    pub board: Board,
    pub turn: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: WHITE,
        }
    }

    pub fn setup(&mut self) {
        for i in 0..8 {
            let white_pawn = Box::new(Pawn::new(Position::new(i, 1), WHITE));
            let black_pawn = Box::new(Pawn::new(Position::new(i, 6), BLACK));
            self.board.place_piece(white_pawn);
            self.board.place_piece(black_pawn);
        }

        let major_pieces: Vec<(&dyn Fn(Position, u8) -> Box<dyn ChessPiece>, &[(Position, u8)])> = vec![
            (&|position, side| Box::new(Rook::new(position, side)), &[(A1, WHITE), (A8, WHITE), (H1, BLACK), (H8, BLACK)]),
            (&|position, side| Box::new(Bishop::new(position, side)), &[(C1, WHITE), (C8, WHITE), (F1, BLACK), (F8, BLACK)]),
            (&|position, side| Box::new(Knight::new(position, side)), &[(B1, WHITE), (B8, WHITE), (G1, BLACK), (G8, BLACK)]),
            (&|position, side| Box::new(Queen::new(position, side)), &[(D1, WHITE), (D8, BLACK)]),
            (&|position, side| Box::new(King::new(position, side)), &[(E1, WHITE), (E8, BLACK)]),
        ];

        for (constructor, positions) in major_pieces {
            for &(position, side) in positions {
                let piece = constructor(position, side);
                self.board.place_piece(piece);
            }
        }
    }
}