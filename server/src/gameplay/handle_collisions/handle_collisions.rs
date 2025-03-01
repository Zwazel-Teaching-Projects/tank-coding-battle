use bevy::prelude::*;
use shared::{
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        structs::Obb3d,
        triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
    },
    networking::lobby_management::{InLobby, MyLobby},
};
use std::sync::Mutex;

use crate::gameplay::triggers::{CalculateCollisionsTrigger, FinishedNextSimulationStepTrigger};

const STEP_SIZE: f32 = 0.05;

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
            &Transform,
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

    // A thread-safe hoard for entities that encounter collision misfortune.
    let collided_entities = Mutex::new(Vec::new());

    // --- World Collision Check ---
    colliders.par_iter_mut().for_each(
        |(entity, current_transform, mut wanted_transform, collider, _layer, in_lobby)| {
            if in_lobby.0 != my_lobby_entity {
                return;
            }

            let current = *current_transform;
            let target = **wanted_transform;
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
                let candidate_transform = Transform {
                    translation: candidate_translation,
                    rotation: candidate_rotation,
                    ..default()
                };

                let right = candidate_transform.right();
                let forward = candidate_transform.forward();
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

            wanted_transform.translation = safe_translation;
            wanted_transform.rotation = safe_rotation;
        },
    );

    commands.trigger_targets(
        CollidedWithWorldTrigger,
        collided_entities.into_inner().unwrap(),
    );
    commands.trigger_targets(CalculateCollisionsTrigger, my_lobby_entity);
}

pub fn collision_system(
    trigger: Trigger<CalculateCollisionsTrigger>,
    mut commands: Commands,
    mut combinations: Query<(
        Entity,
        &mut Transform,
        &mut WantedTransform,
        &Collider,
        &CollisionLayer,
        &InLobby,
    )>,
    #[cfg(feature = "debug")] mut debug_obb_gizmos: Query<(
        &mut debug::DebugColliderComponent,
        &InLobby,
    )>,
) {
    let my_lobby_entity = trigger.entity();
    let steps = (1.0 / STEP_SIZE) as usize;

    #[cfg(feature = "debug")]
    {
        for (mut gizmos, in_lobby) in debug_obb_gizmos.iter_mut() {
            if in_lobby.0 != my_lobby_entity {
                continue;
            }
            gizmos.clear();
        }
    }

    let mut combinations_iter = combinations.iter_combinations_mut();
    while let Some(
        [(a_entity, mut a_transform, mut a_wanted, a_collider, a_collision_layer, a_in_lobby), (b_entity, mut b_transform, mut b_wanted, b_collider, b_collision_layer, b_in_lobby)],
    ) = combinations_iter.fetch_next()
    {
        // Existing lobby and collision layer checks...
        if a_in_lobby.0 != my_lobby_entity || b_in_lobby.0 != my_lobby_entity {
            continue;
        }
        if !a_collision_layer.intersects(b_collision_layer) {
            continue;
        }
        if a_collision_layer.ignore.contains(&b_entity)
            || b_collision_layer.ignore.contains(&a_entity)
        {
            continue;
        }

        // Calculate movement trajectories
        let mut t_collision = None;
        let mut a_safe_transform = *a_transform;
        let mut b_safe_transform = *b_transform;
        for step in 1..=steps {
            let t = step as f32 * STEP_SIZE;
            let a_safe = interpolate_transform(&*a_transform, &a_wanted.0, t);
            let b_safe = interpolate_transform(&*b_transform, &b_wanted.0, t);

            let a_obb = Obb3d::from_transform(&a_safe, a_collider);
            let b_obb = Obb3d::from_transform(&b_safe, b_collider);

            #[cfg(feature = "debug")]
            {
                let (mut a_gizmos, _) = debug_obb_gizmos
                    .get_mut(a_entity)
                    .expect("Failed to get debug collider");
                a_gizmos.push((t, a_obb));

                let (mut b_gizmos, _) = debug_obb_gizmos
                    .get_mut(b_entity)
                    .expect("Failed to get debug collider");
                b_gizmos.push((t, b_obb));
            }

            if a_obb.intersects_obb(&b_obb) {
                t_collision = Some(t);
                break;
            } else {
                a_safe_transform = a_safe;
                b_safe_transform = b_safe;
            }
        }

        *a_transform = a_safe_transform;
        a_wanted.0 = a_safe_transform;

        *b_transform = b_safe_transform;
        b_wanted.0 = b_safe_transform;

        if let Some(_t) = t_collision {
            // Trigger events...
            commands.trigger_targets(CollidedWithTrigger { entity: b_entity }, a_entity);
            commands.trigger_targets(CollidedWithTrigger { entity: a_entity }, b_entity);
        }
    }
}

fn interpolate_transform(start: &Transform, end: &Transform, t: f32) -> Transform {
    Transform {
        translation: start.translation.lerp(end.translation, t),
        rotation: start.rotation.slerp(end.rotation, t),
        scale: start.scale.lerp(end.scale, t),
    }
}

#[cfg(feature = "debug")]
pub mod debug {
    use bevy::{math::Vec3A, prelude::*};
    use shared::game::collision_handling::{components::Collider, structs::Obb3d};

    pub struct CollisionDebugPlugin;

    impl Plugin for CollisionDebugPlugin {
        fn build(&self, app: &mut App) {
            app.register_type::<DebugColliderComponent>()
                .add_systems(Update, visualize_obb3ds)
                .add_observer(insert_debug_collider);
        }
    }

    #[derive(Default, Component, Reflect, Debug, Deref, DerefMut)]
    #[reflect(Component)]
    pub struct DebugColliderComponent(pub Vec<(f32, Obb3d)>);

    fn visualize_obb3ds(mut gizmos: Gizmos, mut obb_gizmos: Query<&mut DebugColliderComponent>) {
        for debug_obb in obb_gizmos.iter_mut() {
            if debug_obb.is_empty() {
                continue;
            }

            let (max_step, _) = debug_obb
                .iter()
                .max_by(|(step_a, _), (step_b, _)| step_a.partial_cmp(step_b).unwrap())
                .unwrap();

            let (min_step, _) = debug_obb
                .iter()
                .min_by(|(step_a, _), (step_b, _)| step_a.partial_cmp(step_b).unwrap())
                .unwrap();

            let step_range = max_step - min_step;

            for (step, obb) in debug_obb.iter() {
                let t = (step - min_step) / step_range;
                let color = Color::srgba(1.0 - t, t, 0.0, 1.0);

                let obb = Obb3d {
                    half_size: obb.half_size + Vec3A::splat(0.01),
                    ..*obb
                };

                gizmos.primitive_3d(
                    &Cuboid {
                        half_size: obb.half_size.into(),
                    },
                    Isometry3d::new(obb.center, Quat::from_mat3a(&obb.basis)),
                    color,
                );
            }
        }
    }

    fn insert_debug_collider(trigger: Trigger<OnAdd, Collider>, mut commands: Commands) {
        commands
            .entity(trigger.entity())
            .insert(DebugColliderComponent(Vec::new()));
    }
}
