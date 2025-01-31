use bevy::prelude::*;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_character)
        .run();
}

fn animate_character(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let character_idle = asset_server.load("textures/idle.png");
    let character_texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        12,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlas_layouts.add(character_texture_atlas);

    // Camera
    commands.spawn(Camera2d);

    // Our lil' character
    commands.spawn((
        Sprite::from_atlas_image(
            character_idle,
            TextureAtlas {
                layout: texture_atlas_handle,
                index: 0
            }
        ),
        Transform::from_scale(Vec3::splat(1.5)),
        AnimationIndices { first: 0, last: 10 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
