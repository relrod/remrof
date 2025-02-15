use bevy::prelude::*;

use crate::character::Character;

#[derive(Component)]
pub struct CharacterCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, CharacterCamera));
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
    let direction = Vec3::new(x, y, camera_transform.translation.z);
    camera_transform.translation = camera_transform
        .translation
        .lerp(direction, time.delta_secs() * 2.0);
}
