use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component, Debug)]
pub struct Cookie {
    pub score: u32,
    pub give_amount: f32,
}

