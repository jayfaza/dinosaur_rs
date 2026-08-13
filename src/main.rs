mod app;
mod dino;
mod ground;
mod models;
mod overlay;
mod camera;
mod step;

use crate::app::Game;

fn main() {
    let game = Game::init();
    game.run();
}
