use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ground)
            .add_systems(Update, update_ground);
    }
}

impl Ground {
    pub fn new(asset_server: AssetServer) -> (Ground, Sprite, Transform) {
        let sprite = Sprite::from_image(asset_server.load("ground.png"));
        let xyz = Transform::from_xyz(745., -35., 0.);
        (Ground, sprite, xyz)
    }
}

pub fn update_ground(ground: Query<&mut Transform, With<Ground>>) {
    for mut trans in ground {
        if trans.translation.x > -722. {
            trans.translation.x -= 5.;
        } else {
            trans.translation.x = 0.;
        }
    }
}

pub fn setup_ground(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let ground = Ground::new(asset_server.clone());
    commands.spawn(ground);
}
