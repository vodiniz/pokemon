use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::*;
use bevy_rapier2d::prelude::*;
use player::player_movement;

//import all modules
mod camera;
mod ldtk_map;
mod player;
mod collisions;

//Main game function
fn run() {
    //Initialize all systems
    App::new()
        //default plugins such as editor for debuggin, rapier basics and bevy basics and rapier debug
        .add_plugins(DefaultPlugins)
        //easy editor plugin for debugging value and entities
        .add_plugin(EditorPlugin)
        // change units for the rapier plugin, as for rapier 1px = 1meter
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.))
        //rapier debug from seeing the collisions
        .add_plugin(RapierDebugRenderPlugin::default())
        //default ldtk plugin
        .add_plugin(LdtkPlugin)
        //
        //
        // add resources
        .insert_resource(LevelSelection::Uid(0)) // level id from ldtk file current world map is 0;
        //startup system only run one at the start of the program
        .add_startup_system(camera::setup_camera)
        .add_startup_system(player::spawn_player)
        .add_startup_system(ldtk_map::load_map)
        //
        //systems running at every frame
        .add_system(player::player_movement)
        .add_system(camera::follow_player.after(player_movement))
        //
        //
        //register ldtk int cell to components
        .register_ldtk_int_cell::<collisions::FloorBundle>(1)
        .register_ldtk_int_cell::<collisions::WallBundle>(2)
        .register_ldtk_int_cell::<collisions::WaterBundle>(3)
        .register_ldtk_int_cell::<collisions::GrassBundle>(4)
        .register_ldtk_int_cell::<collisions::DoorBundle>(5)
        .register_ldtk_int_cell::<collisions::PlatformBundle>(6)


        //System to add collisions to ldkt int cells
        .add_system(collisions::assign_wall_collision)
        .run();


}

fn main() {
    run();
}
