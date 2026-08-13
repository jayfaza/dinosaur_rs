use bevy::prelude::*;

use crate::Step;

#[derive(Component)]
pub struct DinoInfo {
    pub step: Step,
}

#[derive(Component)]
pub struct Dino;

impl Dino {
    pub fn new(asset_server: ResMut<AssetServer>) -> (Self, Sprite, Transform) {
        (
            Dino,
            Sprite::from_image(asset_server.load("dinosaur-static.png")),
            Transform::from_xyz(0., 0., 0.),
        )
    }
}

pub fn update_dino(
    keys: Res<ButtonInput<KeyCode>>,
    translation: Query<&mut Transform, With<Dino>>,
) {
    for mut trans in translation {
        if keys.pressed(KeyCode::KeyA) {
            if trans.translation.x <= -410. {
                break;
            }
            trans.translation.x -= 2.;
        } else if keys.pressed(KeyCode::KeyD) {
            if trans.translation.x >= 410. {
                break;
            }
            trans.translation.x += 2.;
        }
    }
}
