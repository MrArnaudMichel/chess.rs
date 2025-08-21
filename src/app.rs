//! Application entry point and UI setup for the Chess game.
//!
//! This module initializes the GTK application, sets up the chessboard UI,
//! and manages the game controller and board state.

use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::*;
use crate::model::board::board::Board;
use crate::model::piece::{chess_piece::ChessPiece, pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use crate::model::structs::position::Position;
use crate::ui::components::chessboard::ChessboardUI;
use crate::ui::controllers::{home::HomeController, game_page::GamePageController, settings::SettingsController};
use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::controllers::game_controller::GameController;
use adw::prelude::*;

/// Runs the GTK application.
///
/// Initializes the application, connects the activation signal,
/// and starts the main event loop.
pub fn run(){
    // Initialize Adwaita
    let _ = adw::init();
    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);

    // Create a new GTK application
    let app = Application::new(
        Some("fr.arnaudmichel.chess-rs"),
        Default::default(),
    );

    // Connect to the activated signal
    app.connect_activate(build_ui);

    app.run();
}

/// Builds the main UI when the application is activated.
///
/// Creates the main window, initializes the board and controller,
/// and sets up the chessboard UI.
///
/// # Arguments
///
/// * `app` - The GTK application instance.
fn build_ui(app: &Application) {
    use gtk4::{HeaderBar, Stack, StackSwitcher};
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
    // Build UI pages with a Stack
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Chess Game Rust"));
    // Enforce 16:9 default size
    window.set_default_size(1280, 720);
    // Allow resizing so the board can take all remaining space
    window.set_resizable(true);

    // Header with switcher
    let header = HeaderBar::new();
    let switcher = StackSwitcher::new();
    let stack = Stack::new();
    switcher.set_stack(Some(&stack));
    header.set_title_widget(Some(&switcher));
    window.set_titlebar(Some(&header));

    // HOME PAGE (controller)
    let home = HomeController::new();
    stack.add_titled(home.root(), Some("home"), "Accueil");

    // GAME PAGE (controller)
    let game_page = GamePageController::new(controller_ref.clone());
    stack.add_titled(game_page.root(), Some("game"), "Jeu");

    // SETTINGS PAGE (controller)
    let settings = SettingsController::new();
    stack.add_titled(settings.root(), Some("settings"), "Paramètres");

    // Hook up home buttons
    {
        let stack_clone = stack.clone();
        home.btn_play.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("game");
        });
    }
    {
        let stack_clone = stack.clone();
        home.btn_settings.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("settings");
        });
    }

    // Set initial page to home
    stack.set_visible_child_name("home");

    window.set_child(Some(&stack));
    window.present();
}

/// Sets up the initial chess game state on the board.
///
/// Places all pieces in their starting positions.
///
/// # Arguments
///
/// * `board` - A mutable reference to the `Board` to initialize.
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


    println!("État initial de l'échiquier :");
    board.display_all();
}