use bevy::prelude::*;

use crate::dino::Dino;

#[derive(Component)]
pub struct Position;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay)
            .add_systems(Update, update_overlay);
    }
}

pub fn update_overlay(
    mut text_query: Query<&mut Text2d, With<Position>>,
    dino_pos: Query<&Transform, With<Dino>>,
) {
    for mut text in text_query.iter_mut() {
        for pos in dino_pos {
            let x = pos.translation.x;
            let y = pos.translation.y;
            text.clear();
            let info = format!("x: {}, y: {}", x, y);
            text.push_str(&info);
        }
    }
}

pub fn setup_overlay(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Position,
        Text2d::new("dino x: 0"),
        TextColor(Color::BLACK),
        TextFont::from(font.clone()).with_font_size(60.),
        Transform::from_xyz(-485., 335., 0.),
    ));
}
