#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

mod embedded_assets;

use crate::embedded_assets::EmbeddedAssetPlugin;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct AnimationIndices {
    idle: (usize, usize),
    run: (usize, usize),
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Character;

#[derive(Resource)]
struct CharacterAnimations {
    idle_texture: Handle<Image>,
    idle_layout: Handle<TextureAtlasLayout>,
    run_texture: Handle<Image>,
    run_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, PartialEq)]
enum CharacterState {
    Idle,
    Running,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EmbeddedAssetPlugin,
        ))
        .add_systems(Startup, setup)
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    // Idle texture and atlas
    let character_idle = asset_server.load("embedded://remrof/../assets/textures/idle.png");
    let character_idle_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 1, None, None);
    let character_idle_handle = texture_atlas_layouts.add(character_idle_layout);

    // Run texture and atlas
    let character_run = asset_server.load("embedded://remrof/../assets/textures/run.png");
    let character_run_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 1, None, None);
    let character_run_handle = texture_atlas_layouts.add(character_run_layout);

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

    commands.insert_resource(CharacterAnimations {
        idle_texture: character_idle.clone(),
        idle_layout: character_idle_handle.clone(),
        run_texture: character_run.clone(),
        run_layout: character_run_handle,
    });

    // Our lil' character
    commands.spawn((
        Sprite::from_atlas_image(
            character_idle,
            TextureAtlas {
                layout: character_idle_handle,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(1.5)),
        AnimationIndices {
            idle: (0, 10),
            run: (0, 11),
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Character,
        CharacterState::Idle,
        Velocity { x: 0.0, y: 0.0 },
    ));
}
