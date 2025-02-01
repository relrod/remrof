#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

mod animation;
mod character;
mod embedded_assets;
mod physics;

use crate::{
    animation::AnimationIndices, animation::AnimationTimer, character::Character,
    character::CharacterAnimations, character::CharacterState,
    embedded_assets::EmbeddedAssetPlugin, physics::Velocity,
};
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
                animate_character,
                move_character,
                apply_velocity,
                apply_gravity,
                jump,
            ),
        )
        .run();
}

fn animate_character(
    time: Res<Time>,
    animations: Res<CharacterAnimations>,
    mut query: Query<(
        &CharacterState,
        &AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
) {
    for (state, indices, mut timer, mut sprite) in &mut query {
        let (texture, layout, current_indices) = match state {
            CharacterState::Idle => (
                &animations.idle_texture,
                &animations.idle_layout,
                indices.idle,
            ),
            CharacterState::Running => {
                (&animations.run_texture, &animations.run_layout, indices.run)
            }
        };

        // If we go from running to idle or vice verse, swap the texture and layout.
        let must_switch = sprite.image != *texture
            || sprite
                .texture_atlas
                .as_ref()
                .map_or(true, |atlas| atlas.layout != *layout);

        if must_switch {
            sprite.image = texture.clone();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.layout = layout.clone();
                atlas.index = current_indices.0;
            }
        }

        // Animate the current frame
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == current_indices.1 {
                    current_indices.0
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn move_character(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut CharacterState, &mut Sprite), With<Character>>,
) {
    let running_speed = 300.0;
    let acceleration = 20.0;

    let (mut velocity, mut state, mut sprite) = query.single_mut();
    let move_left = keyboard_input.pressed(KeyCode::ArrowLeft);
    let move_right = keyboard_input.pressed(KeyCode::ArrowRight);

    let target_speed = if move_left {
        -running_speed
    } else if move_right {
        running_speed
    } else {
        0.0
    };

    if velocity.x < target_speed {
        velocity.x += acceleration;
    } else if velocity.x > target_speed {
        velocity.x -= acceleration;
    }

    if move_left {
        sprite.flip_x = true;
    } else if move_right {
        sprite.flip_x = false;
    }

    *state = if move_left || move_right {
        CharacterState::Running
    } else {
        CharacterState::Idle
    };
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let gravity_factor = 300.0;

    for mut velocity in &mut query {
        velocity.y -= gravity_factor * time.delta_secs();
    }
}

fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Character>>,
) {
    let mut velocity = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.y = 400.0;
    }
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
