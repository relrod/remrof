use bevy::prelude::*;

use crate::{animation::AnimationIndices, animation::AnimationTimer, physics::Velocity};

#[derive(Component)]
pub struct Character;

#[derive(Resource)]
pub struct CharacterAnimations {
    pub idle_texture: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub run_texture: Handle<Image>,
    pub run_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, PartialEq)]
pub enum CharacterState {
    Idle,
    Running,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Idle texture and atlas
    let character_idle = asset_server.load("embedded://remrof/../assets/textures/idle.png");
    let character_idle_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 1, None, None);
    let character_idle_handle = texture_atlas_layouts.add(character_idle_layout);

    // Run texture and atlas
    let character_run = asset_server.load("embedded://remrof/../assets/textures/run.png");
    let character_run_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 1, None, None);
    let character_run_handle = texture_atlas_layouts.add(character_run_layout);

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

pub fn animate_character(
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

pub fn move_character(
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

pub fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Character>>,
) {
    let mut velocity = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.y = 400.0;
    }
}
