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
            // Second clic : try to move
            if selected != pos {
                let moved = self.board.borrow_mut().move_piece(selected.clone(), pos.clone());
                if moved {
                    // Update the UI with the movement
                    self.ui.update_image_button(Movement::new(selected, pos));
                    self.clear_ui_buttons();
                    // Change the turn
                    self.turn = !self.turn;
                    println!("Check if the king is in check after the move.");
                } else {
                    println!("Move failed, clearing selection.");
                    self.ui.clear_selected_button(&selected);
                    self.clear_ui_buttons();
                }
            } else {
                println!("Clicked on the same position, deselecting.");
                // Deselect the piece if the same position is clicked
                self.ui.clear_selected_button(&pos);
                self.clear_ui_buttons();
            }
        } else {
            // First click : select a piece
            // Check if the position is valid and if the piece belongs to the current player
            if let Some(piece) = self.board.borrow().get_piece(&pos) {
                if piece.get_piece().get_side() != if self.turn { 0 } else { 1 } {
                    println!("It's not your turn to move this piece.");
                    return;
                }
                self.available_moves = piece.all_valid_moves(&self.board.borrow());
                if self.available_moves.is_empty() {
                    println!("No valid moves available for this piece.");
                    self.ui.clear_selected_button(&pos);
                    self.selected_position = None;
                } else {
                    self.ui.set_selected_button(&pos);
                    self.selected_position = Some(pos);
                    // Highlight the available moves in the UI
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
