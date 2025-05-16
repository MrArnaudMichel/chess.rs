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
        let (from_x, from_y) = (from.0 as usize, from.1 as usize);
        let (to_x, to_y) = (to.0 as usize, to.1 as usize);

        if let Some(mut piece) = self.pieces[from_y][from_x].take() {
            piece.get_position_mut().x = to.0;
            piece.get_position_mut().y = to.1;

            self.pieces[to_y][to_x] = Some(piece);
        } else {
            println!("Aucune pièce trouvée à la position ({}, {})", from.0, from.1);
        }
    }
}