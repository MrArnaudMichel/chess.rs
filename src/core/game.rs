//! Core game utilities: setup functions to initialize the board and manage turns.
use crate::core::board::board::Board;
use crate::core::piece::{chess_piece::ChessPiece, pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use crate::core::types::color::{BLACK, WHITE};

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
            self.board.add_piece(Box::new(Pawn::new(i, 1, WHITE)));
            self.board.add_piece(Box::new(Pawn::new(i, 6, BLACK)));
        }

        let major_pieces: Vec<(&dyn Fn(i8, i8, u8) -> Box<dyn ChessPiece>, &[(i8, i8, u8)])> = vec![
            (&|x, y, side| Box::new(Rook::new(x, y, side)), &[(0, 0, WHITE), (7, 0, WHITE), (0, 7, BLACK), (7, 7, BLACK)]),
            (&|x, y, side| Box::new(Bishop::new(x, y, side)), &[(2, 0, WHITE), (5, 0, WHITE), (2, 7, BLACK), (5, 7, BLACK)]),
            (&|x, y, side| Box::new(Knight::new(x, y, side)), &[(1, 0, WHITE), (6, 0, WHITE), (1, 7, BLACK), (6, 7, BLACK)]),
            (&|x, y, side| Box::new(Queen::new(x, y, side)), &[(3, 0, WHITE), (3, 7, BLACK)]),
            (&|x, y, side| Box::new(King::new(x, y, side)), &[(4, 0, WHITE), (4, 7, BLACK)]),
        ];

        for (constructor, positions) in major_pieces.iter() {
            for &(x, y, side) in *positions {
                self.board.add_piece(constructor(x, y, side));
            }
        }
    }
}