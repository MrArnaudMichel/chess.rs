use std::io::{self, Write};
use crate::cli::input::UserInput::{Invalid, Quit};
use crate::core::types::position::Position;
use crate::core::types::movement::Movement;

pub enum UserInput {
    Move(Movement),
    Quit,
    Invalid,
}

pub fn read_move() -> UserInput {
    print!("Entrez votre coup (ex: e2 e4) ou 'quit' pour arrêter : ");
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Invalid;
    }

    let input = input.trim();

    if input.eq_ignore_ascii_case("quit") {
        return Quit;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        println!("Format invalide.");
        return Invalid;
    }

    let from = match Position::from_algebraic(parts[0]) {
        Some(p) => p,
        None => return Invalid,
    };

    let to = match Position::from_algebraic(parts[1]) {
        Some(p) => p,
        None => return Invalid,
    };

    UserInput::Move(Movement::new(from, to))
}