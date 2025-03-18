use bevy::prelude::*;
use shared::{
    asset_handling::maps::MapDefinition,
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        structs::Obb3d,
        triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
    },
    networking::lobby_management::{InLobby, MyLobby},
};

use crate::gameplay::triggers::{CheckForCollisionsTrigger, DespawnOutOfBoundsProjectilesTrigger};

// How many sub-steps we take for partial rotation/translation checks:
const SUB_STEPS: usize = 10;

// A small helper to interpolate rotation and translation separately.
fn interpolate_transform_separate(
    original: &Transform,
    wanted: &Transform,
    rot_t: f32,
    move_t: f32,
) -> Transform {
    // Slerp rotation
    let rot = original.rotation.slerp(wanted.rotation, rot_t);

    // Lerp translation
    let translation = original.translation + (wanted.translation - original.translation) * move_t;

    Transform {
        translation,
        rotation: rot,
        scale: original.scale, // or wanted.scale if you prefer
    }
}

/// Checks collision against the world (tile bounds, slopes, etc.).
/// Returns `true` if `candidate` collides, `false` otherwise.
fn check_world_collision(
    candidate: &Transform,
    collider: &Collider,
    map_def: &MapDefinition, // Adapt to your real map or pass what you need.
) -> bool {
    let right = candidate.rotation.mul_vec3(Vec3::X);
    let forward = candidate.rotation.mul_vec3(Vec3::Z);

    // Compute bounding corners in XZ plane.
    let corners = [
        candidate.translation + right * collider.half_size.x + forward * collider.half_size.z,
        candidate.translation - right * collider.half_size.x + forward * collider.half_size.z,
        candidate.translation + right * collider.half_size.x - forward * collider.half_size.z,
        candidate.translation - right * collider.half_size.x - forward * collider.half_size.z,
    ];

    let (min_x, max_x) = corners.iter().fold((f32::MAX, f32::MIN), |(mn, mx), c| {
        (mn.min(c.x), mx.max(c.x))
    });
    let (min_z, max_z) = corners.iter().fold((f32::MAX, f32::MIN), |(mn, mx), c| {
        (mn.min(c.z), mx.max(c.z))
    });

    let tile_min_x = min_x.floor() as isize;
    let tile_max_x = max_x.ceil() as isize;
    let tile_min_z = min_z.floor() as isize;
    let tile_max_z = max_z.ceil() as isize;

    // Gather tile heights; check out-of-bounds or impassable tiles.
    let mut tile_heights = Vec::new();
    for tx in tile_min_x..tile_max_x {
        for tz in tile_min_z..tile_max_z {
            if tx < 0 || tz < 0 || (tx as usize) >= map_def.width || (tz as usize) >= map_def.depth
            {
                // Out of map bounds => collision
                return true;
            }
            match map_def.get_floor_height_of_tile((tx as usize, tz as usize)) {
                Some(height) => tile_heights.push(height),
                None => {
                    // Impassable tile => collision
                    return true;
                }
            }
        }
    }

    // Determine highest and lowest floor in footprint.
    let candidate_max_floor = tile_heights.iter().cloned().fold(f32::MIN, f32::max);
    let candidate_min_floor = tile_heights.iter().cloned().fold(f32::MAX, f32::min);

    // If no climbing is allowed (max_slope == 0), block if below top of floor.
    if collider.max_slope == 0.0 {
        if candidate.translation.y < candidate_max_floor + collider.half_size.y {
            return true;
        }
    } else {
        // Climbing allowed: if slope difference is bigger than max_slope => collision.
        if candidate_max_floor - candidate_min_floor > collider.max_slope {
            return true;
        }
        // If the candidate is too far below or cannot climb up, treat that as collision too.
        if candidate.translation.y + collider.half_size.y < candidate_max_floor {
            let climb_needed = (candidate_max_floor - candidate.translation.y).abs();
            if climb_needed > collider.max_slope {
                return true;
            }
        }
    }

    false
}

// Example collision check between two entities, given their transforms and colliders.
// Return true if they intersect.
fn check_entity_collision(obb_a: &Obb3d, obb_b: &Obb3d) -> bool {
    obb_a.intersects_obb(obb_b)
}

// Prepare simulation data
struct SimEntity {
    entity: Entity,
    original: Transform,
    wanted: Transform,
    final_transform: Transform, // updated incrementally
    collider: Collider,
    collision_layer: CollisionLayer,
    no_collision: bool,
}

