use std::cell::RefCell;
use std::rc::Rc;
use crate::model::board::board::Board;
use crate::model::structs::position::Position;
use crate::ui::components::chessboard::ChessboardUI;

pub struct GameController {
    pub board: RefCell<Board>,
    pub ui: RefCell<ChessboardUI>,
    pub selected_position: RefCell<Option<Position>>,
}

impl GameController {
    pub fn new(board: Board, ui: ChessboardUI) -> Rc<Self> {
        Rc::new(Self {
            board: RefCell::new(board),
            ui: RefCell::new(ui),
            selected_position: RefCell::new(None),
        })
    }

    pub fn on_square_clicked(self: Rc<Self>, pos: Position) {
        let mut selected = self.selected_position.borrow_mut();
        if let Some(prev_pos) = selected.take() {
            let mut board = self.board.borrow_mut();
            if board.move_piece(prev_pos.clone(), pos.clone()) {
                self.ui.borrow().set_image_button(); // refresh board UI
            } else {
                println!("Mouvement invalide");
            }
        } else {
            *selected = Some(pos);
        }
    }
}
