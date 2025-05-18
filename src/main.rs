mod board;
mod piece;

use crate::piece::piece::{ChessPiece, Position};
use board::board::Board;
use piece::pawn::Pawn;

fn main() {
    let mut board = Board::new();

    let pawn = Pawn::new(1, 1, 0);
    let pawn2 = Pawn::new(1, 2, 1);
    pawn.display();
    board.add_piece(Box::new(pawn));
    board.add_piece(Box::new(pawn2));

    println!("État initial de l'échiquier :");
    board.display_all();
    
    println!("\nDéplacement du pion de b2 à b3...");
    board.move_piece(Position::new(1, 1), Position::new(1, 2));
    
    println!("\nDéplacement du pion de b3 à b4...");
    board.move_piece(Position::new(1, 2), Position::new(1, 3));

    // Réafficher l'échiquier après le déplacement
    println!("État de l'échiquier après le déplacement :");
    board.display_all();
}
