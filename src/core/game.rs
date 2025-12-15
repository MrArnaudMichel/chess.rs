use crate::core::board::board::Board;
use crate::core::piece::{chess_piece::ChessPiece, pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use crate::core::types::color::WHITE;

pub struct Game {
    pub board: Board,
    pub turn: i8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: WHITE,
        }
    }
    
    pub fn setup(&mut self) {
        // Place pawns
        for i in 0..8 {
            self.board.add_piece(Box::new(Pawn::new(i, 1, 0)));
            self.board.add_piece(Box::new(Pawn::new(i, 6, 1)));
        }

        // Piece positions: (x, y, side)
        let major_pieces: Vec<(&dyn Fn(i8, i8, u8) -> Box<dyn ChessPiece>, &[(i8, i8, u8)])> = vec![
            // Rooks
            (&|x, y, side| Box::new(Rook::new(x, y, side)), &[(0, 0, 0), (7, 0, 0), (0, 7, 1), (7, 7, 1)]),
            // Bishops
            (&|x, y, side| Box::new(Bishop::new(x, y, side)), &[(2, 0, 0), (5, 0, 0), (2, 7, 1), (5, 7, 1)]),
            // Knights
            (&|x, y, side| Box::new(Knight::new(x, y, side)), &[(1, 0, 0), (6, 0, 0), (1, 7, 1), (6, 7, 1)]),
            // Queens
            (&|x, y, side| Box::new(Queen::new(x, y, side)), &[(3, 0, 0), (3, 7, 1)]),
            // Kings
            (&|x, y, side| Box::new(King::new(x, y, side)), &[(4, 0, 0), (4, 7, 1)]),
        ];

        for (constructor, positions) in major_pieces.iter() {
            for &(x, y, side) in *positions {
                self.board.add_piece(constructor(x, y, side));
            }
        }


        println!("État initial de l'échiquier :");
        self.board.display_all();
    }
}