mod input;
mod model;

use model::board::board::Board;
use model::piece::{pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use input::input::input_turn;
use crate::model::piece::piece::ChessPiece;

fn main() {
    setup_game();
}

fn setup_game() {
    let mut board = Board::new();

    // Place pawns
    for i in 0..8 {
        board.add_piece(Box::new(Pawn::new(i, 1, 0)));
        board.add_piece(Box::new(Pawn::new(i, 6, 1)));
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
            board.add_piece(constructor(x, y, side));
        }
    }

    let mut turn: u8 = 0;

    println!("État initial de l'échiquier :");
    board.display_all();

    while play_turn(&mut turn, &mut board) {
        board.display_all();
    }
}

fn play_turn(turn: &mut u8, board: &mut Board) -> bool {
    let mut mouv = input_turn();
    let mut result = board.is_occupied(&mouv.0);

    while result != *turn as i8 || !board.move_piece(mouv.0, mouv.1) {
        if result != *turn as i8 {
            println!("Aucune pièce ou ce n’est pas à vous de jouer.");
        } else {
            println!("La case n'est pas atteignable.");
        }
        mouv = input_turn();
        result = board.is_occupied(&mouv.0);
    }
    next_turn(turn);
    true
}

fn next_turn(turn: &mut u8){
    *turn = (*turn + 1) %2
}