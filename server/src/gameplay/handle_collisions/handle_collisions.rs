use std::sync::Mutex;

use bevy::prelude::*;
use shared::{
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        triggers::CollidedWithWorldTrigger,
    },
    networking::lobby_management::{InLobby, MyLobby},
};

use crate::gameplay::triggers::FinishedNextSimulationStepTrigger;

pub fn check_collision_and_apply_movement(
    trigger: Trigger<FinishedNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    mut colliders: Query<
        (
            Entity,
            &mut Transform,
            &mut WantedTransform,
            &Collider,
            &CollisionLayer,
            &InLobby,
        ),
        Changed<WantedTransform>,
    >,
    mut commands: Commands,
) {
    // Claim the domain of our lobby—only entities in this foul hovel are processed!
    let my_lobby_entity = trigger.entity();
    let my_lobby = lobby
        .get(my_lobby_entity)
        .expect("Failed to secure our dominion—your lobby is weak!");
    let map_definition = &my_lobby
        .map_config
        .as_ref()
        .expect("The map config is missing, you miserable wretch!")
        .map;

    const STEP_SIZE: f32 = 0.01; // Our infernal step distance.

    // For each colliding minion under our command...
    let collided_entities = Mutex::new(Vec::new());
    colliders.par_iter_mut().for_each(
        |(entity, mut transform, mut wanted, collider, _collision_layer, in_lobby)| {
            // Only process those within our accursed lobby.
            if in_lobby.0 != my_lobby_entity {
                return;
            }

            let current = *transform;
            let target = **wanted;
            let delta = target.translation - current.translation;
            let total_distance = delta.length();

            // Determine the number of steps required for our meticulous progress.
            let steps = if total_distance == 0.0 {
                1
            } else {
                (total_distance / STEP_SIZE).ceil() as i32
            };

            // Our safe haven begins at the current transform.
            let mut safe_translation = current.translation;
            let mut safe_rotation = current.rotation;

            let mut collision_happened = false; // Track if any collision occurs

            for step in 1..=steps {
                let t = step as f32 / steps as f32;
                let candidate_translation = current.translation.lerp(target.translation, t);
                let candidate_rotation = current.rotation.slerp(target.rotation, t);

                // Compute the collider's rotated footprint using its half extents.
                let right = candidate_rotation * Vec3::new(1.0, 0.0, 0.0);
                let forward = candidate_rotation * Vec3::new(0.0, 0.0, 1.0);

                let corners = [
                    candidate_translation
                        + right * collider.half_size.x
                        + forward * collider.half_size.z,
                    candidate_translation - right * collider.half_size.x
                        + forward * collider.half_size.z,
                    candidate_translation + right * collider.half_size.x
                        - forward * collider.half_size.z,
                    candidate_translation
                        - right * collider.half_size.x
                        - forward * collider.half_size.z,
                ];

                // Determine the axis-aligned bounding box of this unholy footprint.
                let mut min_x = f32::MAX;
                let mut max_x = f32::MIN;
                let mut min_z = f32::MAX;
                let mut max_z = f32::MIN;
                for corner in &corners {
                    min_x = min_x.min(corner.x);
                    max_x = max_x.max(corner.x);
                    min_z = min_z.min(corner.z);
                    max_z = max_z.max(corner.z);
                }

                let tile_min_x = min_x.floor() as isize;
                let tile_max_x = max_x.ceil() as isize;
                let tile_min_z = min_z.floor() as isize;
                let tile_max_z = max_z.ceil() as isize;

                let mut collision = false;
                let mut candidate_floor = f32::MIN;

                // Probe the map for out-of-bounds or missing tiles.
                for tx in tile_min_x..tile_max_x {
                    for tz in tile_min_z..tile_max_z {
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

                if collision {
                    collision_happened = true;
                    break;
                }

                // Verify that the terrain's slope is not too steep.
                for tx in tile_min_x..tile_max_x {
                    for tz in tile_min_z..tile_max_z {
                        if let Some(tile_height) =
                            map_definition.get_floor_height_of_tile((tx as usize, tz as usize))
                        {
                            if (candidate_floor - tile_height).abs() > collider.max_slope {
                                collision = true;
                                break;
                            }
                        }
                    }
                    if collision {
                        break;
                    }
                }

                if collision {
                    collision_happened = true;
                    break;
                }

                // Instead of forcing our minion to the floor, check if they are trying to fly.
                // If they dip below the floor, that's a collision—otherwise, let them soar!
                if candidate_translation.y < candidate_floor + collider.half_size.y {
                    collision_happened = true;
                    break;
                }

                // Record this step as safe—preserve their chosen altitude!
                safe_translation = candidate_translation;
                safe_rotation = candidate_rotation;
            }

            // Print once if a collision was detected.
            if collision_happened {
                collided_entities.lock().unwrap().push(entity);
            }

            // Command the entity's transform to our calculated, secure state.
            transform.translation = safe_translation;
            transform.rotation = safe_rotation;
            transform.scale = Vec3::ONE;

            // Optionally, update the wanted transform to mirror our progress.
            **wanted = Transform {
                translation: safe_translation,
                rotation: safe_rotation,
                scale: Vec3::ONE,
            };

            // Mwahaha! Revel in the chaos as obstacles crumble beneath our calculated advance.
        },
    );

    // If any collisions were detected
    commands.trigger_targets(
        CollidedWithWorldTrigger,
        collided_entities.into_inner().unwrap(),
    );
}
