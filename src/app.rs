use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
use crate::model::board::board::Board;
use crate::model::piece::{chess_piece::ChessPiece, pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use crate::model::structs::position::Position;
use crate::ui::components::chessboard::ChessboardUI;
use std::cell::RefCell;
use std::rc::Rc;
use crate::game_controller::GameController;

/// Main function to run the GTK application
/// This function initializes the GTK application, sets up the UI, and starts the event loop.
/// # Arguments
/// None
/// # Returns
/// None
pub fn run(){
    // Create a new GTK application
    let app = Application::new(
        Some("Chess Rust"),
        Default::default(),
    );

    // Connect to the activated signal
    app.connect_activate(build_ui);

    app.run();
}

/// Function to build the UI of the application
/// This function sets up the chessboard UI, initializes the game controller, and creates the main application window.
/// # Arguments
/// * `app`: A reference to the GTK application.
/// # Returns
/// None
fn build_ui(app: &Application) {
    // Crée un Board partagé
    let board = Rc::new(RefCell::new(Board::new()));
    setup_game(&mut board.borrow_mut());

    // Charge le CSS
    ChessboardUI::load_css();

    // On va créer le GameController dans une RefCell pour pouvoir le muter dans le callback
    let controller_ref = Rc::new(RefCell::new(None::<GameController>));
    let board_clone = board.clone();
    let controller_ref_clone = controller_ref.clone();

    // Crée le ChessboardUI avec un callback qui appelle le contrôleur
    let chessboard = ChessboardUI::new(board_clone, move |pos: Position| {
        if let Some(ref mut controller) = *controller_ref_clone.borrow_mut() {
            controller.on_square_clicked(pos);
        }
    });

    chessboard.set_image_button();

    // Crée le contrôleur et le stocke dans le Rc<RefCell<Option<...>>>
    *controller_ref.borrow_mut() = Some(GameController::new(board.clone(), chessboard));
    let controller = controller_ref.borrow();
    let chessboard_ref = &controller.as_ref().unwrap().ui;

    // Crée la fenêtre
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Chess Game Rust"));
    window.set_default_size(600, 600);

    // Ajoute l'échiquier à la fenêtre
    window.set_child(Some(chessboard_ref.widget()));

    // Affiche la fenêtre
    window.present();
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

    // let mut turn: u8 = 0;

    println!("État initial de l'échiquier :");
    board.display_all();
}