use bevy::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    physics::{Collider, Grounded, Velocity},
    restart::RestartRespawn,
};

#[derive(Component)]
pub struct Character;

#[derive(Resource)]
pub struct CharacterAnimations {
    pub idle_texture: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub run_texture: Handle<Image>,
    pub run_layout: Handle<TextureAtlasLayout>,
    pub jump_texture: Handle<Image>,
}

#[derive(Component, PartialEq)]
pub enum CharacterState {
    Idle,
    Running,
    Jumping,
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

    // Jump texture (no atlas, just a single image)
    let character_jump = asset_server.load("embedded://remrof/../assets/textures/jump.png");

    commands.insert_resource(CharacterAnimations {
        idle_texture: character_idle.clone(),
        idle_layout: character_idle_handle.clone(),
        run_texture: character_run.clone(),
        run_layout: character_run_handle,
        jump_texture: character_jump.clone(),
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
        Transform {
            scale: Vec3::splat(1.5),
            translation: Vec3::new(0.0, 0.0, 100.0),
            ..default()
        },
        AnimationIndices {
            idle: (0, 10),
            run: (0, 11),
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Character,
        CharacterState::Idle,
        Velocity { x: 0.0, y: 0.0, is_grounded: false },
        Grounded,
        Collider {
            size: Vec2::new(32.0 * 1.1, 32.0 * 1.5),
            offset: Vec2::new(0.0, -5.0),
        },
        RestartRespawn,
    ));
}

pub fn animate_character(
    time: Res<Time>,
    animations: Res<CharacterAnimations>,
    mut query: Query<(
        &mut CharacterState,
        &Velocity,
        &AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
) {
    for (mut state, velocity, indices, mut timer, mut sprite) in &mut query {
        let (texture, layout, current_indices) = match *state {
            CharacterState::Idle => (
                &animations.idle_texture,
                Some(&animations.idle_layout),
                indices.idle,
            ),
            CharacterState::Running => (
                &animations.run_texture,
                Some(&animations.run_layout),
                indices.run,
            ),
            CharacterState::Jumping => {
                // If we're jumping but our y velocity has gone back to 0, we're
                // not jumping anymore, so reset to something else.
                if velocity.y == 0.0 {
                    *state = CharacterState::Idle;
                    (
                        &animations.idle_texture,
                        Some(&animations.idle_layout),
                        indices.idle,
                    )
                } else {
                    (&animations.jump_texture, None, (0, 0))
                }
            }
        };

        let texture_changed = sprite.image != *texture;
        let atlas_changed = match (&sprite.texture_atlas, layout) {
            (Some(atlas), Some(layout_handle)) => atlas.layout != *layout_handle,
            (Some(_), None) => true, // Had an atlas but now using a plain texture
            (None, Some(_)) => true, // Had no atlas but now using one
            (None, None) => false,   // No change (both are plain textures)
        };
        let must_switch = texture_changed || atlas_changed;

        if must_switch {
            sprite.image = texture.clone();
            match layout {
                Some(layout) => {
                    sprite.texture_atlas = Some(TextureAtlas {
                        layout: layout.clone(),
                        index: current_indices.0,
                    });
                }
                None => {
                    sprite.texture_atlas = None;
                }
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
    let deceleration = 40.0;

    let query_res = query.get_single_mut();
    if let Ok((mut velocity, mut state, mut sprite)) = query_res {
        let move_left = keyboard_input.pressed(KeyCode::ArrowLeft);
        let move_right = keyboard_input.pressed(KeyCode::ArrowRight);

        if move_left {
            velocity.x -= acceleration;
        } else if move_right {
            velocity.x += acceleration;
        } else {
            if velocity.x > 0.0 {
                velocity.x = (velocity.x - deceleration).max(0.0);
            } else if velocity.x < 0.0 {
                velocity.x = (velocity.x + deceleration).min(0.0);
            }
        }

        velocity.x = velocity.x.clamp(-running_speed, running_speed);

        if move_left {
            sprite.flip_x = true;
        } else if move_right {
            sprite.flip_x = false;
        }

        *state = if velocity.y != 0.0 {
            CharacterState::Jumping
        } else if move_left || move_right {
            CharacterState::Running
        } else {
            CharacterState::Idle
        };
    }
}

pub fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Character>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let velocity = query.get_single_mut();
    if let Ok(mut velocity) = velocity {
        // TODO: Give some small buffer if we're moving downward.
        if velocity.is_grounded {
            velocity.y = 500.0;
        }
    }
}
