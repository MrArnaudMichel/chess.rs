mod piece;
mod board;

use piece::pawn::Pawn;
use board::board::Board;
use crate::piece::piece::ChessPiece;

fn main() {
    let mut board = Board::new();

    let pawn = Pawn::new(1, 1);
    pawn.display();
    board.add_piece(Box::new(pawn));

    println!("État initial de l'échiquier :");
    board.display_all();

    println!("\nDéplacement du pion de b2 à b3...");
    board.move_piece((1, 1), (1, 2));
    board.move_piece((1, 2), (1, 3));

    // Réafficher l'échiquier après le déplacement
    println!("État de l'échiquier après le déplacement :");
    board.display_all();
}