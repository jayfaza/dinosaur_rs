use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::dino::DinoPlugin;
use crate::ground::GroundPlugin;
use crate::models::ModelsPlugin;
use crate::overlay::OverlayPlugin;
use crate::step::StepPlugin;
use crate::window::GameWindowPlugin;

pub struct Game {
    app: App,
}

impl Game {
    pub fn init() -> Self {
        let mut game = Game { app: App::new() };

        game.app
            .insert_resource(ClearColor(Color::linear_rgb(1.0, 1.0, 1.0)))
            .add_plugins((
                GameWindowPlugin,
                DinoPlugin,
                ModelsPlugin,
                OverlayPlugin,
                GroundPlugin,
                CameraPlugin,
                StepPlugin,
            ));

        game
    }

    pub fn run(mut self) -> AppExit {
        self.app.run()
    }
}
