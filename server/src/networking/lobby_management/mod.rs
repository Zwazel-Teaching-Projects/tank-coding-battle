use bevy::{prelude::*, utils::HashMap};
use handle_first_contact::{handle_awaiting_first_contact, handle_first_contact_message};
use shared::networking::networking_state::MyNetworkingState;

pub mod handle_first_contact;

pub struct MyLobbyManagementPlugin;

impl Plugin for MyLobbyManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_first_contact_message)
            .add_systems(
                Update,
                handle_awaiting_first_contact.run_if(in_state(MyNetworkingState::Running)),
            )
            .register_type::<MyLobbies>()
            .init_resource::<MyLobbies>()
            .register_type::<MyLobby>()
            .register_type::<InLobby>();
    }
}

#[derive(Debug, Reflect, Component)]
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
}
