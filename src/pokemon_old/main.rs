mod camera;
mod ldtk_map;
mod lib;
mod player;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_ecs_ldtk::prelude::*;

fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_startup_system(camera::setup_camera)
        .add_startup_system(ldtk_map::setup_ldtk)
        .add_startup_system(player::spawn_player.after(ldtk_map::setup_ldtk))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(player::player_movement),
        )
        .add_system(camera::camera_system.after(player::player_movement))
        .run();
}

fn main() {
    run();
    //ldtk_map::run_ldtk_map();
    //snakegame::run_snakegame();
    //player::spawn_player_test();
}
