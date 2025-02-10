use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use lobby_management::LobbyManagementSystemParam;

use crate::{
    asset_handling::{
        config::ServerConfigSystemParam,
        maps::{MapConfig, MapConfigSystemParam},
    },
    game::game_state::GameState,
    networking::messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_data::{
            message_error_types::ErrorMessageTypes, server_config::ServerConfigMessageData,
        },
        message_queue::{InMessageQueue, OutMessageQueue},
    },
};

use super::messages::{message_data::first_contact::ClientType, message_queue::ErrorMessageQueue};

pub mod lobby_management;

pub struct MyLobbyManagementPlugin;

impl Plugin for MyLobbyManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyLobbies>()
            .init_resource::<MyLobbies>()
            .register_type::<MyLobby>()
            .register_type::<InLobby>()
            .register_type::<LobbyState>()
            .register_type::<AwaitingFirstContact>()
            .add_observer(finish_setting_up_lobby)
            .add_observer(adding_player_to_lobby);
    }
}

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct AwaitingFirstContact(pub Timer);

impl AwaitingFirstContact {
    pub fn new(time_millis: u64) -> Self {
        Self(Timer::new(
            Duration::from_millis(time_millis),
            TimerMode::Once,
        ))
    }
}

#[derive(Debug, Default, Reflect, Clone, Component)]
#[reflect(Component)]
pub struct InTeam {
    pub team_name: String,
}

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct InLobby(pub Entity);

#[derive(Debug, Event)]
pub struct PlayerRemovedFromLobbyTrigger;

#[derive(Debug, Event)]
pub struct PlayerWantsToJoinLobbyTrigger {
    pub player: Entity,
    pub lobby: Entity,
    pub player_type: ClientType,
    pub team_name: Option<String>,
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct MyLobbies {
    pub lobbies: HashMap<String, Entity>,
}

#[derive(Debug, Reflect, Default, Component, PartialEq)]
#[reflect(Component)]
#[require(InMessageQueue, OutMessageQueue)]
pub struct MyLobby {
    pub state: LobbyState,
    pub lobby_name: String,

    pub players: Vec<Entity>,
    pub spectators: Vec<Entity>,

    pub map_name: String,
    pub map_config: Option<MapConfig>,

    pub tick_timer: Timer,

    pub game_state: GameState,
}

impl MyLobby {
    pub fn new(name: String, map_name: String, tick_rate: u64) -> Self {
        let time_per_tick = 1.0 / tick_rate as f32;
        info!("Time per tick: {}", time_per_tick);

        Self {
            state: LobbyState::default(),
            lobby_name: name,

            players: Vec::new(),
            spectators: Vec::new(),

            map_name,
            map_config: None,

            tick_timer: Timer::from_seconds(time_per_tick, TimerMode::Repeating),

            game_state: GameState::default(),
        }
    }

    pub fn with_player(mut self, player: Entity) -> Self {
        self.players.push(player);
        self
    }

    pub fn with_spectator(mut self, spectator: Entity) -> Self {
        self.spectators.push(spectator);
        self
    }

