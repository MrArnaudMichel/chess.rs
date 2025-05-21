use crate::input::input::input_turn;
use crate::model::board::board::Board;

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