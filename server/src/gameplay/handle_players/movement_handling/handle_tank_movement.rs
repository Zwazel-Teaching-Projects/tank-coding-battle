use bevy::prelude::*;
use shared::{
    asset_handling::{
        config::{TankConfig, TankConfigSystemParam},
        maps::MapDefinition,
    },
    game::tank_types::TankType,
    networking::{
        lobby_management::{InLobby, MyLobby},
        messages::message_container::MoveTankCommandTrigger,
    },
};

pub fn handle_tank_movement(
    trigger: Trigger<MoveTankCommandTrigger>,
    mut tank: Query<(&mut Transform, &TankType, &InLobby)>,
    tank_config: TankConfigSystemParam,
    lobby: Query<&MyLobby>,
) {
    let client_entity = trigger.entity();
    let (mut tank_transform, tank_type, in_lobby) = tank
        .get_mut(client_entity)
        .expect("Failed to get tank transform");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");
    let distance = tank_config.move_speed.min(trigger.distance);
    let move_direction = tank_transform.rotation * Vec3::new(0.0, 0.0, distance);
    let next_tank_position = tank_transform.translation + move_direction;

    let my_lobby = lobby.get(in_lobby.0).expect("Failed to get lobby");

    tank_transform.translation = check_collision_and_apply_movement(
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
    current_transform: &Transform,
    target_position: &Vec3,
    tank_config: &TankConfig,
    map_definition: &MapDefinition,
) -> Vec3 {
    // Extract configuration values.
    let max_slope = tank_config.max_slope;
    // For horizontal collision, use half extents from size.x and size.z.
    let half_width = tank_config.size.x;
    let half_depth = tank_config.size.z;
    // Vertical offset: the tank’s “base” height offset relative to the map floor.
    let vertical_offset = tank_config.size.y;

    // We'll start from the current position and step toward the target.
    let mut safe_position = current_transform.translation;
    let total_distance = (*target_position - current_transform.translation).length();
    // Define a small step size (adjust as needed).
    let step_size = 0.1;
    let steps = (total_distance / step_size).ceil() as i32;

    // We assume that the tank always moves forward in the direction it faces.
    // (If target_position is computed from input, then a linear interpolation works fine.)
    for i in 1..=steps {
        // Linear interpolation parameter.
        let t = i as f32 / steps as f32;
        // Candidate new position along the movement path.
        let candidate_horizontal = current_transform.translation.lerp(*target_position, t);

        // Compute the tank's rotated footprint.
        // We'll derive the right and forward vectors from the tank's rotation.
        // (Assuming that, in local space, forward is along +Z and right is along +X.)
        let forward = current_transform.rotation * Vec3::new(0.0, 0.0, 1.0);
        let right = current_transform.rotation * Vec3::new(1.0, 0.0, 0.0);

        // Compute the four corners of the tank's base in world space.
        let corners = [
            candidate_horizontal + right * half_width + forward * half_depth,
            candidate_horizontal - right * half_width + forward * half_depth,
            candidate_horizontal + right * half_width - forward * half_depth,
            candidate_horizontal - right * half_width - forward * half_depth,
        ];

        // Determine the axis-aligned bounding box (AABB) of these corners.
        let (mut min_x, mut max_x, mut min_z, mut max_z) =
            (std::f32::MAX, std::f32::MIN, std::f32::MAX, std::f32::MIN);
        for corner in &corners {
            if corner.x < min_x {
                min_x = corner.x;
            }
            if corner.x > max_x {
                max_x = corner.x;
            }
            if corner.z < min_z {
                min_z = corner.z;
            }
            if corner.z > max_z {
                max_z = corner.z;
            }
        }

        // Convert world coordinates to tile indices.
        let tile_min_x = min_x.floor() as isize;
        let tile_max_x = max_x.ceil() as isize;
        let tile_min_z = min_z.floor() as isize;
        let tile_max_z = max_z.ceil() as isize;

        let mut collision = false;
        // We'll also compute the candidate "floor" height as the maximum tile height
        // beneath the tank's footprint—this simulates the tank climbing up.
        let mut candidate_floor = std::f32::MIN;

        // Iterate over all tiles covered by the AABB.
        for tx in tile_min_x..tile_max_x {
            for tz in tile_min_z..tile_max_z {
                // Out-of-bounds is treated as a collision (an invisible wall).
                if tx < 0
                    || tz < 0
                    || (tx as usize) >= map_definition.width
                    || (tz as usize) >= map_definition.depth
                {
                    collision = true;
                    break;
                }
                if let Some(tile_height) =
                    map_definition.get_floor_height_of_tile((tx as usize, tz as usize))
                {
                    // Update candidate floor: use the highest floor value beneath the tank.
                    candidate_floor = candidate_floor.max(tile_height);
                } else {
                    collision = true;
                    break;
                }
            }
            if collision {
                break;
            }
        }

        // If any tile is out-of-bounds, we have a collision.
        if collision {
            break;
        }

        // Check that the terrain variation under the tank is not too steep.
        // In other words, for every tile beneath the tank, the difference between
        // the candidate floor (max height) and the tile's height must be ≤ max_slope.
        for tx in tile_min_x..tile_max_x {
            for tz in tile_min_z..tile_max_z {
                if let Some(tile_height) =
                    map_definition.get_floor_height_of_tile((tx as usize, tz as usize))
                {
                    if (candidate_floor - tile_height).abs() > max_slope {
                        collision = true;
                        break;
                    }
                }
            }
            if collision {
                break;
            }
        }

        // If a collision is detected at this step, stop moving further.
        if collision {
            break;
        }

        // Otherwise, update the safe position.
        // We set the tank's y coordinate so that its base sits at candidate_floor plus vertical_offset.
        safe_position = Vec3::new(
            candidate_horizontal.x,
            candidate_floor + vertical_offset,
            candidate_horizontal.z,
        );
    }

    safe_position
}
