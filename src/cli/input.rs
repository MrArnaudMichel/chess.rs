use std::io::{self, Write};
use crate::core::types::movement::Movement;
use crate::core::types::position::Position;

pub fn read_move() -> Option<Movement> {
    print!("Entrez votre coup (ex: e2 e4) : ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Format invalide.");
        return None;
    }

    let from = Position::from_algebraic(parts[0])?;
    let to = Position::from_algebraic(parts[1])?;

    Some(Movement::new(from, to))
}
