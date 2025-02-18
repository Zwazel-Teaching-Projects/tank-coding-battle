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
        &tank_transform.position,
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
    current_position: &Vec3,
    target_position: &Vec3,
    tank_config: &TankConfig,
    map_definition: &MapDefinition,
) -> Vec3 {
    let max_slope = tank_config.max_slope;
    let tank_size = tank_config.size; // x/z: half extents, y: vertical offset

    // Retrieve current tile and its floor height.
    let current_tile = match map_definition.get_closest_tile(*current_position) {
        Some(tile) => tile,
        None => return *current_position, // Out-of-bounds? The tank stays put.
    };
    let (curr_tile_x, curr_tile_y) = current_tile.into();
    let current_floor = map_definition
        .get_floor_height_of_tile(curr_tile_x, curr_tile_y)
        .expect(&format!(
            "Failed to get floor height of tile ({}, {})",
            curr_tile_x, curr_tile_y
        ));

    // Calculate movement delta.
    let delta = *target_position - *current_position;
    let dx = delta.x;
    let dz = delta.z;

    // Helper to compute allowed movement factor.
    let compute_t = |current: f32, delta: f32, bound: f32| -> f32 {
        if delta.abs() < std::f32::EPSILON {
            1.0
        } else if delta > 0.0 {
            (bound - current) / delta
        } else {
            (current - bound) / (-delta)
        }
    };

    // If the target tile exists and its floor is known...
    if let Some(target_tile) = map_definition.get_closest_tile(*target_position) {
        let (target_tile_x, target_tile_y) = target_tile.into();
        if let Some(target_floor) =
            map_definition.get_floor_height_of_tile(target_tile_x, target_tile_y)
        {
            // Is the slope too steep for our war machine?
            if target_floor > current_floor && (target_floor - current_floor) > max_slope {
                let t_x = if dx > 0.0 {
                    compute_t(
                        current_position.x,
                        dx,
                        curr_tile_x as f32 + 1.0 - tank_size.x,
                    )
                } else if dx < 0.0 {
                    compute_t(current_position.x, dx, curr_tile_x as f32 + tank_size.x)
                } else {
                    1.0
                };

                let t_z = if dz > 0.0 {
                    compute_t(
                        current_position.z,
                        dz,
                        curr_tile_y as f32 + 1.0 - tank_size.z,
                    )
                } else if dz < 0.0 {
                    compute_t(current_position.z, dz, curr_tile_y as f32 + tank_size.z)
                } else {
                    1.0
                };

                let t_allowed = t_x.min(t_z).min(1.0);
                let allowed_position = *current_position + delta * t_allowed;
                return Vec3::new(
                    allowed_position.x,
                    current_floor + tank_size.y,
                    allowed_position.z,
                );
            } else {
                // Slope is acceptable – proceed with the full advance.
                return Vec3::new(
                    target_position.x,
                    target_floor + tank_size.y,
                    target_position.z,
                );
            }
        }
    }

    // If the target tile is beyond our dominion, clamp movement to the map's borders.
    let map_width = map_definition.width as f32;
    let map_height = map_definition.height as f32;

    let t_x = if dx > 0.0 {
        compute_t(current_position.x, dx, map_width - tank_size.x)
    } else if dx < 0.0 {
        compute_t(current_position.x, dx, tank_size.x)
    } else {
        1.0
    };

    let t_z = if dz > 0.0 {
        compute_t(current_position.z, dz, map_height - tank_size.z)
    } else if dz < 0.0 {
        compute_t(current_position.z, dz, tank_size.z)
    } else {
        1.0
    };

    let t_allowed = t_x.min(t_z).min(1.0);
    let allowed_position = *current_position + delta * t_allowed;
    Vec3::new(
        allowed_position.x,
        current_floor + tank_size.y,
        allowed_position.z,
    )
}
