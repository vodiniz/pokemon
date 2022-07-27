use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

//load and spawn ldk map from file
pub fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("./map/main_map.ldtk");

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
