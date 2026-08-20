use crate::config::*;
use crate::window::WindowModeEnum::{Fullscreen, Windowed};
use bevy::prelude::*;

use bevy::window::{WindowMode, WindowResolution};

pub struct GameWindowPlugin;

#[derive(Resource)]
pub struct WindowInfo {
    pub width: u32,
    pub height: u32,
    pub mode: WindowModeEnum,
}

pub enum WindowModeEnum {
    Fullscreen,
    Windowed,
}

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        let (title, mode, name, resolution) = calculate_window_properties();
        let primary_window = Some(Window {
            title: title,
            mode: mode,
            name: name,
            resolution: resolution,
            ..default()
        });

        let window_plugin = WindowPlugin {
            primary_window: primary_window,
            ..default()
        };

        app.add_plugins(DefaultPlugins.set(window_plugin));
    }
}

fn calculate_window_properties() -> (String, WindowMode, Option<String>, WindowResolution) {
    let title = WINDOW_TITLE.to_string();
    let mode = match WINDOW_MODE {
        Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        Fullscreen => WindowMode::Windowed,
    };
    let name = Some(WINDOW_NAME.to_string());
    let resolution = WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);
    (title, mode, name, resolution)
}
