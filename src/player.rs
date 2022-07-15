use bevy::core::FixedTimestep;
use bevy::prelude::*;

const ARENA_HEIGHT: u32 = 9;
const ARENA_WIDTH: u32 = 16;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            99.0,
        );
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("./sprites/characters/hero_simple.png"),
            transform: Transform{
                scale: Vec3::new(4.0, 4.0, 99.0),
            ..default()
        },
            ..default()
        })
        .insert(Player)
        .insert(Position { x: 3, y: 3 });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_position: Query<&mut Position, With<Player>>,
) {
    for mut pos in player_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}


pub fn spawn_player_test() {

    App::new()
    .add_startup_system(setup_camera) // <--
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_player) // <--
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation)
            .with_system(size_scaling),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.150))
            .with_system(player_movement),
    )
    .run();
}

