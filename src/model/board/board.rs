use crate::model::piece::piece::{ChessPiece, Position};
pub struct Board {
    pieces: [[Option<Box<dyn ChessPiece>>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }

    pub fn add_piece(&mut self, piece: Box<dyn ChessPiece>) {
        let pos = piece.get_position().clone();

        if pos.x < 8 && pos.y < 8 {
            self.pieces[pos.y as usize][pos.x as usize] = Some(piece);
        } else {
            println!("Position invalide : la pièce doit être dans un tableau 8x8");
        }
    }

    pub fn get_piece(&self, position: &Position) -> Option<&Box<dyn ChessPiece>> {
        if position.x < 8 && position.y < 8 {
            self.pieces[position.y as usize][position.x as usize].as_ref()
        } else {
            None
        }
    }

    pub fn is_occupied(&self, position: &Position) -> i8 {
        if position.x < 8 && position.y < 8 {
            if let Some(piece) = self.pieces[position.y as usize][position.x as usize].as_ref() {
                piece.get_side() as i8
            } else {
                -1
            }
        } else {
            -1
        }
    }


    pub fn display_all(&self) {
        for y in (0..8).rev() {
            print!("{} ", y + 1);
            for x in 0..8 {
                match &self.pieces[y][x] {
                    Some(piece) => print!("{} ", piece.piece_to_hex()),
                    None => print!("-- "),
                }
            }
            println!();
        }
        println!("  A  B  C  D  E  F  G  H");
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> bool{
        println!("Deplacement de {} en {}", from.to_string(), to.to_string());
        if let Some(piece) = self.get_piece(&from) {
            if piece.is_valid_move(&to, self) {
                self._move_piece(from, to);
                true
            } else { 
                println!("Mouvement impossible");
                false
            }
        } else {
            println!("Aucune pièce trouvée a ces coordonnés");
            false
        }
    }


    fn _move_piece(&mut self, from: Position, to: Position) {
        let (from_x, from_y) = (from.x as usize, from.y as usize);
        let (to_x, to_y) = (to.x as usize, to.y as usize);

        if let Some(mut piece) = self.pieces[from_y][from_x].take() {
            piece.shift(to.x, to.y);
            piece.display();
            self.pieces[to_y][to_x] = Some(piece);
        } else {
            println!("Aucune pièce trouvée à la position ({}, {})", from.x, from.y);
        }
    }
}