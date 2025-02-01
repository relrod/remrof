#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

mod animation;
mod character;
mod embedded_assets;
mod physics;

use crate::embedded_assets::EmbeddedAssetPlugin;
use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EmbeddedAssetPlugin,
        ))
        .add_systems(Startup, (setup, character::setup))
        .add_systems(
            Update,
            (
                character::animate_character,
                character::move_character,
                character::jump,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                physics::apply_velocity,
                physics::apply_gravity,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    // Camera
    commands.spawn(Camera2d);

    // Background
    let window = query.single();
    commands.spawn((
        Sprite {
            image: asset_server.load("embedded://remrof/../assets/bg/green.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            custom_size: Some(window.size()),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
    ));
}
