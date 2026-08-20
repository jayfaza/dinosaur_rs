use bevy::prelude::*;

use crate::step::Step;

#[derive(Component)]
pub struct DinoInfo {
    pub step: Step,
}

#[derive(Resource)]
pub struct DinoPos {
    pub x: i32,
    pub y: i32,
    pub max_x: f32,
    pub min_x: f32,
}

#[derive(Component)]
pub struct Dino;

pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_dino)
            .add_systems(Update, update_dino);
    }
}

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
    mut dino_pos: ResMut<DinoPos>,
) {
    for mut trans in translation {
        if keys.pressed(KeyCode::KeyA) {
            if trans.translation.x <= dino_pos.min_x {
                break;
            }
            trans.translation.x -= 2.;
            dino_pos.x -= 2;
        } else if keys.pressed(KeyCode::KeyD) {
            if trans.translation.x >= dino_pos.max_x {
                break;
            }
            trans.translation.x += 2.;
            dino_pos.x += 2;
        }
    }
}

pub fn setup_dino(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let dino = Dino::new(asset_server);
    let dino_info = DinoInfo { step: Step::Left };
    commands.spawn(dino);
    commands.spawn(dino_info);
}
