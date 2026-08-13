use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

impl Ground {
    pub fn new(asset_server: AssetServer) -> (Ground, Sprite, Transform) {
        let sprite = Sprite::from_image(asset_server.load("ground.png"));
        let xyz = Transform::from_xyz(745., -35., 0.);
        (Ground, sprite, xyz)
    }
}

pub fn update_ground(asset_server: Res<AssetServer>, ground: Query<&mut Transform, With<Ground>>) {
    for mut trans in ground {
        if trans.translation.x > -722. {
            trans.translation.x -= 5.;
        } else {
            trans.translation.x = 0.;
        }
    }
}
