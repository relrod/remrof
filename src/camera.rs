use bevy::{prelude::*, window::PrimaryWindow};

use crate::character::Character;

#[derive(Component)]
pub struct CharacterCamera;

#[derive(Component)]
pub struct Background;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn((Camera2d, CharacterCamera));

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
        Background,
    ));
}

/// On restart, reset the camera right away rather than slowly
/// lerping it back to the character.
pub fn reset(mut camera_transform: Query<&mut Transform, With<CharacterCamera>>) {
    let Ok(mut camera_transform) = camera_transform.get_single_mut() else {
        return;
    };

    camera_transform.translation = Vec3::new(0.0, 0.0, 0.0);
}

/// Track the character with the camera.
///
/// Make the tracking smooth by applying linear interpolation (lerp).
pub fn track_character(
    mut camera_transform: Query<&mut Transform, With<CharacterCamera>>,
    character_transform: Query<&Transform, (With<Character>, Without<CharacterCamera>)>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_transform.get_single_mut() else {
        return;
    };

    let Ok(character_transform) = character_transform.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = character_transform.translation;
    let direction = Vec3::new(x, camera_transform.translation.y, camera_transform.translation.z);
    camera_transform.translation = camera_transform
        .translation
        .lerp(direction, time.delta_secs() * 2.0);
}

/// Move the background to match the camera position, but keep it in world space.
pub fn update_background(
    camera: Query<&Transform, With<CharacterCamera>>,
    mut bg: Query<&mut Transform, (With<Background>, Without<CharacterCamera>)>,
) {
    let Ok(camera_transform) = camera.get_single() else {
        return;
    };

    let Ok(mut bg_transform) = bg.get_single_mut() else {
        return;
    };

    bg_transform.translation.x = camera_transform.translation.x;
    bg_transform.translation.y = camera_transform.translation.y;
}
