use crate::piece::piece::ChessPiece;
pub struct Board {
    pieces: [[Option<Box<dyn ChessPiece>>; 8]; 8]
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

    pub fn get_piece(&self, position: (u8, u8)) -> Option<&Box<dyn ChessPiece>> {
        let (x, y) = position;
        if x < 8 && y < 8 {
            self.pieces[y as usize][x as usize].as_ref()
        } else {
            None
        }
    }

    pub fn get_piece_mut(&mut self, position: (u8, u8)) -> Option<&mut Box<dyn ChessPiece>> {
        let (x, y) = position;
        if x < 8 && y < 8 {
            self.pieces[y as usize][x as usize].as_mut()
        } else {
            None
        }
    }


    pub fn display_all(&self) {
        for y in (0..8).rev() {
            print!("{} ", y + 1);
            for x in 0..8 {
                match &self.pieces[y][x] {
                    Some(_) => print!("P "),
                    None => print!("- "),
                }
            }
            println!();
        }
        println!("  a b c d e f g h");
    }
    
    pub fn move_piece(&mut self, from: (u8, u8), to: (u8, u8)) {
        if let Some(piece) = self.get_piece_mut((from.0, from.1)) {
            piece.display();
        }
    }
}