
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkIntCell, LdtkWorldBundle};



//Creating Components and bundles for each integer Grid in LDTK for future map collision


//Floor Component and bundle for walkkable tiles
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Floor;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct FloorBundle {
    floor: Floor,
}

//Wall Component and bundle for collision and immovable objects
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}


//Water Component and bundle for surfable, fishable and for collision (without surf)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Water;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WaterBundle {
    water: Water,
}

//Grass Component and bundle for pokemon encounters
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Grass;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GrassBundle {
    grass: Grass,
}

//Door Component and bundle for entering buildings
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Door;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct DoorBundle {
    door: Door,
}

//Platform Component for Dropping down in specific diirection and immovable on the other direction.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Platform;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PlatformBundle {
    platform: Platform,
}



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
