use crate::player;
use bevy::{prelude::*, render::camera};

#[derive(Component)]
pub struct MainCamera;


pub fn setup_camera(mut commands: Commands) {
    let far = 1000.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far,
        depth_calculation: camera::DepthCalculation::ZDifference,
        scaling_mode: camera::ScalingMode::FixedHorizontal,
        ..Default::default()
    };
    camera.transform.scale = Vec3::new(10. * 16., 10. * 16., 1.);

    commands.spawn_bundle(camera);
}



pub fn camera_system(
    mut camera_query: Query<(
    &mut bevy::render::camera::OrthographicProjection,
    &mut Transform,
    ),
    Without<player::Player>
    >,
    player_query: Query<&Transform, With<player::Player>>
){
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        if let Ok((_, mut camera_transform)) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_translation.x;
            camera_transform.translation.y = player_translation.y;
        }
    }
}