// Interpolate rotation & translation in sub-steps, checking collisions each time.
// This is the new approach to replace the old chunk of “phase 1 / phase 2” logic.
pub fn improved_unified_collision_system(
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
) {
    let my_lobby_entity = trigger.entity();
    let my_lobby = lobby
        .get(my_lobby_entity)
        .expect("Sniveling incompetent, no lobby found!");

    let map_def = &my_lobby
        .map_config
        .as_ref()
        .expect("Map config missing, you worthless creature!")
        .map;

    let mut sim_entities = Vec::new();
    for (entity, transform, mut wanted, collider, collision_layer, in_lobby) in query.iter_mut() {
        if in_lobby.0 != my_lobby_entity {
            continue;
        }
        sim_entities.push(SimEntity {
            entity,
            original: *transform,
            wanted: **wanted,
            final_transform: *transform,
            collider: collider.clone(),
            collision_layer: collision_layer.clone(),
            no_collision: collision_layer.contains(CollisionLayer::NO_COLLISION),
        });
    }

    // We do multiple sub-steps. In each sub-step, we try to advance rotation & translation from
    // the current final_transform toward the wanted transform.
    for _ in 0..SUB_STEPS {
        //
        // === Pass 1: Build candidate transforms (READ-ONLY pass) ===
        //
        // We create a temporary vector of candidate transforms, so we don't
        // mutate `sim_entities` while also needing to read from it in collisions.
        let mut step_candidates = Vec::with_capacity(sim_entities.len());
        {
            // Read only
            for sim in &sim_entities {
                if sim.no_collision {
                    // No collision => candidate is just the final transform or wanted
                    step_candidates.push(sim.final_transform);
                } else {
                    // Calculate how far we've rotated/moved so far, build candidate
                    let rot_t = rotation_progress(
                        &sim.original.rotation,
                        &sim.wanted.rotation,
                        &sim.final_transform.rotation,
                    );
                    let mov_t = translation_progress(
                        sim.original.translation,
                        sim.wanted.translation,
                        sim.final_transform.translation,
                    );
                    let step = 1.0 / SUB_STEPS as f32;

                    // Attempt both rotation + translation
                    let candidate_both = interpolate_transform_separate(
                        &sim.original,
                        &sim.wanted,
                        (rot_t + step).min(1.0),
                        (mov_t + step).min(1.0),
                    );
                    step_candidates.push(candidate_both);
                }
            }
        }

        //
        // === Pass 2: Check collisions and update final transforms (MUTABLE pass) ===
        //
        // Now we can safely iterate mutably over `sim_entities` because we are not
        // calling a function that borrows `&sim_entities` anymore. We pass `step_candidates`
        // plus a snapshot of existing final transforms/colliders for collision checks.
        let final_xforms: Vec<_> = sim_entities.iter().map(|s| s.final_transform).collect();
        let colliders: Vec<_> = sim_entities
            .iter()
            .map(|s| (s.entity, s.collider.clone(), s.no_collision))
            .collect();

        for (i, sim) in sim_entities.iter_mut().enumerate() {
            if sim.no_collision {
                // Remain at final transform or set to wanted; no collision checks needed
                sim.final_transform = sim.wanted;
                continue;
            }

            let candidate_both = step_candidates[i];
            // Check collisions with world and other entities
            if collides_with_world(&candidate_both, &sim.collider, map_def)
                || collides_with_entities(
                    sim.entity,
                    &candidate_both,
                    &colliders,
                    &final_xforms,
                    i, // index of "this" entity
                )
            {
                // Fallback logic: rotation-only, translation-only, or none
                let rot_t = rotation_progress(
                    &sim.original.rotation,
                    &sim.wanted.rotation,
                    &sim.final_transform.rotation,
                );
                let mov_t = translation_progress(
                    sim.original.translation,
                    sim.wanted.translation,
                    sim.final_transform.translation,
                );
                let step = 1.0 / SUB_STEPS as f32;

                let candidate_rot_only = interpolate_transform_separate(
                    &sim.original,
                    &sim.wanted,
                    (rot_t + step).min(1.0),
                    mov_t, // no extra translation
                );
                if collides_with_world(&candidate_rot_only, &sim.collider, map_def)
                    || collides_with_entities(
                        sim.entity,
                        &candidate_rot_only,
                        &colliders,
                        &final_xforms,
                        i,
                    )
                {
                    let candidate_move_only = interpolate_transform_separate(
                        &sim.original,
                        &sim.wanted,
                        rot_t,
                        (mov_t + step).min(1.0),
                    );
                    if collides_with_world(&candidate_move_only, &sim.collider, map_def)
                        || collides_with_entities(
                            sim.entity,
                            &candidate_move_only,
                            &colliders,
                            &final_xforms,
                            i,
                        )
                    {
                        // Both blocked => no movement
                    } else {
                        // Move only
                        sim.final_transform = candidate_move_only;
                    }
                } else {
                    // Rotate only
                    sim.final_transform = candidate_rot_only;
                }
            } else {
                // No collision => accept both rotation + translation
                sim.final_transform = candidate_both;
            }
        }
    }

    // After finishing all sub-steps, we have final_transform for each entity. Now we do triggers:
    // If final_transform is less than the wanted transform, we consider that a collision event.
    // We can compare to see if we collided with the world or with other entities.
    // For simplicity, you can do a final pass: if final_transform != wanted, we triggered a collision.
    // Then issue CollidedWithWorldTrigger or CollidedWithTrigger accordingly.

    for sim in &sim_entities {
        // Update the actual transform + WantedTransform
        commands.entity(sim.entity).insert(sim.final_transform);
        commands
            .entity(sim.entity)
            .insert(WantedTransform(sim.final_transform));

        // If final_transform != wanted, presumably we collided with something.
        // We can do more fine-grained checks here to see if it was the world or entity.
        let moved_completely = sim
            .final_transform
            .translation
            .abs_diff_eq(sim.wanted.translation, f32::EPSILON)
            && sim
                .final_transform
                .rotation
                .abs_diff_eq(sim.wanted.rotation, f32::EPSILON);
        if !moved_completely {
            // Decide if it was a world collision or entity collision or both.
            // You can store that logic from the sub-step collision checks or do a final check here.
            commands.trigger_targets(CollidedWithWorldTrigger, sim.entity);
        }
    }

    // (Optional) if you want to re-check which entities actually collided with each other in the final state,
    // do a final pass and trigger CollidedWithTrigger accordingly.

    // Finally, despawn out-of-bounds projectiles or do other logic.
    commands.trigger_targets(DespawnOutOfBoundsProjectilesTrigger, my_lobby_entity);
}

