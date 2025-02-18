use bevy::prelude::*;
use shared::{
    asset_handling::{
        config::{TankConfig, TankConfigSystemParam},
        maps::MapDefinition,
    },
    game::{player_handling::TankTransform, tank_types::TankType},
    networking::{
        lobby_management::{InLobby, MyLobby},
        messages::{
            message_container::MoveTankCommandTrigger, message_data::tank_messages::MoveDirection,
        },
    },
};

pub fn handle_tank_movement(
    trigger: Trigger<MoveTankCommandTrigger>,
    mut client: Query<(&mut TankTransform, &TankType, &InLobby)>,
    tank_config: TankConfigSystemParam,
    lobby: Query<&MyLobby>,
) {
    let client_entity = trigger.entity();
    let (mut tank_transform, tank_type, in_lobby) = client
        .get_mut(client_entity)
        .expect("Failed to get tank transform");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");
    let direction = match trigger.direction {
        MoveDirection::Forward => 1.0,
        MoveDirection::Backward => -1.0,
    };
    let speed = tank_config.move_speed.min(trigger.distance);
    let distance = direction * speed;
    let move_direction = tank_transform.rotation * Vec3::new(0.0, 0.0, distance);
    let next_tank_position = tank_transform.position + move_direction;

    let my_lobby = lobby.get(in_lobby.0).expect("Failed to get lobby");

    tank_transform.position = check_collision_and_apply_movement(
        &tank_transform,
        &next_tank_position,
        tank_config,
        &my_lobby
            .map_config
            .as_ref()
            .expect("Failed to get map config")
            .map,
    );
}

fn check_collision_and_apply_movement(
    current_transform: &TankTransform,
    target_position: &Vec3,
    tank_config: &TankConfig,
    map_definition: &MapDefinition,
) -> Vec3 {
    let max_slope = tank_config.max_slope;
    let half_extents = tank_config.size; // x/z: half extents, y: vertical offset (ignored for collision)

    let start_pos = current_transform.position;
    let move_vec = *target_position - start_pos;
    let distance = move_vec.length();

    // Define a resolution for sampling—finer steps catch even the feeblest obstacle.
    let step_size = 0.1; // Adjust as needed for precision
    let steps = (distance / step_size).ceil() as usize;
    let mut last_valid_position = start_pos;

    // Define the local corners of the tank's bounding box (relative to its center)
    let local_corners = [
        Vec3::new(half_extents.x, 0.0, half_extents.z),
        Vec3::new(-half_extents.x, 0.0, half_extents.z),
        Vec3::new(-half_extents.x, 0.0, -half_extents.z),
        Vec3::new(half_extents.x, 0.0, -half_extents.z),
    ];

    // Sample the path from the start to the target position.
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let candidate_pos = start_pos + move_vec * t;

        // Obtain the center tile (using the candidate's position)
        let center_tile = map_definition.get_closest_tile(candidate_pos);
        if center_tile.is_none() {
            // Out of bounds – an invisible wall stops your advance!
            break;
        }
        let center_tile_def = center_tile.unwrap();
        // Retrieve the floor height at the center tile.
        let center_floor =
            match map_definition.get_floor_height_of_tile(center_tile_def.x, center_tile_def.y) {
                Some(h) => h,
                None => break, // Shouldn't occur; treat as collision.
            };

        // Now, check each corner of the rotated bounding box.
        let mut collision = false;
        for local_corner in &local_corners {
            // Rotate the local corner to world space
            let rotated_corner = current_transform.rotation.mul_vec3(*local_corner);
            let world_corner = candidate_pos + rotated_corner;

            // Check the tile under this corner.
            let tile = map_definition.get_closest_tile(world_corner);
            if tile.is_none() {
                collision = true;
                break;
            }
            let tile_def = tile.unwrap();
            let tile_floor = match map_definition.get_floor_height_of_tile(tile_def.x, tile_def.y) {
                Some(h) => h,
                None => {
                    collision = true;
                    break;
                }
            };

            // If the slope is too steep relative to the center floor, declare a collision.
            if (tile_floor - center_floor).abs() > max_slope {
                collision = true;
                break;
            }
        }

        // If any corner collides, we cannot proceed further.
        if collision {
            break;
        }

        // Otherwise, the candidate position is still valid.
        last_valid_position = candidate_pos;
    }

    // Finally, adjust the Y coordinate of the valid position to match the terrain.
    if let Some(center_tile) = map_definition.get_closest_tile(last_valid_position) {
        if let Some(floor) = map_definition.get_floor_height_of_tile(center_tile.x, center_tile.y) {
            last_valid_position.y = floor;
        }
    }
    last_valid_position
}
