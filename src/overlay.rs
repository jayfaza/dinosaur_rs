use bevy::prelude::*;

use crate::dino::Dino;

#[derive(Component)]
pub struct Position;

pub fn update_overlay(
    mut text_query: Query<&mut Text2d, With<Position>>,
    dino_pos: Query<&Transform, With<Dino>>,
    mut writer: TextUiWriter,
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
