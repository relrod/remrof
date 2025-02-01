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
