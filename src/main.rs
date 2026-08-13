mod app;
mod dino;
mod ground;
mod models;
mod overlay;
mod camera;
mod step;
mod window;
mod config;

use crate::app::Game;

fn main() {
    let game = Game::init();
    game.run();
}
