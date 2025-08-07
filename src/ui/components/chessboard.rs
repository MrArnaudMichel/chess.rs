use std::cell::RefCell;
use std::rc::Rc;
use gtk4::prelude::*;
use gtk4::{Button, Grid, CssProvider, Image, DrawingArea};
use crate::model::board::board::Board;
use crate::model::structs::{position::Position, movement::Movement};

pub struct ChessboardUI {
    grid: Grid,
    pub board: Rc<RefCell<Board>>,
    buttons: Vec<Vec<Button>>
}

impl ChessboardUI {
    pub fn new<F>(board: Rc<RefCell<Board>>, callback: F) -> Self
    where
        F: Fn(Position) + 'static,
    {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let mut buttons = vec![vec![Button::new(); 8]; 8];

        // Wrapping the callback in an Rc to share it across buttons
        let callback = Rc::new(callback);

        for y in 0..8 {
            for x in 0..8 {
                let button = Button::new();
                button.set_hexpand(true);
                button.set_vexpand(true);

                let is_light = (x + y) % 2 == 0;
                let class = if is_light { "light-square" } else { "dark-square" };
                button.add_css_class(class);

                let x_coord = x;
                let y_coord = 7 - y;

                let callback_clone = callback.clone(); // Clone Rc for move closure
                button.connect_clicked(move |_| {
                    let position = Position::new(x_coord as i8, y_coord as i8);
                    callback_clone(position);
                });

                grid.attach(&button, x, y, 1, 1);
                buttons[y as usize][x as usize] = button;
            }
        }

        Self { grid, board, buttons }
    }

    
    pub fn update_image_button(&self, movement: Movement) {
        // Clear the image at the start position
        if let Some(button_start) = self.get_button(movement.get_start().x as u8, movement.get_start().y as u8) {
            button_start.set_child(None::<&gtk4::Widget>);
            button_start.remove_css_class("selected");
        }
        // Set the image at the end position if there is a piece
        if let Some(button_end) = self.get_button(movement.get_finish().x as u8, movement.get_finish().y as u8) {
            if let Some(piece_image_path) = self.get_piece_image(movement.get_finish().x, movement.get_finish().y) {
                let image = Image::from_file(piece_image_path);
                button_end.set_child(Some(&image));
            } else {
                button_end.set_child(None::<&gtk4::Widget>);
            }
        }
    }

    pub fn set_image_button(&self){
        for x in 0..8 {
            for y in [0, 1, 6, 7] {
                if let Some(button) = self.get_button(x, y){
                    if let Some(piece_image_path) = self.get_piece_image(x as i8, y as i8){
                        let image = Image::from_file(piece_image_path);
                        button.set_child(Some(&image));
                    }
                }
            }
        }
    }
    
    pub fn set_selected_button(&self, position: &Position) {
        if let Some(button) = self.get_button(position.x as u8, position.y as u8) {
            button.add_css_class("selected");
        }
    }

    pub fn clear_selected_button(&self, position: &Position) {
        if let Some(button) = self.get_button(position.x as u8, position.y as u8) {
            button.remove_css_class("selected");
        }
    }

    fn create_dot_widget() -> DrawingArea {
        let area = DrawingArea::new();
        area.set_content_width(20);
        area.set_content_height(20);
        area.set_draw_func(move |_, cr, w, h| {
            let radius = f64::min(w as f64, h as f64) / 6.0;
            cr.set_source_rgba(0.1, 0.1, 0.1, 0.7);
            cr.arc((w as f64) / 2.0, (h as f64) / 2.0, radius, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill().unwrap();
        });
        area
    }


    pub fn highlight_valid_move(&self, position: &Position) {
        if let Some(button) = self.get_button(position.x as u8, position.y as u8) {
            button.add_css_class("valid-move");
            // Only add dot if no piece image is present
            if button.child().is_none() {
                let dot = Self::create_dot_widget();
                button.set_child(Some(&dot));
            }
        }
    }

    #[allow(dead_code)]
    pub fn clear_highlighted_moves(&self) {
        for row in &self.buttons {
            for button in row {
                button.remove_css_class("valid-move");
            }
        }
    }

    pub fn clear_highlighted_moves_for_position(&self, position: &Position) {
        if let Some(button) = self.get_button(position.x as u8, position.y as u8) {
            button.remove_css_class("valid-move");
            // Remove dot if present and no piece image should be there
            if let Some(child) = button.child() {
                // If the child is a DrawingArea, remove it
                if child.is::<DrawingArea>() {
                    button.set_child(None::<&gtk4::Widget>);
                }
            }
        }
    }

    // Helper method to determine which piece image to use based on position
    fn get_piece_image(&self, x: i8, y: i8) -> Option<String> {
        let position: Position = Position::new(x, y);
        if let Some(piece) = self.board.borrow().get_piece(&position) {
            Option::from(piece.get_path_image())
        } else { None }
    }

    pub fn get_button(&self, x: u8, y: u8) -> Option<&Button> {
        if x < 8 && y < 8 {
            Some(&self.buttons[7 - y as usize][x as usize])
        } else {
            None
        }
    }


    pub fn widget(&self) -> &Grid {
        &self.grid
    }

    pub fn load_css() {
        let provider = CssProvider::new();

        let css_data = include_str!("../../style.css");
        provider.load_from_data(css_data);

        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Pas de display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

}
