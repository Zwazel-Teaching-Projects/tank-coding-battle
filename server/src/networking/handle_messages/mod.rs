use bevy::prelude::*;
use receiving_messages::handle_reading_messages;
use sending_messages::sending_messages;
use shared::networking::networking_system_sets::MyNetworkingSet;

pub mod message_queue;
pub mod receiving_messages;
pub mod sending_messages;

pub struct HandleMessagesPlugin;

impl Plugin for HandleMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<message_queue::OutgoingMessageQueue>()
            .register_type::<message_queue::OutgoingMessageQueue>()
            .add_systems(
                Update,
                (
                    handle_reading_messages.in_set(MyNetworkingSet::ReadingMessages),
                    sending_messages.in_set(MyNetworkingSet::SendingMessages),
                ),
            );
    }
}
