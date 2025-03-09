use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        collision_handling::components::{CollisionLayer, WantedTransform},
        player_handling::{Health, PlayerState, RespawnTimer, TankBodyMarker, TankTurretMarker},
        tank_types::TankType,
    },
    networking::{
        lobby_management::{
            lobby_management::LobbyManagementSystemParam, InLobby, InTeam, MyLobby,
        },
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::entity_data::EntityDataWrapper,
            message_queue::OutMessageQueue,
        },
    },
};

use crate::{
    gameplay::triggers::StartNextTickProcessingTrigger,
    networking::handle_clients::lib::MyNetworkClient,
};

pub fn tick_respawn_timer(
    trigger: Trigger<StartNextTickProcessingTrigger>,
    mut query: Query<(Entity, &InLobby, &mut RespawnTimer)>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();

    for (player_entity, in_lobby, mut respawn_timer) in query.iter_mut() {
        if in_lobby.0 == lobby_entity {
            if respawn_timer.0 > 0 {
                respawn_timer.0 -= 1;
            } else {
                commands.trigger_targets(RespawnPlayerTrigger, player_entity);
            }
        }
    }
}

#[derive(Debug, Reflect, Event)]
pub struct RespawnPlayerTrigger;

pub fn respawn_player(
    trigger: Trigger<RespawnPlayerTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut lobby_message_queue: Query<&mut OutMessageQueue, (With<MyLobby>, Without<MyNetworkClient>)>,
    mut body_query: Query<(
        &mut Transform,
        &mut WantedTransform,
        &mut PlayerState,
        &mut Health,
        &mut CollisionLayer,
        &MyNetworkClient,
        &InTeam,
        &InLobby,
        &TankType,
        &TankBodyMarker,
    )>,
    mut turret_query: Query<&mut Transform, (With<TankTurretMarker>, Without<TankBodyMarker>)>,
    tank_configs: TankConfigSystemParam,
    mut commands: Commands,
) {
    let entity_to_respawn = trigger.entity();

    if let Ok((
        mut tank_transform,
        mut wanted_transform,
        mut player_state,
        mut health,
        mut collision_layer,
        client,
        client_team,
        client_in_lobby,
        tank_type,
        tank_body_marker,
    )) = body_query.get_mut(entity_to_respawn)
    {
        *player_state = PlayerState::Alive;
        health.health = health.max_health;

        *collision_layer = CollisionLayer::player().with_additional_layers(&[CollisionLayer::FLAG]);

        commands.entity(entity_to_respawn).remove::<RespawnTimer>();

        let (_, lobby, _) = lobby_management
            .get_lobby(client_in_lobby.0)
            .expect("Failed to get lobby");
        let map = &lobby
            .map_config
            .as_ref()
            .expect("Failed to get map config")
            .map;
        let spawn_point = client
            .assigned_spawn_point
            .expect(format!("Failed to get assigned spawn point for client {:?}", client).as_str());

        let mut turret_transform = turret_query
            .get_mut(
                tank_body_marker
                    .turret
                    .expect("Failed to get turret entity"),
            )
            .expect("Failed to get turret transform");
        turret_transform.rotation = Quat::IDENTITY;

        let spawn_point_position = map.get_spawn_point_position(client_team, spawn_point);
        let spawn_point_rotation = map.get_spawn_point_rotation(client_team, spawn_point);

        if let Some(spawn_point_position) = spawn_point_position {
            let tank_config = tank_configs
                .get_tank_type_config(tank_type)
                .expect("Failed to get tank config");
            tank_transform.translation =
                spawn_point_position + Vec3::new(0.0, tank_config.size.y / 2.0, 0.0);
            wanted_transform.translation = tank_transform.translation;
        } else {
            error!(
                "Failed to get spawn point position for team {} and spawn point {}",
                client_team.0, spawn_point
            );
        }

        if let Some(spawn_point_rotation) = spawn_point_rotation {
            tank_transform.rotation = spawn_point_rotation;
            wanted_transform.rotation = spawn_point_rotation;
        } else {
            error!(
                "Failed to get spawn point rotation for team {} and spawn point {}",
                client_team.0, spawn_point
            );
        }

        let mut message_queue = lobby_message_queue
            .get_mut(client_in_lobby.0)
            .expect("Failed to get lobby message queue");
        message_queue.push_back(MessageContainer::new(
            MessageTarget::AllInLobby,
            NetworkMessageType::PlayerRespawned(EntityDataWrapper::new(entity_to_respawn)),
        ));
    } else {
        error!(
            "Failed to get tank transform for client {:?}",
            entity_to_respawn
        );
    }
}
