use crate::player;
use bevy::prelude::{Commands, OrthographicCameraBundle, Query, Transform, Without, With};
use bevy_rapier2d::prelude::RigidBody;



//spawn basic 2d camera
pub fn setup_camera(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);
}


//MUST INSERT TRANSFORM INTO CAMERA FOR MODIFYING TRANSLATION
pub fn follow_player(
    mut camera_query: Query<(
    &mut bevy::render::camera::OrthographicProjection,
    &mut Transform,
    ),
    Without<player::Player>
    >,
    player_position: Query<& Transform, (With<player::Player>, With<RigidBody>)>
){


    for position in player_position.iter() {
        if let Ok((_, mut camera_transform)) = camera_query.get_single_mut() {
            println!("camera");
            // dbg!(&camera_query);
            // dbg!("____________________________________");
            camera_transform.translation.x = position.translation.x;
            camera_transform.translation.y = position.translation.y;
        }
    }
}
