use bevy::prelude::*;
use receiving_messages::handle_reading_messages;
use sending_messages::sending_messages;
use shared::networking::{
    messages::message_container::FirstContactTrigger, networking_system_sets::MyNetworkingSet,
};

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
            )
            .add_observer(handle_first_contact_message);
    }
}

// Proof of concept for handling a message using an observer
// We can even make targeted ones and only trigger for specific clients!
fn handle_first_contact_message(trigger: Trigger<FirstContactTrigger>) {
    info!("Received first contact message: {:?}", trigger.0);
}
