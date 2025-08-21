use crate::model::board::board::Board;
use crate::model::structs::position::Position;
use crate::ui::components::chessboard::ChessboardUI;
use std::cell::RefCell;
use std::rc::Rc;
use crate::model::structs::movement::Movement;
use crate::model::piece::chess_piece::ChessPiece;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::Button;

struct MoveRecord {
    from: Position,
    to: Position,
    captured: Option<Box<dyn ChessPiece>>,
    moved_was: bool,
}

pub struct GameController {
    pub board: Rc<RefCell<Board>>,
    pub ui: ChessboardUI,
    pub selected_position: Option<Position>,
    turn : bool, // true for white's turn, false for black
    available_moves: Vec<Position>, // All valid moves for the selected piece
    move_count: usize,
    history: Vec<MoveRecord>,
    cursor: usize, // index of next move to play (0..=history.len())
    white_time: i32,
    black_time: i32,
    timer_id: Option<glib::SourceId>,
    back_button: Option<Button>,
    forward_button: Option<Button>,
}

impl GameController {
    pub fn new(board: Rc<RefCell<Board>>, ui: ChessboardUI) -> Self {
        Self {
            board,
            ui,
            selected_position: None,
            turn: true,
            available_moves: Vec::new(),
            move_count: 0,
            history: Vec::new(),
            cursor: 0,
            white_time: 300,
            black_time: 300,
            timer_id: None,
            back_button: None,
            forward_button: None,
        }
    }

    pub fn on_square_clicked(&mut self, pos: Position) {

        if let Some(selected) = self.selected_position.take() {
            // Second click: try to move
            if selected != pos {
                // Validate move manually (so we can capture info before applying)
                let valid = {
                    let b = self.board.borrow();
                    if let Some(p) = b.get_piece(&selected) {
                        p.is_valid_move(&pos, &b)
                    } else { false }
                };
                if valid {
                    // If we had undone some moves, drop the redo branch
                    if self.cursor < self.history.len() {
                        self.history.truncate(self.cursor);
                    }
                    // capture moving piece state and any captured target
                    let moved_was = {
                        let b = self.board.borrow();
                        b.get_piece(&selected).map(|p| p.get_piece().has_moved()).unwrap_or(false)
                    };
                    let captured = self.board.borrow_mut().apply_move_unchecked(selected.clone(), pos.clone());

                    // Update the UI with the movement
                    let movement = Movement::new(selected.clone(), pos.clone());
                    self.ui.update_image_button(movement);
                    // Record history
                    self.history.push(MoveRecord { from: selected.clone(), to: pos.clone(), captured, moved_was });
                    self.cursor += 1;
                    // Add to move list and update info
                    self.ui.append_move_entry(&selected, &pos, self.move_count);
                    self.move_count += 1;
                    self.clear_ui_buttons();
                    // Change the turn
                    self.turn = !self.turn;
                    self.ui.set_turn_label(self.turn);
                    // Simple info example: check status
                    let checked = self.board.borrow_mut().is_checked(!self.turn as u8, None);
                    if checked { self.ui.set_info_text("Échec !"); } else { self.ui.set_info_text(""); }
                    // restart timer for new turn
                    self.restart_timer();
                    self.update_nav_buttons();
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
            // First click: select a piece
            // Check if the position is valid and if the piece belongs to the current player
            let board = self.board.borrow();
            if let Some(piece) = board.get_piece(&pos) {
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

    pub fn update_nav_buttons(&self) {
        let back_enabled = self.cursor > 0;
        let forward_enabled = self.cursor < self.history.len();
        if let Some(btn) = &self.back_button { btn.set_sensitive(back_enabled); }
        if let Some(btn) = &self.forward_button { btn.set_sensitive(forward_enabled); }
    }

    pub fn set_nav_buttons(&mut self, back: Button, forward: Button) {
        self.back_button = Some(back);
        self.forward_button = Some(forward);
        self.update_nav_buttons();
    }

    pub fn step_back(&mut self) {
        if self.cursor == 0 { return; }
        let idx = self.cursor - 1;
        let rec = &mut self.history[idx];
        // Undo on board (take ownership of captured)
        let captured = rec.captured.take();
        let from = rec.from.clone();
        let to = rec.to.clone();
        let moved_was = rec.moved_was;
        self.board.borrow_mut().undo_move_unchecked(from.clone(), to.clone(), captured, moved_was);
        // Update UI squares
        self.ui.refresh_squares(&[from.clone(), to.clone()]);
        // Move cursor back and toggle turn
        self.cursor -= 1;
        self.turn = !self.turn;
        self.ui.set_turn_label(self.turn);
        let checked = self.board.borrow_mut().is_checked(!self.turn as u8, None);
        if checked { self.ui.set_info_text("Échec !"); } else { self.ui.set_info_text(""); }
        self.restart_timer();
        self.update_nav_buttons();
    }

    pub fn step_forward(&mut self) {
        if self.cursor >= self.history.len() { return; }
        let rec = &self.history[self.cursor];
        let from = rec.from.clone();
        let to = rec.to.clone();
        // Apply the stored move again
        let _captured_now = self.board.borrow_mut().apply_move_unchecked(from.clone(), to.clone());
        // Update UI squares based on new board state
        self.ui.refresh_squares(&[from.clone(), to.clone()]);
        self.cursor += 1;
        // Toggle turn and info
        self.turn = !self.turn;
        self.ui.set_turn_label(self.turn);
        let checked = self.board.borrow_mut().is_checked(!self.turn as u8, None);
        if checked { self.ui.set_info_text("Échec !"); } else { self.ui.set_info_text(""); }
        self.restart_timer();
        self.update_nav_buttons();
    }

    fn clear_ui_buttons(&mut self) {
        self.selected_position = None;
        for move_pos in self.available_moves.clone() {
            self.ui.clear_highlighted_moves_for_position(&move_pos);
        }
    }

    fn restart_timer(&mut self) {
        // Timer not implemented yet: placeholder to satisfy calls
    }
}
