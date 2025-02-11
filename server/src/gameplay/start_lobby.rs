use bevy::prelude::*;
use shared::{
    asset_handling::config::ServerConfigSystemParam,
    networking::{
        lobby_management::{lobby_management::LobbyManagementSystemParam, LobbyState, MyLobby},
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::game_starts::GameStarts,
            message_queue::ImmediateOutMessageQueue,
        },
    },
};

#[derive(Debug, Event)]
pub struct StartLobbyTrigger;

pub fn check_if_lobby_should_start(
    lobbies: Query<(Entity, &MyLobby), Changed<MyLobby>>,
    mut commands: Commands,
) {
    for (entity, lobby) in lobbies.iter() {
        if lobby.state != LobbyState::ReadyToStart {
            continue;
        }

        info!("Checking if lobby should start: {:?}", entity);
        let needed_players = lobby
            .map_config
            .as_ref()
            .expect("Failed to get map config")
            .teams
            .iter()
            .fold(0, |acc, (_, team)| acc + team.max_players);
        if lobby.players.len() < needed_players {
            info!(
                "Not enough players in lobby: {} < {}",
                lobby.players.len(),
                needed_players
            );
            continue;
        }

        commands.trigger_targets(StartLobbyTrigger, entity);
    }
}

pub fn start_lobby(
    trigger: Trigger<StartLobbyTrigger>,
    mut lobby_management: LobbyManagementSystemParam,
    mut queues: Query<&mut ImmediateOutMessageQueue>,
    server_config: ServerConfigSystemParam,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby");

    let server_config = server_config.server_config();

    let connected_clients = lobby_management.get_connected_configs_in_lobby(lobby_entity);
    for (_, player_entity) in lobby.players.iter() {
        let mut queue = queues.get_mut(*player_entity).expect("Failed to get queue");
        queue.push_back(MessageContainer::new(
            MessageTarget::Client(*player_entity),
            NetworkMessageType::GameStarts(GameStarts {
                client_id: *player_entity,
                connected_clients: connected_clients.clone(),
                tick_rate: server_config.tick_rate,
            }),
        ));
    }

    lobby_management
        .get_lobby_mut(lobby_entity)
        .expect("Failed to get lobby")
        .state = LobbyState::InProgress;
}
