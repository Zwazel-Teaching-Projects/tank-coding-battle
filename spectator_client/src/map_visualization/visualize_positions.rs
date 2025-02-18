use bevy::{
    color::palettes::css::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    prelude::*,
};
use shared::{
    game::{player_handling::TankTransform, tank_types::TankType},
    networking::messages::message_data::game_starts::GameStarts,
};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyPositionGizmos {}

const NEIGHBOR_SIZE: f32 = 0.3;
const CENTER_SIZE: f32 = 0.5;

/// Visualizes neighboring cells of a given cell and the cell itself
pub fn visualize_cells(
    mut my_gizmos: Gizmos<MyPositionGizmos>,
    game_config: Res<GameStarts>,
    tanks: Query<(&TankTransform, &TankType)>,
) {
    let map_definition = &game_config.map_definition;
    let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);

    for (tank_transform, tank_type) in tanks.iter() {
        let position = tank_transform.position;
        let tank_config = game_config
            .tank_configs
            .get(tank_type)
            .expect("Failed to get tank config");

        let closest_tile = map_definition
            .get_closest_tile(position)
            .expect(format!("Failed to get closest tile to position {:?}", position).as_str());
        if let Some(tile) = map_definition
            .get_real_world_position_of_tile(closest_tile.x, closest_tile.y)
        {
            my_gizmos.circle(Isometry3d::new(tile, rotation), CENTER_SIZE, GREEN);

            let neighbors = map_definition
                .get_neighbours(closest_tile.x, closest_tile.y);

            // Center
            let center_position = map_definition
                .get_real_world_position_of_tile(neighbors.center.x, neighbors.center.y)
                .expect(
                    format!(
                        "Failed to get real world position of tile {:?}",
                        neighbors.center
                    )
                    .as_str(),
                );
            my_gizmos.circle(
                Isometry3d::new(center_position, rotation),
                NEIGHBOR_SIZE,
                ORANGE,
            );

            // North
            if let Some(north) = neighbors.north {
                let north_position = map_definition
                    .get_real_world_position_of_tile(north.x, north.y)
                    .expect(
                        format!("Failed to get real world position of tile {:?}", north).as_str(),
                    );
                my_gizmos.circle(
                    Isometry3d::new(north_position, rotation),
                    NEIGHBOR_SIZE,
                    GREEN,
                );
                my_gizmos.line(
                    center_position,
                    north_position,
                    get_color_for_height_difference(
                        tank_config.max_slope,
                        center_position.y,
                        north_position.y,
                    ),
                );
            }

            // East
            if let Some(east) = neighbors.east {
                let east_position = map_definition
                    .get_real_world_position_of_tile(east.x, east.y)
                    .expect(
                        format!("Failed to get real world position of tile {:?}", east).as_str(),
                    );
                my_gizmos.circle(
                    Isometry3d::new(east_position, rotation),
                    NEIGHBOR_SIZE,
                    BLUE,
                );
                my_gizmos.line(
                    center_position,
                    east_position,
                    get_color_for_height_difference(
                        tank_config.max_slope,
                        center_position.y,
                        east_position.y,
                    ),
                );
            }

            // South
            if let Some(south) = neighbors.south {
                let south_position = map_definition
                    .get_real_world_position_of_tile(south.x, south.y)
                    .expect(
                        format!("Failed to get real world position of tile {:?}", south).as_str(),
                    );
                my_gizmos.circle(
                    Isometry3d::new(south_position, rotation),
                    NEIGHBOR_SIZE,
                    RED,
                );
                my_gizmos.line(
                    center_position,
                    south_position,
                    get_color_for_height_difference(
                        tank_config.max_slope,
                        center_position.y,
                        south_position.y,
                    ),
                );
            }

            // West
            if let Some(west) = neighbors.west {
                let west_position = map_definition
                    .get_real_world_position_of_tile(west.x, west.y)
                    .expect(
                        format!("Failed to get real world position of tile {:?}", west).as_str(),
                    );
                my_gizmos.circle(
                    Isometry3d::new(west_position, rotation),
                    NEIGHBOR_SIZE,
                    YELLOW,
                );
                my_gizmos.line(
                    center_position,
                    west_position,
                    get_color_for_height_difference(
                        tank_config.max_slope,
                        center_position.y,
                        west_position.y,
                    ),
                );
            }
        }
    }
}

// A function that compares the height of two tiles, if they're higher than a certain threshold, return red as a color, otherwise white
fn get_color_for_height_difference(max_slope: f32, tile1: f32, tile2: f32) -> Color {
    let height_difference = (tile1 - tile2).abs();
    if height_difference > max_slope {
        RED.into()
    } else {
        WHITE.into()
    }
}
