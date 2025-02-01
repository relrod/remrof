use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let gravity_factor = 300.0;

    for mut velocity in &mut query {
        velocity.y -= gravity_factor * time.delta_secs();
    }
}
