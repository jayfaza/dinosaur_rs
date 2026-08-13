use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::linear_rgb(1.0, 1.0, 1.0)));
        app.add_systems(Startup, setup_camera);
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
