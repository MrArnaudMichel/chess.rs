use super::piece::{Piece, Position, ChessPiece};

pub struct Pawn {
    piece: Piece,
    has_moved: bool,
}

impl Pawn {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            piece: Piece::new(x, y, 0),
            has_moved: false,
        }
    }

    // Vérifie si le mouvement est valide pour un pion
    pub fn is_valid_move(&self, new_x: u8, new_y: u8) -> bool {
        let current_pos = self.get_position();
        
        // Le pion ne peut se déplacer que vers l'avant (y augmente)
        if new_x != current_pos.x {
            return false;
        }

        let forward_distance = new_y as i16 - current_pos.y as i16;
        
        if !self.has_moved {
            // Premier mouvement : peut avancer de 1 ou 2 cases
            forward_distance == 1 || forward_distance == 2
        } else {
            // Après le premier mouvement : peut avancer que d'une case
            forward_distance == 1
        }
    }
}

// Implémentation du trait ChessPiece pour Pawn
impl ChessPiece for Pawn {
    fn get_position(&self) -> &Position {
        self.piece.get_position()
    }

    fn get_position_mut(&mut self) -> &mut Position {
        self.piece.get_position_mut()
    }
}
