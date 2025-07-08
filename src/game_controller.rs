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
}

impl GameController {
    pub fn new(board: Rc<RefCell<Board>>, ui: ChessboardUI) -> Self {
        Self {
            board,
            ui,
            selected_position: None,
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
                }
            }
        } else {
            // Premier clic : sélection
            // Vérifie qu'il y a bien une pièce à cette position
            if self.board.borrow().get_piece(&pos).is_some() {
                self.selected_position = Some(pos);
            }
        }
        self.board.borrow().display_all();
    }
}
