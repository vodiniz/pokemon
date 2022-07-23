use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const DEFAULT_PLAYER_VELOCITY: f32 = 100.;

#[derive(Component)]
pub struct Player;

//function to spawn the player on the map
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("./sprites/characters/hero_simple.png"), // get the texture handle from file
            ..default()
        })
        .insert(Player)
        //add rigid body to player (for rapier simulations),
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1., 2.))
        .insert(Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        })
        .insert(GravityScale(0.))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            2. * 16. + 15. / 2.,
            2. * 16. + 19. / 2.,
            10.0)))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}

//set player velocity based on key press
// can probably change to a more idiomatic
//way with match but had some problems when trying

pub fn player_movement(keyboard_input: Res<Input<KeyCode>>, mut velocities: Query<&mut Velocity>) {
    for mut vel in velocities.iter_mut() {
        let key = keyboard_input.get_pressed().next();
        if let Some(key) = key {
            match key {
                KeyCode::Up => vel.linvel = Vec2::new(0., DEFAULT_PLAYER_VELOCITY),
                KeyCode::Down => vel.linvel = Vec2::new(0.0, -DEFAULT_PLAYER_VELOCITY),
                KeyCode::Left => vel.linvel = Vec2::new(-DEFAULT_PLAYER_VELOCITY, 0.),
                KeyCode::Right => vel.linvel = Vec2::new(DEFAULT_PLAYER_VELOCITY, 0.),
                _ => (),
            }
        } else {
            vel.linvel = Vec2::new(0., 0.);
        }
    }
}