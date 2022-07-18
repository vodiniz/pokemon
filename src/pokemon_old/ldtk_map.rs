use bevy::prelude::{AssetServer, Commands, Res, Transform, Vec3};
use bevy_ecs_ldtk::prelude::*;

pub fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("./map/pallet_town.ldtk");

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
