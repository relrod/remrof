use bevy::prelude::*;

use crate::{physics::Collider, platform::Platform};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.3),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, -50.0, 0.0),
            scale: Vec3::new(100.0, 30.0, 1.0),
            ..default()
        },
        Platform,
        Collider {
            size: Vec2::new(100.0, 30.0),
            ..default()
        },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.3),
            ..default()
        },
        Transform {
            translation: Vec3::new(150.0, -80.0, 0.0),
            scale: Vec3::new(100.0, 30.0, 1.0),
            ..default()
        },
        Platform,
        Collider {
            size: Vec2::new(100.0, 30.0),
            ..default()
        },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.3),
            ..default()
        },
        Transform {
            translation: Vec3::new(350.0, -45.0, 0.0),
            scale: Vec3::new(10.0, 30.0, 1.0),
            ..default()
        },
        Platform,
        Collider {
            size: Vec2::new(10.0, 30.0),
            ..default()
        },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.3),
            ..default()
        },
        Transform {
            translation: Vec3::new(-180.0, -80.0, 0.0),
            scale: Vec3::new(10.0, 30.0, 1.0),
            ..default()
        },
        Platform,
        Collider {
            size: Vec2::new(10.0, 30.0),
            ..default()
        },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.3),
            ..default()
        },
        Transform {
            translation: Vec3::new(-300.0, -15.0, 0.0),
            scale: Vec3::new(10.0, 30.0, 1.0),
            ..default()
        },
        Platform,
        Collider {
            size: Vec2::new(10.0, 30.0),
            ..default()
        },
    ));
}
