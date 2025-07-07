use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
use crate::model::board::board::Board;
use crate::model::piece::{piece::ChessPiece, pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use crate::model::structs::position::Position;
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
    let mut board = Board::new();
    setup_game(&mut board);

    // Load CSS for the chessboard
    ChessboardUI::load_css();

    // Create the chessboard UI
    let chessboard = ChessboardUI::new(board, callback);

    chessboard.set_image_button();

    // Create a window
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Chess Game Rust"));
    window.set_default_size(600, 600);

    // Add the chessboard to the window
    window.set_child(Some(chessboard.widget()));

    // Show the window
    window.present();
}

fn callback(position: Position){
    println!("{position}");
}

fn setup_game(board: &mut Board) {

    // Place pawns
    for i in 0..8 {
        board.add_piece(Box::new(Pawn::new(i, 1, 0)));
        board.add_piece(Box::new(Pawn::new(i, 6, 1)));
    }

    // Piece positions: (x, y, side)
    let major_pieces: Vec<(&dyn Fn(i8, i8, u8) -> Box<dyn ChessPiece>, &[(i8, i8, u8)])> = vec![
        // Rooks
        (&|x, y, side| Box::new(Rook::new(x, y, side)), &[(0, 0, 0), (7, 0, 0), (0, 7, 1), (7, 7, 1)]),
        // Bishops
        (&|x, y, side| Box::new(Bishop::new(x, y, side)), &[(2, 0, 0), (5, 0, 0), (2, 7, 1), (5, 7, 1)]),
        // Knights
        (&|x, y, side| Box::new(Knight::new(x, y, side)), &[(1, 0, 0), (6, 0, 0), (1, 7, 1), (6, 7, 1)]),
        // Queens
        (&|x, y, side| Box::new(Queen::new(x, y, side)), &[(3, 0, 0), (3, 7, 1)]),
        // Kings
        (&|x, y, side| Box::new(King::new(x, y, side)), &[(4, 0, 0), (4, 7, 1)]),
    ];

    for (constructor, positions) in major_pieces.iter() {
        for &(x, y, side) in *positions {
            board.add_piece(constructor(x, y, side));
        }
    }

    let mut turn: u8 = 0;

    println!("État initial de l'échiquier :");
    board.display_all();
}