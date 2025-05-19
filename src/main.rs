mod input;
mod model;

use model::board::board::Board;
use model::piece::{bishop::Bishop, pawn::Pawn, rook::Rook, queen::Queen};
use input::input::input_turn;

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

    board.add_piece(Box::new(Bishop::new(2, 0, 0)));
    board.add_piece(Box::new(Bishop::new(5, 0, 0)));
    board.add_piece(Box::new(Bishop::new(2, 7, 1)));
    board.add_piece(Box::new(Bishop::new(5, 7, 1)));

    board.add_piece(Box::new(Queen::new(3, 0, 1)));
    board.add_piece(Box::new(Queen::new(3, 7, 1)));

    let mut turn: u8 = 0;

    println!("État initial de l'échiquier :");
    board.display_all();

    while play_turn(&mut turn, &mut board) {
        board.display_all();
    }
}

fn play_turn(turn: &mut u8, board: &mut Board) -> bool {
    println!("{turn}");
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