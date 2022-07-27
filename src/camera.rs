use crate::player;
use bevy::{prelude::*, render::camera};

#[derive(Component)]
//tag maincamera for future query
pub struct MainCamera;

//spawn basic 2d camera
pub fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode =
        bevy::render::camera::ScalingMode::FixedHorizontal;
    camera.transform.scale = Vec3::new(16. * 8., 16. * 8., 2.);

    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}

//Camera system to follow player translation.
//QUery mainCamera mutable transform and player transform.
pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<player::Player>, Without<MainCamera>)>,
) {
    //check if transform translation result has a value and assign it;
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        // check if camera transform result is ok and modify it acording to player
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_translation.x;
            camera_transform.translation.y = player_translation.y;
        }
    }
}
