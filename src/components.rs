use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component, Debug)]
pub struct Cookie {
    pub give_amount: f32,
}

#[derive(Resource)]
pub struct Score(pub u32);

