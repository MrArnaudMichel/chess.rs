use crate::core::game::Game;

pub fn render(game: &Game) {
    println!();
    game.board.display_all();
    println!();
}
