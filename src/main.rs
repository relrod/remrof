#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::type_complexity)]

mod animation;
mod camera;
mod character;
mod embedded_assets;
mod level1;
mod physics;
mod platform;
mod restart;

use bevy::prelude::*;

use crate::{embedded_assets::EmbeddedAssetPlugin, restart::RestartableSystems};

fn main() {
    let mut app = App::new();

    let respawnables = RestartableSystems(vec![
        app.register_system(character::setup),
        app.register_system(camera::reset),
    ]);

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        EmbeddedAssetPlugin,
    ))
    .add_systems(
        Startup,
        (camera::setup, character::setup, level1::setup),
    )
    .add_systems(
        Update,
        (
            character::animate_character,
            character::move_character,
            character::jump,
            camera::track_character,
            camera::update_background,
            // draw_aabb_boxes,
            restart::respawn_restartable_on_command,
            restart::quit_on_command,
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            physics::apply_velocity,
            physics::apply_gravity,
            physics::check_for_collisions,
        )
            .chain(),
    )
    .insert_resource(respawnables)
    .run();
}

use crate::physics::{Collider, Grounded};
pub fn draw_aabb_boxes(
    mut gizmos: Gizmos,
    grounded: Query<(&mut Transform, &Collider), With<Grounded>>,
    colliders: Query<(&Transform, &Collider), (With<Collider>, Without<Grounded>)>,
) {
    for (collider_transform, collider_collider) in &colliders {
        let collider_center = collider_transform.translation.truncate() + collider_collider.offset;
        let collider_half_size = collider_collider.size;
        gizmos.rect_2d(
            collider_center,
            collider_half_size,
            Color::srgb(1.0, 0.0, 0.0),
        );
    }

    for (grounded_transform, grounded_collider) in &grounded {
        let grounded_center = grounded_transform.translation.truncate() + grounded_collider.offset;
        let grounded_half_size = grounded_collider.size;
        gizmos.rect_2d(
            grounded_center,
            grounded_half_size,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}
