use bevy::prelude::*;
use handle_first_contact::{handle_awaiting_first_contact, handle_first_contact_message};
use shared::networking::networking_state::MyNetworkingState;

pub mod handle_first_contact;

pub struct MyLobbyManagement;

impl Plugin for MyLobbyManagement {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_first_contact_message).add_systems(
            Update,
            handle_awaiting_first_contact.run_if(in_state(MyNetworkingState::Running)),
        );
    }
}
