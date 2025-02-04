//! Handles restarting the game. For now.
//!
//! This will need to change and become level-specific, once levels are a thing.

use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Resource)]
pub struct RestartableSystems(pub Vec<SystemId>);

#[derive(Component)]
pub struct RestartRespawn;

pub fn respawn_restartable_on_command(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<RestartRespawn>>,
    mut commands: Commands,
    systems: Res<RestartableSystems>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for entity in &query {
            commands.entity(entity).despawn();
        }

        for system_id in &systems.0 {
            commands.run_system(*system_id);
        }
    }
}

pub fn quit_on_command(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
