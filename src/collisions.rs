use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::LdtkEntity, GridCoords, IntGridCell, LdtkIntCell};
use bevy_rapier2d::prelude::*;

//Creating Components and bundles for each integer Grid in LDTK for future map collision

//Floor Component and bundle for walkkable tiles
#[derive(Component, Default)]
pub struct Floor;

#[derive(Bundle, LdtkIntCell)]
pub struct FloorBundle {
    floor: Floor,
}

//Wall Component and bundle for collision and immovable objects
#[derive(Component, Default)]
pub struct Wall;

#[derive(Component, Bundle, LdtkIntCell, LdtkEntity)]
pub struct WallBundle {
    wall: Wall,
    #[grid_coords]
    grid_coords: GridCoords, // #[from_int_grid_cell]
                             // pub collider: Collider,
                             // #[from_int_grid_cell]
                             // pub rigid_body: RigidBody,
                             // #[from_int_grid_cell]
                             // int_grid_cell: IntGridCell,
}

// impl From<IntGridCell> for WallBundle {
//     fn from(int_grid_cell: IntGridCell) -> WallBundle {
//         WallBundle {
//             wall: Wall,
//             collider: Collider::cuboid(1.6, 1.6),
//             rigid_body: RigidBody::Fixed,
//             int_grid_cell: IntGridCell { value: int_grid_cell.value},
//         }
//     }
// }

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

// currently spawning collision rigid bodies on top of tiles -> If I could make tiles have collision would be better.
pub fn assign_wall_collision(
    mut commands: Commands,
    wall_query: Query<(Entity, &GridCoords), (Added<Wall>, Without<RigidBody>)>,
) {
    for (wall, coord) in wall_query.iter() {
        commands
            .entity(wall)
            .insert(RigidBody::Fixed)
            .insert(Name::new("Walls"))
            .with_children(|children| {
                children.spawn().insert_bundle((
                    Collider::cuboid(8., 8.),
                    Transform::from_xyz((coord.x * 16 + 8) as f32, (coord.y * 16 + 8) as f32, 0.),
                ));
            });
    }
}
