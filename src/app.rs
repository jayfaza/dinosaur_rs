use crate::step::*;
use bevy::prelude::*;

use crate::dino::DinoPlugin;
use crate::ground::GroundPlugin;
use crate::models::ModelsPlugin;
use crate::overlay::OverlayPlugin;
use crate::camera::CameraPlugin;
use crate::step::StepPlugin;

pub struct Game {
    app: App,
}

impl Game {
    pub fn init() -> Self {
        let mut game = Game { app: App::new() };

        game.app
            .insert_resource(StepTimer(Timer::from_seconds(0.13, TimerMode::Repeating)))
            .insert_resource(ClearColor(Color::linear_rgb(1.0, 1.0, 1.0)))
            .add_plugins(DefaultPlugins)
            .add_plugins((
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
