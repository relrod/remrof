use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub idle: (usize, usize),
    pub run: (usize, usize),
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