    pub fn get_team(&self, team_name: &str) -> Option<&Vec<Entity>> {
        self.map_config
            .as_ref()
            .and_then(|map_config| map_config.get_team(team_name).map(|team| &team.players))
    }
}

#[derive(Debug, Reflect, Default, PartialEq)]
pub enum LobbyState {
    #[default]
    SettingUp,
    ReadyToStart,
    InProgress,
    Finished,
}

pub fn remove_player_from_lobby(
    trigger: Trigger<PlayerRemovedFromLobbyTrigger>,
    mut commands: Commands,
    server_config: ServerConfigSystemParam,
) {
    let server_config = server_config.server_config();

    let player = trigger.entity();
    info!("Player {} removed from lobby", player);

    // TODO: let player know they've been removed from the lobby?

    commands
        .entity(player)
        .remove::<InLobby>()
        .remove::<InTeam>()
        .insert(AwaitingFirstContact::new(
            server_config.timeout_first_contact,
        ));
}

fn adding_player_to_lobby(
    trigger: Trigger<PlayerWantsToJoinLobbyTrigger>,
    mut lobby_management: LobbyManagementSystemParam,
    mut commands: Commands,
    mut player_error_message_queues: Query<&mut ErrorMessageQueue>,
    mut player_message_queues: Query<&mut OutMessageQueue>,
    server_config: ServerConfigSystemParam,
) {
    let PlayerWantsToJoinLobbyTrigger {
        player,
        lobby: lobby_entity,
        player_type,
        team_name,
    } = trigger.event();

    if let Ok(mut lobby) = lobby_management.get_lobby_mut(*lobby_entity) {
        match lobby.state {
            LobbyState::InProgress | LobbyState::Finished => {
                error!(
                    "Player {:?} wants to join lobby {:?} but it is in state {:?}",
                    player, lobby_entity, lobby.state
                );
                let mut queue = player_error_message_queues.get_mut(*player).unwrap();
                queue.push_back(MessageContainer::new(
                    MessageTarget::Client(*player),
                    NetworkMessageType::MessageError(ErrorMessageTypes::LobbyAlreadyRunning(
                        format!(
                            "Lobby can't be joined because it is in state {:?}",
                            lobby.state
                        ),
                    )),
                ));

                return;
            }
            _ => {}
        }

        match player_type {
            ClientType::Player => {
                if let Some(team_name) = team_name {
                    info!(
                        "Player {:?} joining lobby {:?} on team {:?}",
                        player, lobby_entity, team_name
                    );

                    lobby.players.push(*player);

                    lobby
                        .map_config
                        .as_mut()
                        .expect("Map config should be set up by now")
                        .insert_player_into_team(team_name, *player);

                    commands.entity(*player).insert((InTeam {
                        team_name: team_name.clone(),
                    },));

                    let server_config = server_config.server_config();
                    let mut out_message_queue = player_message_queues.get_mut(*player).unwrap();
                    out_message_queue.push_back(MessageContainer::new(
                        MessageTarget::Client(*player),
                        NetworkMessageType::ServerConfig(ServerConfigMessageData {
                            tick_rate: server_config.tick_rate,
                            client_id: *player,
                        }),
                    ));
                } else {
                    error!("Player wants to join lobby without specifying a team name");
                    let mut queue = player_error_message_queues.get_mut(*player).unwrap();
                    queue.push_back(MessageContainer::new(
                        MessageTarget::Client(*player),
                        NetworkMessageType::MessageError(ErrorMessageTypes::LobbyManagementError(
                            "Player wants to join lobby without specifying a team name".to_string(),
                        )),
                    ));
                }
            }
            ClientType::Spectator => {
                lobby.spectators.push(*player);
            }
        }

        commands
            .entity(*player)
            .insert((InLobby(*lobby_entity),))
            .remove::<AwaitingFirstContact>();
    }
}

fn finish_setting_up_lobby(
    trigger: Trigger<OnAdd, MyLobby>,
    mut lobby_management: LobbyManagementSystemParam,
    map_config: MapConfigSystemParam,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let mut lobby = lobby_management.get_lobby_mut(lobby_entity).unwrap();
    if lobby.map_config.is_none() {
        if let Some(map_config) = map_config.get_map_config(&lobby.map_name) {
            info!(
                "Adding map config \"{}\" to lobby \"{}\"",
                lobby.map_name, lobby.lobby_name
            );

            lobby.map_config = Some(map_config.clone());

            lobby.state = LobbyState::ReadyToStart;
        } else {
            error!(
                "Failed to get map config for lobby \"{}\" with map name \"{}\"",
                lobby.lobby_name, lobby.map_name
            );
            lobby_management.remove_lobby(lobby_entity, &mut commands);
        }
    }
}
