mod input;
mod model;
mod ui;
mod app;
mod controller;

use model::board::board::Board;
use model::piece::{pawn::Pawn, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen, king::King};
use input::input::input_turn;
use crate::model::piece::piece::ChessPiece;

fn main() {
    app::run();
}
