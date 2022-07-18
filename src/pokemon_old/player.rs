use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("./sprites/characters/hero_simple.png"),
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 15.0),
                translation: Vec3::new(8., 10., 3.),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 16.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 16.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 16.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 16.;
        }
    }
}

