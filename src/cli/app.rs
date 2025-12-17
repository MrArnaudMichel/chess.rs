use crate::core::game::Game;
use crate::cli::{renderer, input};
use crate::core::types::color::{color_to_string, invert_color};
use crate::core::types::r#move::{MoveError, MoveOutcome};

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
            break;
        }

        if game.board.is_checked(game.turn, None) {
            println!("ATTENTION : Votre roi est en échec !");
        }

        println!("Tour du joueur {} (entrez 'quit' pour quitter)", color_to_string(game.turn));

        let mv = match input::read_move() {
            input::UserInput::Move(mv) => mv,
            input::UserInput::Quit => {
                println!("Jeu terminé par l'utilisateur.");
                break;
            },
            input::UserInput::Invalid => {
                println!("Entrée invalide, veuillez réessayer.");
                continue;
            },
        };

        // If you added a flag to your move struct or input for quitting:
        // if mv.is_quit() { break; }

        let moved = game.board.move_piece(
            mv.initial_position().clone(),
            mv.finish_position().clone(),
            game.turn
        );

        match moved {
            Ok(MoveOutcome::Valid) |
            Ok(MoveOutcome::Capture) |
            Ok(MoveOutcome::Castling) |
            Ok(MoveOutcome::Promotion) |
            Ok(MoveOutcome::EnPassant {captured: _}) => {
                println!("Coup joué");
                invert_color(&mut game.turn);
            },
            Err(MoveError::NoPiece) => println!("Aucune pièce ici"),
            Err(MoveError::BlockedPath) => println!("Le chemin est bloqué"),
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