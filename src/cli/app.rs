// crate/main.rs (or wherever run is)

use crate::core::game::Game;
use crate::cli::{renderer, input};
use crate::core::types::color::invert_color;
use crate::core::types::move_error::MoveError;

pub fn run() {
    let mut game = Game::new();
    game.setup();

    loop {
        renderer::render(&game);

        // Check for Checkmate or Stalemate BEFORE asking for a move
        if !game.board.has_legal_moves(game.turn) {
            if game.board.is_checked(game.turn, None) {
                println!("ECHEC ET MAT ! Le camp {} a perdu.", game.turn);
            } else {
                println!("PAT ! Match nul.");
            }
            break; // End the game
        }

        // Notify if just in simple Check
        if game.board.is_checked(game.turn, None) {
            println!("ATTENTION : Votre roi est en échec !");
        }

        println!("Tour du joueur {} (entrez 'quit' pour quitter)", game.turn);

        // Assumption: input::read_move handles the string parsing. 
        // If the user types "quit", return None or handle it inside input.
        // Here, assuming read_move returns Option<Move>. 
        // If you want explicit quit, check the raw input or add a Quit variant to your Move struct.
        let mv = match input::read_move() {
            Some(mv) => mv,
            None => {
                // If read_move returns None, it might be an error or a signal to quit.
                // Let's assume for now None means "Invalid Format" or "Quit".
                println!("Commande invalide ou arrêt.");
                continue;
            }
        };

        // If you added a flag to your move struct or input for quitting:
        // if mv.is_quit() { break; }

        let moved = game.board.move_piece(
            mv.initial_position().clone(),
            mv.finish_position().clone(),
            game.turn
        );

        match moved {
            Ok(()) => {
                println!("Coup joué");
                invert_color(&mut game.turn);
            },
            Err(MoveError::NoPiece) => println!("Aucune pièce ici"),
            Err(MoveError::WrongTurn) => println!("Ce n'est pas ton tour"),
            Err(MoveError::InvalidMove) => {
                println!("Coup invalide (règles de déplacement ou échec)");
            },
            Err(MoveError::KingInCheck) => {
                println!("Coup invalide : votre roi serait en échec");
            },
            
        }
    }
}