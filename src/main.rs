use bevy::prelude::*;

mod dino;
mod ground;
mod models;
mod overlay;
mod setups;
mod step;

use dino::update_dino;
use dino::{Dino, DinoInfo};
use ground::*;
use models::Models;
use setups::*;
use step::*;
use overlay::update_overlay;

fn main() {
    App::new()
        .insert_resource(StepTimer(Timer::from_seconds(0.13, TimerMode::Repeating)))
        .insert_resource(ClearColor(Color::linear_rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (setup_dino, setup_camera, setup_models, setup_ground, setup_overlay),
        )
        .add_systems(Update, (update_step, update_ground, update_dino, update_overlay))
        .run();
}
