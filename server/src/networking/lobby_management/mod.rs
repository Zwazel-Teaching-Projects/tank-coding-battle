use bevy::{prelude::*, utils::HashMap};
use handle_first_contact::{handle_awaiting_first_contact, handle_first_contact_message};
use lobby_management::LobbyManagementSystemParam;
use shared::{asset_handling::maps::MapConfig, networking::networking_state::MyNetworkingState};

use super::handle_clients::lib::ClientHasBeenDespawnedTrigger;

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
            .add_observer(despawn_lobby_if_empty);
    }
}

#[derive(Debug, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct InLobby(pub Entity);

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct MyLobbies {
    pub lobbies: HashMap<String, Entity>,
}

#[derive(Debug, Reflect, Default, Component)]
#[reflect(Component)]
pub struct MyLobby {
    pub name: String,
    pub players: Vec<Entity>,
    pub map_name: String,

    pub map_config: Option<MapConfig>,
}

impl MyLobby {
    pub fn new(name: String, map_name: String) -> Self {
        Self {
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

fn despawn_lobby_if_empty(
    _: Trigger<ClientHasBeenDespawnedTrigger>,
    mut lobby_management: LobbyManagementSystemParam,
    mut commands: Commands,
) {
    lobby_management.cleanup_lobbies(&mut commands);
}
