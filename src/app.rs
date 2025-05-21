use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
use crate::app;
use crate::ui::components::chessboard::ChessboardUI;

pub fn run(){
    // Create a new GTK application
    let app = Application::new(
        Some("com.example.chess"),
        Default::default(),
    );

    // Connect to the activated signal
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    // Load CSS for the chessboard
    ChessboardUI::load_css();

    // Create the chessboard UI
    let chessboard = ChessboardUI::new();

    // Create a window
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Chess Game"));
    window.set_default_size(600, 600);

    // Add the chessboard to the window
    window.set_child(Some(chessboard.widget()));

    // Show the window
    window.present();
}