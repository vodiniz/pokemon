mod ldtk_map;
mod lib;
mod player;

use bevy::{prelude::*, core::FixedTimestep};
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
        .add_startup_system(ldtk_map::setup)
        .add_startup_system(player::spawn_player)
        // .add_system_set_to_stage(
        //     CoreStage::PostUpdate,
        //     SystemSet::new()
        //         .with_system(player::position_translation)
        //         .with_system(player::size_scaling),
        // )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(0.150))
        //         .with_system(player::player_movement))
        .add_system(ldtk_map::camera_fit_inside_current_level)
        .run();
}

fn main() {
    //run();
    ldtk_map::run_ldtk_map();
    //snakegame::run_snakegame();
    //player::spawn_player_test();
}
