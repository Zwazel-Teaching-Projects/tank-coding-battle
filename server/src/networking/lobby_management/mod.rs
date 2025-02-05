use bevy::{prelude::*, utils::HashMap};
use handle_first_contact::{handle_awaiting_first_contact, handle_first_contact_message};
use lobby_management::LobbyManagementSystemParam;
use shared::{
    asset_handling::{
        config::ServerConfigSystemParam,
        maps::{MapConfig, MapConfigSystemParam},
    },
    networking::networking_state::MyNetworkingState,
};

use super::handle_clients::lib::AwaitingFirstContact;

pub mod handle_first_contact;
pub mod lobby_management;

pub struct MyLobbyManagementPlugin;

impl Plugin for MyLobbyManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_first_contact_message)
            .add_systems(
                Update,
                (handle_awaiting_first_contact.run_if(in_state(MyNetworkingState::Running)),),
            )
            .register_type::<MyLobbies>()
            .init_resource::<MyLobbies>()
            .register_type::<MyLobby>()
            .register_type::<InLobby>()
            .register_type::<LobbyState>()
            .add_observer(finish_setting_up_lobby);
    }
}

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct InLobby(pub Entity);

#[derive(Debug, Event)]
pub struct PlayerRemovedFromLobbyTrigger;

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct MyLobbies {
    pub lobbies: HashMap<String, Entity>,
}

#[derive(Debug, Reflect, Default, Component, PartialEq)]
#[reflect(Component)]
pub struct MyLobby {
    pub state: LobbyState,

    pub name: String,
    pub players: Vec<Entity>,
    pub map_name: String,

    pub map_config: Option<MapConfig>,
}

impl MyLobby {
    pub fn new(name: String, map_name: String) -> Self {
        Self {
            state: LobbyState::default(),

            name,
            players: Vec::new(),
            map_name,

            map_config: None,
        }
    }

    pub fn with_player(mut self, player: Entity) -> Self {
        self.players.push(player);
        self
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
        .insert(AwaitingFirstContact::new(
            server_config.timeout_first_contact,
        ));
}

fn finish_setting_up_lobby(
    trigger: Trigger<OnAdd, MyLobby>,
    mut lobby_management: LobbyManagementSystemParam,
    map_config: MapConfigSystemParam,
    mut commands: Commands,
) {
    let (lobby_entity, mut lobby) = lobby_management.get_lobby_mut(trigger.entity()).unwrap();
    if lobby.map_config.is_none() {
        if let Some(map_config) = map_config.get_map_config(&lobby.map_name) {
            info!(
                "Adding map config \"{}\" to lobby \"{}\"",
                lobby.map_name, lobby.name
            );

            lobby.map_config = Some(map_config.clone());

            lobby.players.iter().for_each(|&player| {
                commands
                    .entity(player)
                    .remove::<AwaitingFirstContact>()
                    .insert(InLobby(lobby_entity));
            });
        } else {
            error!(
                "Failed to get map config for lobby \"{}\" with map name \"{}\"",
                lobby.name, lobby.map_name
            );
            lobby_management.remove_lobby(lobby_entity, &mut commands);
        }
    }
}
