use gtk4::prelude::*;
use gtk4::{Button, Grid, CssProvider, Image};
use crate::model::board::board::Board;

pub struct ChessboardUI {
    grid: Grid,
    board: Board
}

impl ChessboardUI {
    pub fn new(board: Board) -> Self {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        // Create the chessboard with 8x8 buttons
        for y in 0..8 {
            for x in 0..8 {
                let button = Button::new();
                button.set_hexpand(true);
                button.set_vexpand(true);

                // Set the CSS class based on the square color
                let is_light = (x + y) % 2 == 0;
                let class = if is_light { "light-square" } else { "dark-square" };
                button.add_css_class(class);

                Self::update_image_button(y, x, &button);

                // Store the coordinates for use in the click handler
                let x_coord = x;
                let y_coord = 7 - y; // Invert y-axis to match chess notation (0,0 is bottom-left)

                // Add click handler to print coordinates
                button.connect_clicked(move |_| {
                    let x_char = (b'A' + x_coord as u8) as char;
                    let y_char = (b'1' + y_coord as u8) as char;
                    println!("Clicked: ({}, {})", x_char, y_char);
                });

                // Add the button to the grid
                grid.attach(&button, x, y, 1, 1);
            }
        }

        Self { grid, board }
    }

    fn update_image_button(y: i32, x: i32, button: &Button) {
        // Add chess piece image if it's in the initial position
        let y_coord = 7 - y; // Invert y-axis to match chess notation (0,0 is bottom-left)
        let x_coord = x;

        // Check if there should be a piece at this position
        if let Some(piece_image_path) = Self::get_piece_image(x_coord, y_coord) {
            let image = Image::from_file(piece_image_path);
            button.set_child(Some(&image));
        }
    }

    // Helper method to determine which piece image to use based on position
    fn get_piece_image(x: i32, y: i32) -> Option<String> {
        println!("{}{}", x, y);
        match (x, y) {
            // Pawns
            (0..=7, 1) => Some("assets/images/wpawn.png".to_string()),
            (0..=7, 6) => Some("assets/images/bpawn.png".to_string()),

            // Rooks
            (0, 0) | (7, 0) => Some("assets/images/wrook.png".to_string()),
            (0, 7) | (7, 7) => Some("assets/images/brook.png".to_string()),

            // Knights
            (1, 0) | (6, 0) => Some("assets/images/wknight.png".to_string()),
            (1, 7) | (6, 7) => Some("assets/images/bknight.png".to_string()),

            // Bishops
            (2, 0) | (5, 0) => Some("assets/images/wbishop.png".to_string()),
            (2, 7) | (5, 7) => Some("assets/images/bbishop.png".to_string()),

            // Queens
            (3, 0) => Some("assets/images/wqueen.png".to_string()),
            (3, 7) => Some("assets/images/bqueen.png".to_string()),

            // Kings
            (4, 0) => Some("assets/images/wking.png".to_string()),
            (4, 7) => Some("assets/images/bking.png".to_string()),

            // No piece at this position
            _ => None,
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
