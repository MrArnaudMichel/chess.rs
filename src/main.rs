mod board;
mod piece;

use crate::piece::piece::{ChessPiece, Position};
use board::board::Board;
use piece::{pawn::Pawn, rook::Rook};

fn test(){
    for i in (0..8).rev() {
        println!("{}", i)
    }
}

fn main() {
    // test();
    setup_game();
}

fn setup_game(){
    let mut board = Board::new();

    for i in 0..8 {
        board.add_piece(Box::new(Pawn::new(i, 1, 0)));
    }
    board.add_piece(Box::new(Rook::new(0, 0, 0)));

    println!("État initial de l'échiquier :");
    board.display_all();

    println!("\nDéplacement du pion de b2 à b3...");
    board.move_piece(Position::new(4, 1), Position::new(4, 3));
    board.display_all();

    println!("\nDéplacement du pion de b3 à b4...");
    board.move_piece(Position::new(2, 2), Position::new(2, 3));
    board.display_all();

    println!("\nDéplacement de la tour de a1 à e1...");
    board.move_piece(Position::new(0, 0), Position::new(4, 0));
    board.display_all();

    println!("\nDéplacement de la tour de a1 à e1...");
    board.move_piece(Position::new(4, 0), Position::new(4, 5));
    board.display_all();
}