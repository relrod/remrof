use bevy::prelude::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_character, move_character))
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
            CharacterState::Idle => (&animations.idle_texture, &animations.idle_layout, indices.idle),
            CharacterState::Running => (&animations.run_texture, &animations.run_layout, indices.run),
        };

        // If we go from running to idle or vice verse, swap the texture and layout.
        let must_switch =
            sprite.image != *texture ||
            sprite.texture_atlas.as_ref().map(|atlas| atlas.layout != *layout).unwrap_or(true);

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
    mut query: Query<(&mut Transform, &mut CharacterState, &mut Sprite), With<Character>>,
    time: Res<Time>
) {
    let speed = 200.0;
    let (mut transform, mut state, mut sprite) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed * time.delta_secs();
        *state = CharacterState::Running;
        sprite.flip_x = true;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed * time.delta_secs();
        *state = CharacterState::Running;
        sprite.flip_x = false;
    } else {
        *state = CharacterState::Idle;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Idle texture and atlas
    let character_idle = asset_server.load("textures/idle.png");
    let character_idle_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        11,
        1,
        None,
        None,
    );
    let character_idle_handle = texture_atlas_layouts.add(character_idle_layout);

    // Run texture and atlas
    let character_run = asset_server.load("textures/run.png");
    let character_run_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        12,
        1,
        None,
        None,
    );
    let character_run_handle = texture_atlas_layouts.add(character_run_layout);

    // Camera
    commands.spawn(Camera2d);

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
                index: 0
            }
        ),
        Transform::from_scale(Vec3::splat(1.5)),
        AnimationIndices {
            idle: (0, 10),
            run: (0, 11),
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Character,
        CharacterState::Idle,
    ));


}
