use bevy::prelude::*;
use shared::{
    game::collision_handling::{
        components::{Collider, CollisionLayer, WantedTransform},
        structs::Obb3d,
        triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
    },
    networking::lobby_management::{InLobby, MyLobby},
};

use crate::gameplay::triggers::FinishedNextSimulationStepTrigger;

const STEP_SIZE: f32 = 0.05;

/// Our unified collision system—where the world and pitiful entities alike learn to fear my wrath!
pub fn unified_collision_system(
    trigger: Trigger<FinishedNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    mut colliders: Query<(
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
    mut commands: Commands,
) {
    // Secure dominion over the wretched lobby.
    let my_lobby_entity = trigger.entity();
    let my_lobby = lobby
        .get(my_lobby_entity)
        .expect("Ikit Claw demands a proper lobby!");
    let map_def = &my_lobby
        .map_config
        .as_ref()
        .expect("The map config is missing—pathetic!")
        .map;

    // A temporary host for our simulation data.
    #[derive(Clone)]
    struct SimEntity {
        entity: Entity,
        current: Transform,
        target: Transform,
        collider: Collider,
        collision_layer: CollisionLayer,
        stopped: bool,              // has the entity been halted by a collision?
        stopped_due_to_world: bool, // true if the world crushed its pitiful form
        last_safe_transform: Transform,
    }

    let mut sim_entities = Vec::new();

    // Gather all entities that dare to move.
    for (entity, transform, wanted_transform, collider, collision_layer, in_lobby) in
        colliders.iter_mut()
    {
        if in_lobby.0 != my_lobby_entity {
            continue;
        }
        sim_entities.push(SimEntity {
            entity,
            current: *transform,
            target: **wanted_transform,
            collider: collider.clone(),
            collision_layer: collision_layer.clone(),
            stopped: false,
            stopped_due_to_world: false,
            last_safe_transform: *transform,
        });
    }

    // Prepare to record the infamy of entity collisions.
    let mut entity_collision_events: Vec<(Entity, Entity)> = Vec::new();

    // Determine the number of simulation steps.
    let num_steps = (1.0 / STEP_SIZE).ceil() as usize;

    // Run the unified simulation—each tick a step closer to inevitable ruin!
    for step in 1..=num_steps {
        let t = {
            let raw = step as f32 * STEP_SIZE;
            if raw > 1.0 {
                1.0
            } else {
                raw
            }
        };

        // First, update each entity's candidate transform if it hasn't yet met its doom.
        for sim in sim_entities.iter_mut() {
            if sim.stopped {
                continue;
            }
            let candidate = interpolate_transform(&sim.current, &sim.target, t);

            // --- WORLD COLLISION CHECK ---
            // Compute the four corners of our entity’s bounding footprint.
            let right = candidate.right();
            let forward = candidate.forward();
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
                // The world has claimed this pitiful entity—halt its progress!
                sim.stopped = true;
                sim.stopped_due_to_world = true;
                // Do not update last_safe_transform; it remains from the previous step.
                continue;
            }
            let candidate_floor = tile_heights.clone().into_iter().fold(f32::MIN, f32::max);
            if sim.collider.max_slope == 0.0 {
                if candidate.translation.y < candidate_floor + sim.collider.half_size.y {
                    sim.stopped = true;
                    sim.stopped_due_to_world = true;
                    continue;
                }
                sim.last_safe_transform = candidate;
            } else {
                if {
                    // If any tile’s slope exceeds our collider’s tolerance, doom awaits.
                    let exceeds = tile_heights
                        .into_iter()
                        .any(|h| (candidate_floor - h).abs() > sim.collider.max_slope);
                    exceeds
                } {
                    sim.stopped = true;
                    sim.stopped_due_to_world = true;
                    continue;
                }
                sim.last_safe_transform = Transform {
                    translation: Vec3::new(
                        candidate.translation.x,
                        candidate_floor + sim.collider.half_size.y,
                        candidate.translation.z,
                    ),
                    rotation: candidate.rotation,
                    scale: candidate.scale,
                };
            }
        }

        // --- ENTITY COLLISION CHECK ---
        // Now, check every pair of still-moving entities for a fated collision.
        let len = sim_entities.len();
        for i in 0..len {
            for j in (i + 1)..len {
                // Only consider pairs that haven’t already met their demise.
                if sim_entities[i].stopped || sim_entities[j].stopped {
                    continue;
                }
                // Respect collision layers and ignore lists.
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
                let a_candidate = sim_entities[i].last_safe_transform;
                let b_candidate = sim_entities[j].last_safe_transform;
                let a_obb = Obb3d::from_transform(&a_candidate, &sim_entities[i].collider);
                let b_obb = Obb3d::from_transform(&b_candidate, &sim_entities[j].collider);

                #[cfg(feature = "debug")]
                {
                    if let Ok((mut a_gizmos, a_in_lobby)) =
                        debug_obb_gizmos.get_mut(sim_entities[i].entity)
                    {
                        if a_in_lobby.0 == my_lobby_entity {
                            a_gizmos.push((t, a_obb.clone()));
                        }
                    }
                    if let Ok((mut b_gizmos, b_in_lobby)) =
                        debug_obb_gizmos.get_mut(sim_entities[j].entity)
                    {
                        if b_in_lobby.0 == my_lobby_entity {
                            b_gizmos.push((t, b_obb.clone()));
                        }
                    }
                }

                if a_obb.intersects_obb(&b_obb) {
                    // Collision between entities detected—halt both before they strike a false step!
                    sim_entities[i].stopped = true;
                    sim_entities[j].stopped = true;
                    entity_collision_events.push((sim_entities[i].entity, sim_entities[j].entity));
                }
            }
        }
    } // end simulation loop

    // Update each entity's position to the last safe transform they achieved.
    for sim in sim_entities.iter() {
        // Update the Transform component.
        commands.entity(sim.entity).insert(Transform {
            translation: sim.last_safe_transform.translation,
            rotation: sim.last_safe_transform.rotation,
            scale: sim.last_safe_transform.scale,
        });
        // And update their WantedTransform so they don't try to move further.
        commands
            .entity(sim.entity)
            .insert(WantedTransform(sim.last_safe_transform));
    }

    // Dispatch the world collision triggers.
    let world_collision_entities: Vec<Entity> = sim_entities
        .iter()
        .filter(|sim| sim.stopped && sim.stopped_due_to_world)
        .map(|sim| sim.entity)
        .collect();
    if !world_collision_entities.is_empty() {
        commands.trigger_targets(CollidedWithWorldTrigger, world_collision_entities);
    }
    // Dispatch entity collision triggers.
    for (a, b) in entity_collision_events.into_iter() {
        commands.trigger_targets(CollidedWithTrigger { entity: b }, a);
        commands.trigger_targets(CollidedWithTrigger { entity: a }, b);
    }
}

/// Interpolates between two transforms at a given fraction `t`.
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
