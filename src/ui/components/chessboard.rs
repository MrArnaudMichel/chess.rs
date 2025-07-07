use std::rc::Rc;
use glib::property::PropertyGet;
use gtk4::prelude::*;
use gtk4::{Button, Grid, CssProvider, Image};
use crate::model::board::board::Board;
use crate::model::structs::{position::Position, movement::Movement};

pub struct ChessboardUI {
    grid: Grid,
    board: Board,
    buttons: Vec<Vec<Button>>
}

impl ChessboardUI {
    pub fn new<F>(board: Board, callback: F) -> Self
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

    pub fn update_image_button(movement: Movement) {

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

    // Helper method to determine which piece image to use based on position
    fn get_piece_image(&self, x: i8, y: i8) -> Option<String> {
        let position: Position = Position::new(x, y);
        if let Some(piece) = self.board.get_piece(&position) {
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
