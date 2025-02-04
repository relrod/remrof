use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub is_grounded: bool,
}

/// A Grounded entity is affected by (i.e. falls onto) platforms/the ground.
/// It gives a way to differentiate between "dynamic" entities (e.g. the player)
/// (which would be a "Grounded" entity) and "static" entities (e.g. the ground
/// or a platform), which would not have this component but would still have a
/// Collider component.
#[derive(Component)]
pub struct Grounded;

/// A Collider component describes the size and offset of an entity's collider.
#[derive(Component)]
pub struct Collider {
    /// The size of the collider, *with* scale applied.
    pub size: Vec2,
    /// The offset of the collider from the entity's center.
    /// This can be used to account for e.g. PNGs with transparent space around
    /// the sprite in some direction.
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
                            grounded_velocity.is_grounded = true;
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

/// Given two AABB boxes, determine if they are currently colliding, and if so
/// in from which direction?
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
    let gravity_factor = 1000.0;

    for mut velocity in &mut query {
        if !velocity.is_grounded {
            velocity.y -= gravity_factor * time.delta_secs();
        }
    }
}
