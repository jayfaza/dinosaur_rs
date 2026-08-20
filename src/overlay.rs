use bevy::prelude::*;

use crate::dino::DinoPos;
use crate::window::WindowInfo;

#[derive(Component)]
pub struct Position;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay)
            .add_systems(Update, update_overlay);
    }
}

pub fn update_overlay(mut text_query: Query<&mut Text2d, With<Position>>, dino_pos: Res<DinoPos>) {
    for mut text in text_query.iter_mut() {
        let x = dino_pos.x;
        let y = dino_pos.y;
        text.clear();
        let info = format!("x: {}, y: {}", x, y);
        text.push_str(&info);
    }
}

pub fn setup_overlay(
    mut commands: Commands,
    screen: Res<WindowInfo>,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let (font_x, font_y, font_z) = calculate_font_pos(screen);
    commands.spawn((
        Position,
        Text2d::new("dino x: 0"),
        TextColor(Color::BLACK),
        TextFont::from(font.clone()).with_font_size(20.),
        Transform::from_xyz(font_x, font_y, font_z),
    ));
}

fn calculate_font_pos(screen: Res<WindowInfo>) -> (f32, f32, f32) {
    let screen_width = screen.width as f32;
    let screen_height = screen.height as f32;
    let font_x = -(screen_width / 2. - 210.);
    let font_y = screen_height / 2. - 110.;
    (font_x, font_y, 0.)
}
