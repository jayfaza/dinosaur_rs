use bevy::prelude::*;

use crate::{Dino, DinoInfo, Ground, Models, Step};
use crate::overlay::Position;

pub fn setup_dino(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let dino = Dino::new(asset_server);
    let dino_info = DinoInfo { step: Step::Left };
    commands.spawn(dino);
    commands.spawn(dino_info);
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_models(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Models::new(asset_server));
}

pub fn setup_ground(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let ground = Ground::new(asset_server.clone());
    commands.spawn(ground);
}

pub fn setup_overlay(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Position,
        Text2d::new("dino x: 0"),
        TextColor(Color::BLACK),
        TextFont::from(font.clone()).with_font_size(60.),
        Transform::from_xyz(-485., 335., 0.)
    ));
}
