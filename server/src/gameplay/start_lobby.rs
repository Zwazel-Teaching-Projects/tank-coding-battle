use bevy::prelude::*;
use shared::{
    asset_handling::config::ServerConfigSystemParam,
    networking::{
        lobby_management::{lobby_management::LobbyManagementSystemParam, LobbyState, MyLobby},
        messages::{
            message_container::{
                MessageContainer, MessageTarget, NetworkMessageType, StartGameTrigger,
            },
            message_data::{game_starts::GameStarts, message_error_types::ErrorMessageTypes},
            message_queue::ImmediateOutMessageQueue,
        },
    },
};

#[derive(Debug, Event)]
pub struct StartLobbyTrigger;

pub fn check_if_lobby_should_start(
    trigger: Trigger<StartGameTrigger>,
    lobbies: Query<&MyLobby>,
    mut client_queues: Query<&mut ImmediateOutMessageQueue>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let start_config = &(**trigger.event());
    let lobby = lobbies.get(entity).expect("Failed to get lobby");
    let sender = trigger.sender.unwrap();
    let mut sender_queue = client_queues
        .get_mut(sender)
        .expect("Failed to get queue for sender");

    if lobby.state != LobbyState::ReadyToStart {
        sender_queue.push_back(MessageContainer::new(
            MessageTarget::Client(sender),
            NetworkMessageType::MessageError(ErrorMessageTypes::LobbyNotReadyToStart(format!(
                "Lobby is not ready to start: {:?}",
                lobby.state
            ))),
        ));

        return;
    }

    if start_config.fill_empty_slots_with_bots {
        // TODO: Implement bot filling
    } else {
        let needed_players = lobby
            .map_config
            .as_ref()
            .expect("Failed to get map config")
            .teams
            .iter()
            .fold(0, |acc, (_, team)| acc + team.max_players);
        if lobby.players.len() < needed_players {
            sender_queue.push_back(MessageContainer::new(
                MessageTarget::Client(sender),
                NetworkMessageType::MessageError(ErrorMessageTypes::LobbyNotReadyToStart(format!(
                    "Not enough players in lobby: {} < {}",
                    lobby.players.len(),
                    needed_players
                ))),
            ));

            return;
        }
    }

    commands.trigger_targets(StartLobbyTrigger, entity);
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
    let map = &lobby
        .map_config
        .as_ref()
        .expect("Failed to get map config")
        .map;
    let team_configs = &lobby
        .map_config
        .as_ref()
        .expect("Failed to get map config")
        .teams;

    let server_config = server_config.server_config();

    let connected_clients = lobby_management.get_connected_configs_in_lobby(lobby_entity);
    for client_entity in lobby
        .players
        .iter()
        .map(|(_, entity)| *entity)
        .chain(lobby.spectators.iter().copied())
    {
        let mut queue = queues.get_mut(client_entity).expect("Failed to get queue");
        queue.push_back(MessageContainer::new(
            MessageTarget::Client(client_entity),
            NetworkMessageType::GameStarts(GameStarts {
                client_id: client_entity,
                connected_clients: connected_clients.clone(),
                tick_rate: server_config.tick_rate,
                map_definition: map.clone(),
                team_configs: team_configs.clone(),
            }),
        ));
    }

    lobby_management
        .get_lobby_mut(lobby_entity)
        .expect("Failed to get lobby")
        .state = LobbyState::InProgress;
}
