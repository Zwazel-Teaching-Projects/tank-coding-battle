use bevy::prelude::*;
use shared::{
    asset_handling::config::ServerConfigSystemParam,
    networking::{
        lobby_management::{
            lobby_management::LobbyManagementSystemParam, InLobby, InTeam, LobbyState, MyLobby,
        },
        messages::{
            message_container::{
                MessageContainer, MessageTarget, NetworkMessageType, StartGameTrigger,
            },
            message_data::{
                first_contact::ClientType, game_starts::GameStarts,
                message_error_types::ErrorMessageTypes,
            },
            message_queue::ImmediateOutMessageQueue,
        },
    },
};

use crate::networking::handle_clients::lib::MyNetworkClient;

use super::handle_players::dummy_handling::DummyClientMarker;

#[derive(Debug, Event)]
pub struct StartLobbyTrigger;

pub fn check_if_lobby_should_start(
    trigger: Trigger<StartGameTrigger>,
    mut lobbies: Query<&mut MyLobby>,
    mut client_queues: Query<&mut ImmediateOutMessageQueue>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let start_config = &(**trigger.event());
    let mut lobby = lobbies.get_mut(lobby_entity).expect("Failed to get lobby");
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

    if start_config.fill_empty_slots_with_dummies {
        // go through all teams, and if they have less players than max, fill them with dummies
        let mut dummy_players = Vec::new();
        for (team_name, team) in lobby.map_config.as_mut().unwrap().teams.iter_mut() {
            let needed_players = team.max_players - team.players.len();
            for i in 0..needed_players {
                let dummy_name = format!("{}-dummy-{}", team_name, i);
                let dummy = commands
                    .spawn((
                        Name::new(dummy_name.clone()),
                        DummyClientMarker,
                        InTeam(team_name.clone()),
                        InLobby(lobby_entity),
                        MyNetworkClient::new_dummy(dummy_name.clone()),
                        ClientType::Dummy,
                    ))
                    .id();
                team.players.push(dummy);
                dummy_players.push((dummy_name, dummy));
            }
        }
        lobby.players.extend(dummy_players);
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

    commands.trigger_targets(StartLobbyTrigger, lobby_entity);
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
    // Send to all clients and spectators
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
