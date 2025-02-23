use bevy::prelude::*;
use shared::{
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        triggers::CollidedWithWorldTrigger,
    },
    networking::lobby_management::{InLobby, MyLobby},
};
use std::sync::Mutex;

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
    // Secure our dominion over this wretched lobby!
    let my_lobby_entity = trigger.entity();
    let my_lobby = lobby
        .get(my_lobby_entity)
        .expect("Failed to secure dominion—your lobby is pitiful!");
    let map_def = &my_lobby
        .map_config
        .as_ref()
        .expect("Map config is missing, you miserable wretch!")
        .map;

    const STEP_SIZE: f32 = 0.01;

    // A thread-safe hoard for entities that encounter collision misfortune.
    let collided_entities = Mutex::new(Vec::new());

    // Process each colliding minion in parallel—swift as a plague!
    colliders.par_iter_mut().for_each(
        |(entity, mut transform, mut wanted, collider, _layer, in_lobby)| {
            if in_lobby.0 != my_lobby_entity {
                return;
            }

            let current = *transform;
            let target = **wanted;
            let delta = target.translation - current.translation;
            let total_distance = delta.length();
            let steps = if total_distance == 0.0 {
                1
            } else {
                (total_distance / STEP_SIZE).ceil() as i32
            };

            let mut safe_translation = current.translation;
            let mut safe_rotation = current.rotation;
            let mut collision_happened = false;

            // Advance in meticulous steps, checking for collisions at each infernal increment.
            for step in 1..=steps {
                let t = step as f32 / steps as f32;
                let candidate_translation = current.translation.lerp(target.translation, t);
                let candidate_rotation = current.rotation.slerp(target.rotation, t);

                // Compute the rotated footprint via the right and forward vectors.
                let right = candidate_rotation * Vec3::X;
                let forward = candidate_rotation * Vec3::Z;
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

                // Determine the axis-aligned bounding box (AABB) of the unholy footprint.
                let (min_x, max_x) = corners
                    .iter()
                    .fold((f32::MAX, f32::MIN), |(min, max), corner| {
                        (min.min(corner.x), max.max(corner.x))
                    });
                let (min_z, max_z) = corners
                    .iter()
                    .fold((f32::MAX, f32::MIN), |(min, max), corner| {
                        (min.min(corner.z), max.max(corner.z))
                    });

                let tile_min_x = min_x.floor() as isize;
                let tile_max_x = max_x.ceil() as isize;
                let tile_min_z = min_z.floor() as isize;
                let tile_max_z = max_z.ceil() as isize;

                // First pass: collect all tile heights and ensure tiles exist within bounds.
                let mut tile_heights = Vec::new();
                let mut local_collision = false;
                for tx in tile_min_x..tile_max_x {
                    for tz in tile_min_z..tile_max_z {
                        if tx < 0
                            || tz < 0
                            || (tx as usize) >= map_def.width
                            || (tz as usize) >= map_def.depth
                        {
                            local_collision = true;
                            break;
                        }
                        match map_def.get_floor_height_of_tile((tx as usize, tz as usize)) {
                            Some(height) => tile_heights.push(height),
                            None => {
                                local_collision = true;
                                break;
                            }
                        }
                    }
                    if local_collision {
                        break;
                    }
                }
                if local_collision {
                    collision_happened = true;
                    break;
                }
                // Determine the highest floor among the tiles.
                let candidate_floor = tile_heights.iter().cloned().fold(f32::MIN, f32::max);

                // Second pass: verify that no tile deviates too steeply from the highest floor.
                if tile_heights
                    .iter()
                    .any(|&h| (candidate_floor - h).abs() > collider.max_slope)
                {
                    collision_happened = true;
                    break;
                }

                // This step is safe—record its blessed state.
                safe_translation = Vec3::new(
                    candidate_translation.x,
                    candidate_floor + collider.half_size.y,
                    candidate_translation.z,
                );
                safe_rotation = candidate_rotation;
            }

            // If our progress was thwarted by obstacles, mark this entity.
            if collision_happened {
                collided_entities.lock().unwrap().push(entity);
            }

            // Update the entity’s transform to its newly secured state.
            transform.translation = safe_translation;
            transform.rotation = safe_rotation;
            transform.scale = Vec3::ONE;
            **wanted = Transform {
                translation: safe_translation,
                rotation: safe_rotation,
                scale: Vec3::ONE,
            };
        },
    );

    // Unleash the wrath of collision triggers upon all doomed entities.
    commands.trigger_targets(
        CollidedWithWorldTrigger,
        collided_entities.into_inner().unwrap(),
    );
}