/// Measures how far we have rotated from `base` to `target`, as reflected in `current`.
/// Returns a value in [0.0 .. 1.0].
fn rotation_progress(base: &Quat, target: &Quat, current: &Quat) -> f32 {
    // Transform everything into base's local space.
    let base_inv = base.inverse();
    let local_target = base_inv * *target;
    let local_current = base_inv * *current;

    // Use `to_axis_angle()` to measure the angle.
    let (_, angle_target) = local_target.to_axis_angle();
    let (_, angle_current) = local_current.to_axis_angle();

    // If the target angle is effectively zero, consider we’re already done rotating.
    if angle_target.abs() < f32::EPSILON {
        return 1.0;
    }

    (angle_current / angle_target).clamp(0.0, 1.0)
}

fn translation_progress(base: Vec3, target: Vec3, current: Vec3) -> f32 {
    let total_dist = (target - base).length();
    if total_dist < f32::EPSILON {
        return 1.0;
    }
    let current_dist = (current - base).length();
    (current_dist / total_dist).clamp(0.0, 1.0)
}

fn collides_with_world(
    candidate: &Transform,
    collider: &Collider,
    map_def: &MapDefinition,
) -> bool {
    check_world_collision(candidate, collider, map_def)
}

fn collides_with_entities(
    this_entity: Entity,
    candidate: &Transform,
    colliders: &[(Entity, Collider, bool)],
    final_xforms: &[Transform],
    this_index: usize,
) -> bool {
    let obb_a = Obb3d::from_transform(candidate, &colliders[this_index].1);
    for (j, (other_ent, other_col, no_coll)) in colliders.iter().enumerate() {
        if *no_coll || *other_ent == this_entity {
            continue;
        }
        // Compare OBBs
        let obb_b = Obb3d::from_transform(&final_xforms[j], other_col);
        if obb_a.intersects_obb(&obb_b) {
            return true;
        }
    }
    false
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
