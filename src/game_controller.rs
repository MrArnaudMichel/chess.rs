use crate::model::board::board::Board;
use crate::model::structs::position::Position;
use crate::ui::components::chessboard::ChessboardUI;
use std::cell::RefCell;
use std::rc::Rc;
use crate::model::structs::movement::Movement;

pub struct GameController {
    pub board: Rc<RefCell<Board>>,
    pub ui: ChessboardUI,
    pub selected_position: Option<Position>,
    turn : bool, // true for white's turn, false for black
}

impl GameController {
    pub fn new(board: Rc<RefCell<Board>>, ui: ChessboardUI) -> Self {
        Self {
            board,
            ui,
            selected_position: None,
            turn: true,
        }
    }

    pub fn on_square_clicked(&mut self, pos: Position) {

        if let Some(selected) = self.selected_position.take() {
            println!("Selected position: {}", selected.to_string());
            // Deuxième clic : tentative de déplacement
            if selected != pos {
                let moved = self.board.borrow_mut().move_piece(selected.clone(), pos.clone());
                if moved {
                    println!("Moved: {}", moved);
                    // Met à jour l'UI
                    self.ui.update_image_button(Movement::new(selected, pos));
                    // Change le tour
                    self.turn = !self.turn;
                } else {
                    println!("Move failed, clearing selection.");
                    self.ui.clear_selected_button(&selected);
                    self.selected_position = None;
                }
            } else {
                println!("Clicked on the same position, deselecting.");
                // Si le même bouton est cliqué, on désélectionne
                self.ui.clear_selected_button(&pos);
                self.selected_position = None;
            }
        } else {
            println!("First click on position: {}", pos.to_string());
            // Premier clic : sélection
            // Vérifie qu'il y a bien une pièce à cette position
            if let Some(piece) = self.board.borrow().get_piece(&pos) {
                if piece.get_piece().get_side() != if self.turn { 0 } else { 1 } {
                    println!("It's not your turn to move this piece.");
                    return;
                }
                self.ui.set_selected_button(&pos);
                self.selected_position = Some(pos);
            }
        }
    }
}
