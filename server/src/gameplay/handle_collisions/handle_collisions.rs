use bevy::prelude::*;
use shared::{
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
    },
    networking::lobby_management::{InLobby, MyLobby},
};
use std::sync::Mutex;

use crate::gameplay::triggers::{CalculateCollisionsTrigger, FinishedNextSimulationStepTrigger};

/// Warlock Engineer Ikit Claw’s masterful collision and movement enactor!
///
/// This function governs the motion of our pitiful minions, checking for collisions as they move
/// from their current location to a desired destination. For each entity with a changed WantedTransform,
/// it simulates movement in small, precise increments (STEP_SIZE), computing a rotated “footprint” at each step
/// to determine its axis-aligned bounding box (AABB) over the game map’s tiles.
///
/// The function first gathers the floor heights of all tiles under the footprint, ensuring that each tile exists
/// within the map’s bounds. It then verifies that the slope between the highest tile and its neighbors does not
/// exceed the collider’s maximum allowed slope.
///
/// A special case is observed when the collider’s max_slope is 0.0—this denotes a flying entity, not meant to climb.
/// In such cases, the vertical component of the candidate translation is not adjusted to match the floor height.
/// Instead, if the candidate translation would dip below the calculated floor (plus the collider’s half height),
/// a collision is triggered, ensuring that our aerial minions remain unburdened by the ground’s wretched grasp.
///
/// Upon detecting any collision, the entity is marked, and its transform is updated accordingly. Finally, the function
/// dispatches collision triggers to deal with the unfortunate souls that encountered obstacles.
/// Warlock Engineer Ikit Claw’s masterful collision and movement enactor!
pub fn check_world_collision_and_apply_movement(
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

    // --- World Collision Check ---
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

            // Advance in meticulous steps, checking for collisions with the world.
            for step in 1..=steps {
                let t = step as f32 / steps as f32;
                let candidate_translation = current.translation.lerp(target.translation, t);
                let candidate_rotation = current.rotation.slerp(target.rotation, t);

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
                let candidate_floor = tile_heights.iter().cloned().fold(f32::MIN, f32::max);

                if collider.max_slope == 0.0 {
                    if candidate_translation.y < candidate_floor + collider.half_size.y {
                        collision_happened = true;
                        break;
                    }
                    safe_translation = candidate_translation;
                    safe_rotation = candidate_rotation;
                } else {
                    if tile_heights
                        .iter()
                        .any(|&h| (candidate_floor - h).abs() > collider.max_slope)
                    {
                        collision_happened = true;
                        break;
                    }
                    safe_translation = Vec3::new(
                        candidate_translation.x,
                        candidate_floor + collider.half_size.y,
                        candidate_translation.z,
                    );
                    safe_rotation = candidate_rotation;
                }
            }

            if collision_happened {
                collided_entities.lock().unwrap().push(entity);
            }

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

    commands.trigger_targets(
        CollidedWithWorldTrigger,
        collided_entities.into_inner().unwrap(),
    );
    commands.trigger_targets(CalculateCollisionsTrigger, my_lobby_entity);
}

pub fn detect_pairwise_collisions(
    trigger: Trigger<CalculateCollisionsTrigger>,
    mut all_colliders: Query<(Entity, &Transform, &Collider, &CollisionLayer, &InLobby)>,
    mut commands: Commands,
) {
    let my_lobby_entity = trigger.entity();

    // --- Pairwise Collider Collision Check ---
    // Define helper closures for AABB computation and intersection test.
    let compute_aabb = |transform: &Transform, collider: &Collider| -> (f32, f32, f32, f32) {
        let translation = transform.translation;
        let rotation = transform.rotation;
        let right = rotation * Vec3::X;
        let forward = rotation * Vec3::Z;
        let half = collider.half_size;
        let corners = [
            translation + right * half.x + forward * half.z,
            translation - right * half.x + forward * half.z,
            translation + right * half.x - forward * half.z,
            translation - right * half.x - forward * half.z,
        ];
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
        (min_x, max_x, min_z, max_z)
    };

    let aabb_overlap = |a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)| -> bool {
        a.0 <= b.1 && a.1 >= b.0 && a.2 <= b.3 && a.3 >= b.2
    };

    // Iterate through all combinations of colliders in the lobby.
    let mut combinations = all_colliders.iter_combinations_mut::<2>();
    while let Some(
        [(entity_a, transform_a, collider_a, layer_a, in_lobby_a), (entity_b, transform_b, collider_b, layer_b, in_lobby_b)],
    ) = combinations.fetch_next()
    {
        if in_lobby_a.0 != my_lobby_entity || in_lobby_b.0 != my_lobby_entity {
            continue;
        }
        // Check collision layer intersections and ignore lists.
        if !layer_a.intersects(layer_b) {
            continue;
        }
        if layer_a.ignore.contains(&entity_b) || layer_b.ignore.contains(&entity_a) {
            continue;
        }
        // Compute AABBs.
        let aabb_a = compute_aabb(transform_a, collider_a);
        let aabb_b = compute_aabb(transform_b, collider_b);
        if aabb_overlap(aabb_a, aabb_b) {
            // Dispatch collision triggers for each entity.
            commands.trigger_targets(CollidedWithTrigger { entity: entity_b }, entity_a);
            commands.trigger_targets(CollidedWithTrigger { entity: entity_a }, entity_b);
        }
    }
}
