use gtk4::prelude::*;
use gtk4::{Button, Grid, CssProvider};

pub struct ChessboardUI {
    grid: Grid,
}

impl ChessboardUI {
    pub fn new() -> Self {
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

                let context = button.style_context();

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


        Self { grid }
    }

    pub fn widget(&self) -> &Grid {
        &self.grid
    }

    pub fn load_css() {
        let provider = CssProvider::new();

        let css_data = include_str!("../../style.css");
        provider.load_from_data(css_data); // NE PAS utiliser .as_bytes()

        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Pas de display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

}
