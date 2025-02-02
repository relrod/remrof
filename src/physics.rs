use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// A Grounded entity is affected by (i.e. falls onto) platforms/the ground.
#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            size: Vec2::new(32.0, 32.0),
            offset: Vec2::new(0.0, 0.0),
        }
    }
}

/// From which direction did the collision occur?
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn check_for_collisions(
    mut grounded: Query<(&mut Velocity, &mut Transform, &Collider), With<Grounded>>,
    colliders: Query<(&Transform, &Collider), (With<Collider>, Without<Grounded>)>,
) {
    // For every "Grounded" component, see if it is colliding with a "Collider".
    // A grounded thing can also be a collider (e.g. two players can collide
    // with each other).
    for (mut grounded_velocity, mut grounded_transform, grounded_collider) in &mut grounded {
        for (collider_transform, collider_collider) in &colliders {
            // We need center and half-sizes for both entities.
            let grounded_center =
                grounded_transform.translation.truncate() + grounded_collider.offset;
            let grounded_half_size = grounded_collider.size / 2.0;

            let collider_center =
                collider_transform.translation.truncate() + collider_collider.offset;
            let collider_half_size = collider_collider.size / 2.0;

            // Create bounding boxes for both entities.
            let grounded_box = Aabb2d::new(grounded_center, grounded_half_size);
            let collider_box = Aabb2d::new(collider_center, collider_half_size);

            let collision = collision_direction(&grounded_box, &collider_box);
            if let Some(collision) = collision {
                match collision {
                    Collision::Left => {
                        if grounded_velocity.x > 0.0 {
                            grounded_velocity.x = 0.0;
                            grounded_transform.translation.x =
                                collider_box.min.x - grounded_half_size.x;
                        }
                    }
                    Collision::Right => {
                        if grounded_velocity.x < 0.0 {
                            grounded_velocity.x = 0.0;
                            grounded_transform.translation.x =
                                collider_box.max.x + grounded_half_size.x;
                        }
                    }
                    Collision::Top => {
                        if grounded_velocity.y < 0.0 {
                            grounded_velocity.y = 0.0;
                            grounded_transform.translation.y =
                                collider_box.max.y + grounded_half_size.y;
                        }
                    }
                    Collision::Bottom => {
                        if grounded_velocity.y > 0.0 {
                            grounded_velocity.y = 0.0;
                            grounded_transform.translation.y =
                                collider_box.min.y - grounded_half_size.y;
                        }
                    }
                }
            }
        }
    }
}

fn collision_direction(grounded_box: &Aabb2d, collider_box: &Aabb2d) -> Option<Collision> {
    if !grounded_box.intersects(collider_box) {
        return None;
    }

    let closest = collider_box.closest_point(grounded_box.center());
    let offset = grounded_box.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x > 0.0 {
            Collision::Right
        } else {
            Collision::Left
        }
    } else if offset.y > 0.0 {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let gravity_factor = 700.0;

    for mut velocity in &mut query {
        velocity.y -= gravity_factor * time.delta_secs();
    }
}
