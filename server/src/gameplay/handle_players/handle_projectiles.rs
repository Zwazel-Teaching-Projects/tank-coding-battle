use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::{
            components::{Collider, WantedTransform},
            structs::Side,
            triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
        },
        common_components::TickBasedDespawnTimer,
        player_handling::{Health, TankBodyMarker},
        projectile_handling::ProjectileMarker,
    },
    networking::{
        lobby_management::MyLobby,
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::tank_messages::hit_message_data::{GotHitMessageData, HitMessageData},
            message_queue::OutMessageQueue,
        },
    },
};

use crate::gameplay::{
    lobby_cleanup::CleanupNextTick,
    triggers::{
        FinishedNextSimulationStepTrigger, StartNextSimulationStepTrigger,
        StartNextTickProcessingTrigger,
    },
};

pub fn colliding_with_entity(
    trigger: Trigger<CollidedWithTrigger>,
    projectile: Query<(&ProjectileMarker, &Transform)>,
    mut players: Query<
        (&Transform, &Collider, &mut Health, &mut OutMessageQueue),
        With<TankBodyMarker>,
    >,
    mut commands: Commands,
) {
    // TODO: Apply damage to player
    let projectile_entity = trigger.entity();
    let (projectile, projectile_transform) = projectile
        .get(projectile_entity)
        .expect("Failed to get projectile");
    let collided_with = trigger.event().entity;

    let mut hit_side = None;
    if let Ok((body_transform, body_collider, mut health, mut message_queue)) =
        players.get_mut(collided_with)
    {
        let body_half_size = body_collider.half_size;

        // Get the relative vector from the body to the projectile in world space.
        let relative = projectile_transform.translation - body_transform.translation;

        // Transform the relative vector into the body's local space.
        let local_pos = body_transform.rotation.inverse() * relative;

        let face_dx = body_half_size.x - local_pos.x.abs();
        let face_dy = body_half_size.y - local_pos.y.abs();
        let face_dz = body_half_size.z - local_pos.z.abs();

        hit_side = Some(if face_dx < face_dy && face_dx < face_dz {
            // Collision on x-axis (left or right)
            if local_pos.x > 0.0 {
                Side::Left
            } else {
                Side::Right
            }
        } else if face_dy < face_dx && face_dy < face_dz {
            // Collision on y-axis (top or bottom)
            if local_pos.y > 0.0 {
                Side::Top
            } else {
                Side::Bottom
            }
        } else {
            // Collision on z-axis (front or back)
            if local_pos.z > 0.0 {
                Side::Front
            } else {
                Side::Back
            }
        });

        commands.entity(projectile_entity).insert(CleanupNextTick);
        // TODO: Armor calculation
        health.health -= projectile.damage;
        message_queue.push_back(MessageContainer::new(
            MessageTarget::Client(collided_with),
            NetworkMessageType::GotHit(GotHitMessageData {
                damage_received: projectile.damage,
                hit_side: hit_side.expect("Failed to get side hit"),
                projectile_entity,
                shooter_entity: projectile.owner,
            }),
        ));
    }

    if let Ok((_, _, _, mut projectile_owner_message_queue)) = players.get_mut(projectile.owner) {
        projectile_owner_message_queue.push_back(MessageContainer::new(
            MessageTarget::Client(projectile.owner),
            NetworkMessageType::Hit(HitMessageData {
                hit_entity: collided_with,
                projectile_entity,
                damage_dealt: projectile.damage,
                hit_side: hit_side.expect("Failed to get side hit"),
            }),
        ));
    }
}

pub fn handle_despawn_timer(
    trigger: Trigger<StartNextTickProcessingTrigger>,
    mut lobby: Query<&mut MyLobby>,
    mut despawn_timer: Query<(Entity, &mut TickBasedDespawnTimer)>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let mut lobby = lobby.get_mut(lobby_entity).expect("Failed to get lobby");

    lobby.projectiles.retain(|projectile| {
        if let Ok((entity, mut despawn_timer)) = despawn_timer.get_mut(*projectile) {
            if despawn_timer.ticks_left > 0 {
                despawn_timer.ticks_left -= 1;
                true
            } else {
                commands.entity(entity).despawn_recursive();
                false
            }
        } else {
            false
        }
    });
}

pub fn move_projectiles(
    trigger: Trigger<StartNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    mut projectiles: Query<(&mut WantedTransform, &ProjectileMarker)>,
) {
    let lobby_entity = trigger.entity();

    let lobby = lobby.get(lobby_entity).expect("Failed to get lobby");

    for projectile in lobby.projectiles.iter() {
        let (mut transform, projectile) = projectiles
            .get_mut(*projectile)
            .expect("Failed to get projectile");

        let rotation = transform.rotation;
        transform.translation += rotation * Vec3::new(0.0, 0.0, projectile.speed);
    }
}

pub fn despawn_out_of_bounds(
    trigger: Trigger<FinishedNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    projectiles: Query<&Transform, With<ProjectileMarker>>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();

    let lobby = lobby.get(lobby_entity).expect("Failed to get lobby");
    let map = &lobby
        .map_config
        .as_ref()
        .expect("Failed to get map config")
        .map;

    for projectile_entity in lobby.projectiles.iter() {
        let transform = projectiles
            .get(*projectile_entity)
            .expect("Failed to get projectile");

        if !map.is_inside_bounds(transform.translation) {
            commands.entity(*projectile_entity).despawn_recursive();
        }
    }
}

pub fn despawn_on_collision_with_world(
    trigger: Trigger<CollidedWithWorldTrigger>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).despawn_recursive();
}
