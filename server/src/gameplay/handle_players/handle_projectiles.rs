use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        collision_handling::{
            components::{Collider, WantedTransform},
            structs::Side,
            triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
        },
        common_components::TickBasedDespawnTimer,
        player_handling::{Health, PlayerState, RespawnTimer, TankBodyMarker},
        projectile_handling::ProjectileMarker,
        tank_types::TankType,
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
    tank_configs: TankConfigSystemParam,
    mut players: Query<
        (
            Entity,
            &Transform,
            &Collider,
            &TankType,
            Has<RespawnTimer>,
            &mut PlayerState,
            &mut Health,
            &mut OutMessageQueue,
        ),
        With<TankBodyMarker>,
    >,
    mut commands: Commands,
) {
    let projectile_entity = trigger.entity();
    let (projectile, projectile_transform) = projectile
        .get(projectile_entity)
        .expect("Failed to get projectile");
    let collided_with = trigger.event().entity;

    let mut hit_a_tank = false;
    let mut hit_side = Side::default();
    let mut damage_dealt = 0.0;
    if let Ok((
        player_entity,
        body_transform,
        body_collider,
        tank_type,
        is_respawning,
        mut player_state,
        mut health,
        mut message_queue,
    )) = players.get_mut(collided_with)
    {
        hit_a_tank = true;
        let tank_config = tank_configs
            .get_tank_type_config(tank_type)
            .expect("Failed to get tank config");
        let body_half_size = body_collider.half_size;

        // Get the relative vector from the body to the projectile in world space.
        let relative = projectile_transform.translation - body_transform.translation;

        // Transform the relative vector into the body's local space.
        let local_pos = body_transform.rotation.inverse() * relative;

        let face_dx = body_half_size.x - local_pos.x.abs();
        let face_dy = body_half_size.y - local_pos.y.abs();
        let face_dz = body_half_size.z - local_pos.z.abs();

        hit_side = if face_dx < face_dy && face_dx < face_dz {
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
        };

        let armor = tank_config
            .armor
            .get(&hit_side)
            .expect(format!("Failed to get armor for side {:?}", hit_side).as_str());
        damage_dealt = projectile.damage * (1.0 - armor);
        health.health -= projectile.damage;

        if health.health <= 0.0 && !is_respawning {
            *player_state = PlayerState::Dead;

            commands
                .entity(player_entity)
                .insert(RespawnTimer(tank_config.respawn_timer));
        }

        message_queue.push_back(MessageContainer::new(
            MessageTarget::Client(collided_with),
            NetworkMessageType::GotHit(GotHitMessageData {
                damage_received: damage_dealt,
                hit_side,
                projectile_entity,
                shooter_entity: projectile.owner,
            }),
        ));
    }

    if hit_a_tank {
        if let Ok((_, _, _, _, _, _, _, mut projectile_owner_message_queue)) =
            players.get_mut(projectile.owner)
        {
            projectile_owner_message_queue.push_back(MessageContainer::new(
                MessageTarget::Client(projectile.owner),
                NetworkMessageType::Hit(HitMessageData {
                    hit_entity: collided_with,
                    projectile_entity,
                    damage_dealt,
                    hit_side,
                }),
            ));
        }
    }

    commands.entity(projectile_entity).insert(CleanupNextTick);
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
    mut projectiles: Query<(&mut WantedTransform, &mut ProjectileMarker)>,
) {
    let lobby_entity = trigger.entity();

    let lobby = lobby.get(lobby_entity).expect("Failed to get lobby");

    for projectile in lobby.projectiles.iter() {
        let (mut transform, mut projectile) = projectiles
            .get_mut(*projectile)
            .expect("Failed to get projectile");

        if projectile.just_spawned {
            projectile.just_spawned = false;
            continue;
        }

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
            commands.entity(*projectile_entity).insert(CleanupNextTick);
        }
    }
}

pub fn despawn_projectile_on_collision_with_world(
    trigger: Trigger<CollidedWithWorldTrigger>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).insert(CleanupNextTick);
}
