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
    available_moves: Vec<Position>, // Liste des mouvements valides pour la pièce sélectionnée
}

impl GameController {
    pub fn new(board: Rc<RefCell<Board>>, ui: ChessboardUI) -> Self {
        Self {
            board,
            ui,
            selected_position: None,
            turn: true,
            available_moves: Vec::new(),
        }
    }

    pub fn on_square_clicked(&mut self, pos: Position) {

        if let Some(selected) = self.selected_position.take() {
            // Deuxième clic : tentative de déplacement
            if selected != pos {
                let moved = self.board.borrow_mut().move_piece(selected.clone(), pos.clone());
                if moved {
                    // Met à jour l'UI
                    self.ui.update_image_button(Movement::new(selected, pos));
                    self.clear_ui_buttons();
                    // Change le tour
                    self.turn = !self.turn;
                } else {
                    println!("Move failed, clearing selection.");
                    self.ui.clear_selected_button(&selected);
                    self.clear_ui_buttons();
                }
            } else {
                println!("Clicked on the same position, deselecting.");
                // Si le même bouton est cliqué, on désélectionne
                self.ui.clear_selected_button(&pos);
                self.clear_ui_buttons();
            }
        } else {
            // Premier clic : sélection
            // Vérifie qu'il y a bien une pièce à cette position
            if let Some(piece) = self.board.borrow().get_piece(&pos) {
                if piece.get_piece().get_side() != if self.turn { 0 } else { 1 } {
                    println!("It's not your turn to move this piece.");
                    return;
                }
                self.ui.set_selected_button(&pos);
                self.selected_position = Some(pos);
                self.available_moves = piece.all_valid_moves(&self.board.borrow());
                if self.available_moves.is_empty() {
                    println!("No valid moves available for this piece.");
                    // self.ui.clear_selected_button(&pos);
                    self.selected_position = None;
                } else {
                    // Affiche les mouvements valides
                    for move_pos in self.available_moves.clone() {
                        self.ui.highlight_valid_move(&move_pos);
                    }
                }
            }
        }
    }

    fn clear_ui_buttons(&mut self) {
        self.selected_position = None;
        for move_pos in self.available_moves.clone() {
            self.ui.clear_highlighted_moves_for_position(&move_pos);
        }
    }
}
