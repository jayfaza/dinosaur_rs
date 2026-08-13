use crate::dino::{Dino, DinoInfo};
use crate::models::Models;
use bevy::prelude::*;

#[derive(Resource)]
pub struct StepTimer(pub Timer);

pub struct StepPlugin;

impl Plugin for StepPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(0.13, TimerMode::Repeating)));
        app.add_systems(Update, update_step);
    }
}

pub enum Step {
    Left,
    Right,
}

fn is_step(time: Res<Time>, mut timer: ResMut<StepTimer>) -> bool {
    if timer.0.tick(time.delta()).just_finished() {
        true
    } else {
        false
    }
}

pub fn update_step(
    models: Query<&Models>,
    time: Res<Time>,
    timer: ResMut<StepTimer>,
    mut dino_info: Query<&mut DinoInfo>,
    dino: Query<(&Dino, &mut Sprite, &Transform)>,
) {
    use crate::step::Step;
    if is_step(time, timer) {
        for (_, mut sprite, _) in dino {
            for models in &models {
                for mut dinoinfo in &mut dino_info {
                    match dinoinfo.step {
                        Step::Left => {
                            *sprite = models.dino_step2.clone();
                            dinoinfo.step = Step::Right;
                        }
                        Step::Right => {
                            *sprite = models.dino_step1.clone();
                            dinoinfo.step = Step::Left;
                        }
                    }
                }
            }
        }
    }
}
