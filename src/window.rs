use crate::config::*;
use bevy::prelude::*;

use bevy::window::{WindowMode, WindowResolution};

pub struct GameWindowPlugin;

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        let title = "dinosaur".to_string();
        let mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        let name = Some("Bevy engine.app".to_string());
        let resolution = WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);

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
