use bevy::prelude::*;

use crate::dino::DinoPos;
use crate::window::WindowInfo;
use crate::window::WindowModeEnum;

pub const RESOLUTION_WIDTH: u32 = 1920;
pub const RESOLUTION_HEIGHT: u32 = 1080;
pub const WINDOW_MODE: WindowModeEnum = WindowModeEnum::Windowed;
pub const WINDOW_TITLE: &str = "bevy.appx";
pub const WINDOW_NAME: &str = "dinosaurrr";

pub const TIME_BETWEEN_DINO_STEPS: f32 = 0.13;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let width = RESOLUTION_WIDTH as f32;

        let max_x = width / 2. - 200.;
        let min_x = -max_x;
        app.insert_resource(DinoPos {
            x: 0,
            y: 0,
            max_x: max_x,
            min_x: min_x,
        });
        app.insert_resource(WindowInfo {
            width: RESOLUTION_WIDTH,
            height: RESOLUTION_HEIGHT,
            mode: WINDOW_MODE,
        });
    }
}
