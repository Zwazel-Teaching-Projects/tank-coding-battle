use bevy::{prelude::*, utils::HashMap};
use shared::{
    asset_handling::config::{server_config::ServerConfigSystemParam, TankConfigSystemParam},
    game::{player_handling::TankBodyMarker, tank_types::TankType},
    networking::{
        lobby_management::{
            lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
            InLobby, InTeam, LobbyState, MyLobby,
        },
        messages::{
            message_container::{
                MessageContainer, MessageTarget, NetworkMessageType, StartGameTrigger,
            },
            message_data::{
                first_contact::ClientType,
                game_starts::{ConnectedClientConfig, GameStarts},
                message_error_types::ErrorMessageTypes,
            },
            message_queue::ImmediateOutMessageQueue,
        },
    },
};

use crate::networking::handle_clients::lib::MyNetworkClient;

use super::{
    capture_the_flag::triggers::InitAllFlagsTrigger,
    handle_players::{dummy_handling::DummyClientMarker, handle_spawning::RespawnPlayerTrigger},
};

#[derive(Debug, Event)]
pub struct StartLobbyTrigger;

pub fn check_if_lobby_should_start(
    trigger: Trigger<StartGameTrigger>,
    mut lobbies: Query<&mut MyLobby>,
    mut client_queues: Query<&mut ImmediateOutMessageQueue>,
    mut clients: Query<&mut MyNetworkClient>,
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

    // Assign every player, that hasn't already, a spawnpoint.
    let map_config = lobby.map_config.as_mut().expect("Failed to get map config");
    // Team name -> set of taken spawn point ids
    let mut taken_spawn_points_team: HashMap<String, Vec<usize>> = HashMap::new();
    for (team_name, team) in map_config.teams.iter() {
        let spawn_points = map_config
            .map
            .get_all_spawn_points_of_group(&team_name)
            .iter()
            .map(|(_, id)| *id)
            .collect::<Vec<_>>();
        let mut clients_without_spawn_points = Vec::new();

        for player in team.players.iter() {
            let client = clients.get(*player).expect("Failed to get client");
            if let Some(assigned_spawn_point) = client.assigned_spawn_point {
                taken_spawn_points_team
                    .entry(team_name.clone())
                    .or_insert_with(Vec::new)
                    .push(assigned_spawn_point);
                info!(
                    "Player {:?} already has spawn point {:?}",
                    client.name, assigned_spawn_point
                );
            } else {
                clients_without_spawn_points.push(*player);
            }
        }

        for client in clients_without_spawn_points {
            // Find the first spawn point that is not taken for this team,
            // or just take the first one if all available spawn points are already assigned
            let taken_points = taken_spawn_points_team
                .entry(team_name.clone())
                .or_insert_with(Vec::new);

            let spawn_point = spawn_points
                .iter()
                .find(|spawn_point| !taken_points.contains(spawn_point))
                .unwrap_or(&spawn_points[0]);

            let mut client = clients.get_mut(client).expect("Failed to get client");
            client.assigned_spawn_point = Some(*spawn_point);
            taken_points.push(*spawn_point);
        }
    }

    if start_config.fill_empty_slots_with_dummies {
        // go through all teams, and if they have less players than max, fill them with dummies
        let mut dummy_players = Vec::new();
        for (team_name, team) in map_config.teams.iter_mut() {
            let needed_players = team.max_players - team.players.len();
            let spawn_points = map_config
                .map
                .get_all_spawn_points_of_group(&team_name)
                .iter()
                .map(|(_, id)| *id)
                .collect::<Vec<_>>();

            for i in 0..needed_players {
                let dummy_name = format!("{}-dummy-{}", team_name, i);
                // Find the first spawn point that is not taken for this team,
                // or just take the first one if all available spawn points are already assigned
                let taken_points = taken_spawn_points_team
                    .entry(team_name.clone())
                    .or_insert_with(Vec::new);

                let spawn_point = spawn_points
                    .iter()
                    .find(|spawn_point| !taken_points.contains(spawn_point))
                    .unwrap_or(&spawn_points[0]);

                let mut dummy_client = MyNetworkClient::new_dummy(dummy_name.clone());
                dummy_client.assigned_spawn_point = Some(*spawn_point);
                taken_points.push(*spawn_point);

                let client_type = ClientType::Dummy;
                let tank_type = TankType::LightTank; // TODO: Randomly select tank type? better would be if we could set up a lobby with specific dummies

                let dummy = commands
                    .spawn((
                        Name::new(format!("{:?}_{:?}_{}", client_type, tank_type, dummy_name)),
                        DummyClientMarker,
                        InTeam(team_name.clone()),
                        InLobby(lobby_entity),
                        dummy_client,
                        client_type,
                        tank_type,
                        TankBodyMarker::default(),
                    ))
                    .id();
                team.players.push(dummy);
                dummy_players.push((dummy_name, dummy, ClientType::Dummy));
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
    clients: Query<(&MyNetworkClient, &ClientType, Option<&TankType>)>,
    server_config: ServerConfigSystemParam,
    tank_config: TankConfigSystemParam,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let map;
    let team_configs;
    {
        let (_, lobby, mut lobby_state) = lobby_management
            .get_lobby_mut(lobby_entity)
            .expect("Failed to get lobby");
        let map_config = lobby.map_config.as_ref().expect("Failed to get map config");
        map = map_config.map.clone();
        team_configs = map_config.teams.clone();
        let team_names = team_configs.keys().cloned().collect::<Vec<_>>();
        lobby_state.setup_score(team_names);
    }

    commands.trigger_targets(InitAllFlagsTrigger, lobby_entity);

    let server_config = server_config.server_config();
    let tank_configs = tank_config.tank_configs();

    let connected_clients =
        get_connected_configs_in_lobby(&lobby_management, lobby_entity, &clients);
    match lobby_management.targets_get_players_and_spectators_in_lobby(LobbyManagementArgument {
        lobby: Some(lobby_entity),
        ..default()
    }) {
        Ok(clients_in_lobby) => {
            // Send to all clients and spectators
            for client_entity in clients_in_lobby {
                let mut queue = queues.get_mut(client_entity).expect("Failed to get queue");
                let (_, client_type, _) = clients.get(client_entity).expect("Failed to get client");

                match client_type {
                    ClientType::Spectator => {}
                    _ => {
                        commands.trigger_targets(RespawnPlayerTrigger, client_entity);
                    }
                }

                queue.push_back(MessageContainer::new(
                    MessageTarget::Client(client_entity),
                    NetworkMessageType::GameStarts(GameStarts {
                        client_id: client_entity,
                        connected_clients: connected_clients.clone(),
                        tick_rate: server_config.tick_rate,
                        map_definition: map.clone(),
                        team_configs: team_configs.clone(),
                        tank_configs: tank_configs.tanks.clone(),
                    }),
                ));
            }

            lobby_management
                .get_lobby_mut(lobby_entity)
                .expect("Failed to get lobby")
                .1
                .state = LobbyState::InProgress;
        }
        Err(err) => error!("Failed to get players in lobby: {}", err),
    }
}

fn get_connected_configs_in_lobby(
    lobby_management: &LobbyManagementSystemParam,
    lobby_entity: Entity,
    clients: &Query<(&MyNetworkClient, &ClientType, Option<&TankType>)>,
) -> Vec<ConnectedClientConfig> {
    lobby_management
        .get_lobby(lobby_entity)
        .map(|(_, lobby, _)| {
            let map_config = lobby.map_config.as_ref().unwrap();

            let mut connected_configs = Vec::new();
            // Iterate through each team directly
            for (team_name, team) in map_config.teams.iter() {
                for player in team.players.iter() {
                    if let Some((client, _client_type, tank_type)) = clients.get(*player).ok() {
                        connected_configs.push(ConnectedClientConfig {
                            client_id: *player,
                            client_name: client.name.as_ref().unwrap().clone(),
                            client_team: team_name.clone(),
                            client_tank_type: tank_type.expect("Failed to get tank type").clone(),
                            assigned_spawn_point: client.assigned_spawn_point.unwrap(),
                        });
                    } else {
                        error!(
                            "Player {:?} in team {} not found in lobby.players",
                            player, team_name
                        );
                    }
                }
            }
            connected_configs
        })
        .unwrap_or_default()
}
