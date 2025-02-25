use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use lobby_management::LobbyManagementSystemParam;

use crate::{
    asset_handling::{
        config::ServerConfigSystemParam,
        maps::{MapConfig, MapConfigSystemParam},
    },
    game::game_state::LobbyGameState,
    networking::messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_data::{message_error_types::ErrorMessageTypes, text_data::TextDataWrapper},
        message_queue::OutMessageQueue,
    },
};

use super::messages::{
    message_data::first_contact::ClientType,
    message_queue::{ImmediateOutMessageQueue, MessageQueue},
};

pub mod despawn_on_lobby_removal;
pub mod lobby_management;

pub struct MyLobbyManagementPlugin;

impl Plugin for MyLobbyManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyLobbies>()
            .init_resource::<MyLobbies>()
            .register_type::<MyLobby>()
            .register_type::<InLobby>()
            .register_type::<InTeam>()
            .register_type::<LobbyState>()
            .register_type::<AwaitingFirstContact>()
            .add_observer(finish_setting_up_lobby)
            .add_observer(adding_player_to_lobby)
            .add_observer(despawn_on_lobby_removal::add_observers_on_lobby_join)
            .add_observer(despawn_on_lobby_removal::update_my_lobbies_on_lobby_despawn);
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

#[derive(Debug, Default, Reflect, Clone, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct InTeam(pub String);

#[derive(Debug, Reflect, Component, Deref, DerefMut, Clone)]
#[reflect(Component)]
pub struct InLobby(pub Entity);

#[derive(Debug, Event)]
pub struct PlayerRemovedFromLobbyTrigger;

#[derive(Debug, Event)]
pub struct PlayerWantsToJoinLobbyTrigger {
    pub player: Entity,
    pub player_name: String,
    pub lobby: Entity,
    pub player_type: ClientType,
    pub team_name: Option<String>,
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct MyLobbies {
    pub lobbies: HashMap<String, Entity>,
}

impl MyLobbies {
    pub fn remove_lobby(&mut self, lobby: Entity) {
        if let Some(lobby_name) = self.lobbies.iter().find_map(|(k, &entity)| {
            if entity == lobby {
                Some(k.clone())
            } else {
                None
            }
        }) {
            self.lobbies.remove(&lobby_name);
        }
    }
}

#[derive(Debug, Reflect, Default, Component, PartialEq)]
#[reflect(Component)]
#[require(OutMessageQueue, LobbyGameState)]
pub struct MyLobby {
    pub state: LobbyState,
    pub lobby_name: String,

    pub players: Vec<(String, Entity, ClientType)>,
    pub spectators: Vec<Entity>,
    pub projectiles: Vec<Entity>,

    pub map_name: String,
    pub map_config: Option<MapConfig>,

    /// Timer for ticking the lobby
    pub tick_timer: Timer,
    /// The currently, finished tick
    pub tick_processed: u64,
    /// All unprocessed messages received by the lobby (will be processed in the next tick, or dropped if the messages are too old)
    pub messages: MessageQueue,
}

impl MyLobby {
    pub fn new(name: String, map_name: String, tick_rate: u64) -> Self {
        let time_per_tick = 1.0 / tick_rate as f32;

        Self {
            state: LobbyState::default(),
            lobby_name: name,

            players: Vec::new(),
            spectators: Vec::new(),
            projectiles: Vec::new(),

            map_name,
            map_config: None,

            tick_timer: Timer::from_seconds(time_per_tick, TimerMode::Repeating),
            tick_processed: 0,

            messages: MessageQueue::default(),
        }
    }

    pub fn with_player(mut self, player: (String, Entity, ClientType)) -> Self {
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

    pub fn remove_projectile(&mut self, projectile: Entity) {
        self.projectiles.retain(|&p| p != projectile);
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
    mut player_immediate_message_queues: Query<&mut ImmediateOutMessageQueue>,
) {
    let PlayerWantsToJoinLobbyTrigger {
        player,
        lobby: lobby_entity,
        player_type,
        team_name,
        player_name,
    } = trigger.event();

    if let Ok(mut lobby) = lobby_management.get_lobby_mut(*lobby_entity) {
        let mut queue = player_immediate_message_queues.get_mut(*player).unwrap();

        match lobby.state {
            LobbyState::InProgress | LobbyState::Finished => {
                error!(
                    "Player {:?} wants to join lobby {:?} but it is in state {:?}",
                    player, lobby_entity, lobby.state
                );
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
                    lobby
                        .players
                        .push((player_name.clone(), *player, player_type.clone()));

                    match lobby
                        .map_config
                        .as_mut()
                        .expect("Map config should be set up by now")
                        .insert_player_into_team(team_name, *player)
                    {
                        Ok(_) => {
                            commands
                                .entity(*player)
                                .insert((InTeam(team_name.clone()),));

                            queue.push_back(MessageContainer::new(
                                MessageTarget::Client(*player),
                                NetworkMessageType::SuccessFullyJoinedLobby(TextDataWrapper::new(
                                    format!("Successfully joined lobby on team {}", team_name),
                                )),
                            ));
                        }
                        Err(err) => {
                            error!("Failed to add player to team {}: {:?}", team_name, err);
                            queue.push_back(MessageContainer::new(
                                MessageTarget::Client(*player),
                                NetworkMessageType::MessageError(err),
                            ));

                            return;
                        }
                    }
                } else {
                    error!("Player wants to join lobby without specifying a team name");
                    queue.push_back(MessageContainer::new(
                        MessageTarget::Client(*player),
                        NetworkMessageType::MessageError(ErrorMessageTypes::LobbyManagementError(
                            "Player wants to join lobby without specifying a team name".to_string(),
                        )),
                    ));

                    return;
                }
            }
            ClientType::Spectator => {
                lobby.spectators.push(*player);
            }
            ClientType::Dummy => unimplemented!("Dummy clients should not be able to join lobbies"),
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
        if let Some(map_config) = map_config.get_map_config_from_name(&lobby.map_name) {
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
