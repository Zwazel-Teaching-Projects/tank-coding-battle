use bevy::prelude::*;
use receiving_messages::handle_reading_messages;
use sending_messages::{sending_client_messages, sending_error_messages};
use shared::networking::{lobby_management::MyLobby, networking_system_sets::MyNetworkingSet};

pub mod receiving_messages;
pub mod sending_messages;

pub struct HandleMessagesPlugin;

impl Plugin for HandleMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_reading_messages.in_set(MyNetworkingSet::ReadingMessages),
                sending_error_messages.in_set(MyNetworkingSet::SendingMessages),
            ),
        )
        .add_observer(add_triggers_to_lobby);
    }
}

fn add_triggers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(sending_client_messages);
}
