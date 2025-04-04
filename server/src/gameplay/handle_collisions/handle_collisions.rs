use bevy::prelude::*;
use shared::{
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        structs::Obb3d,
        triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
    },
    networking::lobby_management::{InLobby, MyLobby},
};

use crate::gameplay::triggers::{CheckForCollisionsTrigger, DespawnOutOfBoundsProjectilesTrigger};

const STEP_SIZE: f32 = 0.05;

pub fn unified_collision_system(
    trigger: Trigger<CheckForCollisionsTrigger>,
    lobby: Query<&MyLobby>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Transform,
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
    // Secure dominion over this wretched lobby.
    let my_lobby_entity = trigger.entity();
    let my_lobby = lobby
        .get(my_lobby_entity)
        .expect("Failed to secure dominion over your pitiful lobby!");
    let map_def = &my_lobby
        .map_config
        .as_ref()
        .expect("Map config is missing, you miserable wretch!")
        .map;

    #[cfg(feature = "debug")]
    {
        for (mut gizmos, in_lobby) in debug_obb_gizmos.iter_mut() {
            if in_lobby.0 != my_lobby_entity {
                continue;
            }
            gizmos.clear();
        }
    }

    // Structure to hold our simulation data for each entity.
    struct SimEntity {
        entity: Entity,
        original: Transform,
        wanted: Transform,
        collider: Collider,
        collision_layer: CollisionLayer,
        // The time (from 0.0 to 1.0) at which a world collision occurs; if None, no world collision happens.
        world_collision_time: Option<f32>,
        // The last safe transform before a world collision.
        world_safe: Transform,
        // The earliest entity collision time (default 1.0 means no collision).
        entity_collision_time: f32,
        // New: true if this entity's collision layer is NO_COLLISION.
        no_collision: bool,
    }

    let mut sim_entities = Vec::new();
    for (entity, transform, wanted, collider, collision_layer, in_lobby) in query.iter_mut() {
        if in_lobby.0 != my_lobby_entity {
            continue;
        }

        sim_entities.push(SimEntity {
            entity,
            original: *transform,
            wanted: **wanted,
            collider: collider.clone(),
            collision_layer: collision_layer.clone(),
            world_collision_time: None,
            world_safe: *transform,
            entity_collision_time: 1.0,
            no_collision: collision_layer.contains(CollisionLayer::NO_COLLISION),
        });
    }

    // Determine the number of discrete steps for our simulation.
    let n_steps = (1.0 / STEP_SIZE).ceil() as usize;

    // === Phase 1: World Collision Check ===
    for sim in sim_entities.iter_mut() {
        if sim.no_collision {
            sim.world_collision_time = Some(1.0);
            sim.world_safe = sim.wanted;
            continue;
        }

        let delta = sim.wanted.translation - sim.original.translation;
        let total_distance = delta.length();
        let steps = if total_distance == 0.0 {
            1
        } else {
            (total_distance / STEP_SIZE).ceil() as i32
        };
        let mut safe_transform = sim.original;
        for step in 1..=steps {
            let t = step as f32 / steps as f32;
            let candidate = interpolate_transform(&sim.original, &sim.wanted, t);
            // Compute corners of the candidate's footprint.
            let right = candidate.rotation.mul_vec3(Vec3::X);
            let forward = candidate.rotation.mul_vec3(Vec3::Z);
            let corners = [
                candidate.translation
                    + right * sim.collider.half_size.x
                    + forward * sim.collider.half_size.z,
                candidate.translation - right * sim.collider.half_size.x
                    + forward * sim.collider.half_size.z,
                candidate.translation + right * sim.collider.half_size.x
                    - forward * sim.collider.half_size.z,
                candidate.translation
                    - right * sim.collider.half_size.x
                    - forward * sim.collider.half_size.z,
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
            'tile_loop: for tx in tile_min_x..tile_max_x {
                for tz in tile_min_z..tile_max_z {
                    if tx < 0
                        || tz < 0
                        || (tx as usize) >= map_def.width
                        || (tz as usize) >= map_def.depth
                    {
                        local_collision = true;
                        break 'tile_loop;
                    }
                    match map_def.get_floor_height_of_tile((tx as usize, tz as usize)) {
                        Some(height) => tile_heights.push(height),
                        None => {
                            local_collision = true;
                            break 'tile_loop;
                        }
                    }
                }
            }
            if local_collision {
                sim.world_collision_time = Some(t);
                break;
            }
            let candidate_floor = tile_heights.clone().into_iter().fold(f32::MIN, f32::max);
            if sim.collider.max_slope == 0.0 {
                if candidate.translation.y < candidate_floor + sim.collider.half_size.y {
                    sim.world_collision_time = Some(t);
                    break;
                }
                safe_transform = candidate;
            } else {
                // Compute the maximum and minimum floor heights within the footprint.
                let candidate_max_floor = tile_heights.iter().cloned().fold(f32::MIN, f32::max);
                let candidate_min_floor = tile_heights.iter().cloned().fold(f32::MAX, f32::min);

                // If the slope (difference between max and min) is too steep, declare collision.
                if candidate_max_floor - candidate_min_floor > sim.collider.max_slope {
                    sim.world_collision_time = Some(t);
                    break;
                }

                // Otherwise, climb: update y to the maximum floor height plus half the collider’s height.
                safe_transform = Transform {
                    translation: Vec3::new(
                        candidate.translation.x,
                        candidate_max_floor,
                        candidate.translation.z,
                    ),
                    rotation: candidate.rotation,
                    scale: candidate.scale,
                };
            }
        }
        sim.world_safe = safe_transform;
    }

    // === Phase 2: Entity Collision Check ===
    // We record collisions as (entity A, entity B, collision time).
    let mut collision_events: Vec<(Entity, Entity, f32)> = Vec::new();
    for step in 1..=n_steps {
        let t = step as f32 * STEP_SIZE;
        // Compute candidate transform for each entity based on its world collision time.
        let candidates: Vec<Transform> = sim_entities
            .iter()
            .map(|sim| {
                let world_t = sim.world_collision_time.unwrap_or(1.0);
                if t <= world_t {
                    interpolate_transform(&sim.original, &sim.wanted, t)
                } else {
                    sim.world_safe
                }
            })
            .collect();

        // Check each pair for collision.
        for i in 0..sim_entities.len() {
            for j in (i + 1)..sim_entities.len() {
                if sim_entities[i].no_collision || sim_entities[j].no_collision {
                    continue;
                }

                // Skip if collision layers do not intersect or if either ignores the other.
                if !sim_entities[i]
                    .collision_layer
                    .intersects(&sim_entities[j].collision_layer)
                {
                    continue;
                }
                if sim_entities[i]
                    .collision_layer
                    .ignore
                    .contains(&sim_entities[j].entity)
                    || sim_entities[j]
                        .collision_layer
                        .ignore
                        .contains(&sim_entities[i].entity)
                {
                    continue;
                }

                let obb_i = Obb3d::from_transform(&candidates[i], &sim_entities[i].collider);
                let obb_j = Obb3d::from_transform(&candidates[j], &sim_entities[j].collider);

                #[cfg(feature = "debug")]
                {
                    if let Ok((mut gizmos, _)) = debug_obb_gizmos.get_mut(sim_entities[i].entity) {
                        gizmos.push((t, obb_i.clone()));
                    }
                    if let Ok((mut gizmos, _)) = debug_obb_gizmos.get_mut(sim_entities[j].entity) {
                        gizmos.push((t, obb_j.clone()));
                    }
                }

                if obb_i.intersects_obb(&obb_j) {
                    // Record collision if this is the earliest encounter.
                    if t < sim_entities[i].entity_collision_time
                        || t < sim_entities[j].entity_collision_time
                    {
                        sim_entities[i].entity_collision_time =
                            sim_entities[i].entity_collision_time.min(t);
                        sim_entities[j].entity_collision_time =
                            sim_entities[j].entity_collision_time.min(t);
                        collision_events.push((sim_entities[i].entity, sim_entities[j].entity, t));
                    }
                }
            }
        }
    }

    // === Phase 3: Finalize Transforms and Trigger Events ===
    for sim in sim_entities.iter() {
        let world_t = sim.world_collision_time.unwrap_or(1.0);
        let effective_t = world_t.min(sim.entity_collision_time);

        let final_transform = if sim.no_collision {
            sim.wanted
        } else if effective_t < 1.0 {
            // A collision occurred—do not climb beyond what was safe.
            sim.world_safe
        } else {
            // No collision: full movement is safe.
            // If climbing is allowed, update the y component from safe transform.
            if sim.collider.max_slope > 0.0 {
                Transform {
                    translation: Vec3::new(
                        sim.wanted.translation.x,
                        sim.world_safe.translation.y,
                        sim.wanted.translation.z,
                    ),
                    rotation: sim.wanted.rotation,
                    scale: sim.wanted.scale,
                }
            } else {
                sim.wanted
            }
        };

        commands.entity(sim.entity).insert(final_transform);
        commands
            .entity(sim.entity)
            .insert(WantedTransform(final_transform));
    }

    // Trigger collision events.
    // World collisions: if an entity's world collision time is the earliest, trigger the world collision event.
    for sim in sim_entities.iter() {
        let world_t = sim.world_collision_time.unwrap_or(1.0);
        if world_t <= sim.entity_collision_time && world_t < 1.0 {
            commands.trigger_targets(CollidedWithWorldTrigger, sim.entity);
        }
    }
    // Entity collisions: trigger only if this collision is the earliest for both participants.
    for (entity_a, entity_b, t) in collision_events.into_iter() {
        let eff_a = sim_entities
            .iter()
            .find(|s| s.entity == entity_a)
            .map(|s| {
                s.world_collision_time
                    .unwrap_or(1.0)
                    .min(s.entity_collision_time)
            })
            .unwrap_or(1.0);
        let eff_b = sim_entities
            .iter()
            .find(|s| s.entity == entity_b)
            .map(|s| {
                s.world_collision_time
                    .unwrap_or(1.0)
                    .min(s.entity_collision_time)
            })
            .unwrap_or(1.0);
        if (eff_a - t).abs() < f32::EPSILON && (eff_b - t).abs() < f32::EPSILON && t < 1.0 {
            commands.trigger_targets(CollidedWithTrigger { entity: entity_b }, entity_a);
            commands.trigger_targets(CollidedWithTrigger { entity: entity_a }, entity_b);
        }
    }

    // Despawn out-of-bounds projectiles.
    commands.trigger_targets(DespawnOutOfBoundsProjectilesTrigger, my_lobby_entity);
}

/// A simple interpolation between two transforms over time t (0.0 to 1.0).
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
