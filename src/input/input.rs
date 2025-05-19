use std::io;
use crate::piece::piece::Position;

fn parse_chess_coord(coord: &str) -> Option<Position> {
    if coord.len() != 2 {
        return None;
    }

    let col = coord.chars().nth(0)?;
    let row = coord.chars().nth(1)?;

    let x = match col {
        'a'..='h' => (col as u8 - b'a') as i32,
        'A'..='H' => (col as u8 - b'A') as i32,
        _ => return None,
    };

    let y = match row.to_digit(10) {
        Some(n) if n >= 1 && n <= 8 => (n - 1) as i32,
        _ => return None,
    };

    Some(Position::new(x as i8, y as i8))
}

pub fn input_turn() -> (Position, Position) {
    fn read_position(prompt: &str) -> Position {
        loop {
            println!("{}", prompt);

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let coord = input.trim();
                if let Some(pos) = parse_chess_coord(coord) {
                    return pos;
                }
            }
            println!("Entrée invalide. Format attendu : ex. e2");
        }
    }

    let from = read_position("Entrez la case de départ (ex: e2):");
    let to = read_position("Entrez la case d'arrivée (ex: e4):");

    (from, to)
}