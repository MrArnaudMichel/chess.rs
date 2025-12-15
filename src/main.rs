mod core;
mod cli;
mod gui;

fn main() {
    let mut game = core::game::Game::new();
    game.setup();
}
