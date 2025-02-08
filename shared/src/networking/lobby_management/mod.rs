use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use lobby_management::LobbyManagementSystemParam;

use crate::{asset_handling::{
    config::ServerConfigSystemParam,
    maps::{MapConfig, MapConfigSystemParam},
}, game::game_state::GameState};

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
            .add_observer(finish_setting_up_lobby);
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
pub struct PlayerAddedToLobbyTrigger;

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct MyLobbies {
    pub lobbies: HashMap<String, Entity>,
}

#[derive(Debug, Reflect, Default, Component, PartialEq)]
#[reflect(Component)]
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
        Self {
            state: LobbyState::default(),
            lobby_name: name,

            players: Vec::new(),
            spectators: Vec::new(),

            map_name,
            map_config: None,

            tick_timer: Timer::from_seconds(1.0 / tick_rate as f32, TimerMode::Repeating),

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

fn finish_setting_up_lobby(
    trigger: Trigger<OnAdd, MyLobby>,
    mut lobby_management: LobbyManagementSystemParam,
    map_config: MapConfigSystemParam,
    mut commands: Commands,
    players: Query<&InTeam>,
) {
    let (lobby_entity, mut lobby) = lobby_management.get_lobby_mut(trigger.entity()).unwrap();
    if lobby.map_config.is_none() {
        if let Some(map_config) = map_config.get_map_config(&lobby.map_name) {
            info!(
                "Adding map config \"{}\" to lobby \"{}\"",
                lobby.map_name, lobby.lobby_name
            );

            lobby.map_config = Some(map_config.clone());

            let player_teams = lobby
                .players
                .iter()
                .map(|&player| {
                    commands
                        .entity(player)
                        .remove::<AwaitingFirstContact>()
                        .insert(InLobby(lobby_entity));

                    commands.trigger_targets(PlayerAddedToLobbyTrigger, lobby_entity);

                    let player_team = players.get(player).unwrap().team_name.clone();
                    (player, player_team)
                })
                .collect::<Vec<_>>();

            for (player, team_name) in player_teams {
                lobby
                    .map_config
                    .as_mut()
                    .unwrap()
                    .insert_player_into_team(&team_name, player);
            }

            // TODO: commands.entity(lobby_entity).observe()
        } else {
            error!(
                "Failed to get map config for lobby \"{}\" with map name \"{}\"",
                lobby.lobby_name, lobby.map_name
            );
            lobby_management.remove_lobby(lobby_entity, &mut commands);
        }
    }
}
