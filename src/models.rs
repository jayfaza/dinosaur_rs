use bevy::prelude::*;

#[derive(Component)]
pub struct Models {
    pub dino_static: Sprite,
    pub dino_step2: Sprite,
    pub dino_step1: Sprite,
    pub dino_defeat: Sprite,
    pub ground: Sprite,
}

impl Models {
    pub fn new(asset_server: ResMut<AssetServer>) -> Self {
        let dino_static = Sprite::from_image(asset_server.load("dinosaur-static.png"));
        let dino_step2 = Sprite::from_image(asset_server.load("dinosaur-walk2.png"));
        let dino_step1 = Sprite::from_image(asset_server.load("dinosaur-walk1.png"));
        let dino_defeat = Sprite::from_image(asset_server.load("dinosaur-defeat.png"));
        let ground = Sprite::from_image(asset_server.load("ground.png"));

        Models {
            dino_static: dino_static,
            dino_step2: dino_step2,
            dino_step1: dino_step1,
            dino_defeat: dino_defeat,
            ground: ground,
        }
    }
}
