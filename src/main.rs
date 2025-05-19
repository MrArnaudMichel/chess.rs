mod board;
mod piece;
mod structs;

use crate::piece::piece::{Position};
use board::board::Board;
use piece::{pawn::Pawn, rook::Rook};

fn main() {
    setup_game();
}

fn setup_game(){
    let mut board = Board::new();

    for i in 0..8 {
        board.add_piece(Box::new(Pawn::new(i, 1, 0)));
        board.add_piece(Box::new(Pawn::new(i, 6, 1)));
    }
    board.add_piece(Box::new(Rook::new(0, 0, 0)));
    board.add_piece(Box::new(Rook::new(7, 0, 0)));
    board.add_piece(Box::new(Rook::new(0, 7, 1)));
    board.add_piece(Box::new(Rook::new(7, 7, 1)));

    println!("État initial de l'échiquier :");
    board.display_all();

    println!("\nDéplacement de la tour de a1 à e1...");
    board.move_piece(Position::new(4, 0), Position::new(4, 5));
    board.display_all();
}