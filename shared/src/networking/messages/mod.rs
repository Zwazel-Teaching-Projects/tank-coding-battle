use bevy::prelude::*;

pub mod handle_deserialization;
pub mod handle_serialization;
pub mod message_container;
pub mod message_data;
pub mod message_queue;
pub mod message_targets;
pub mod message_types;

pub struct MySharedNetworkMessagesPlugin;

impl Plugin for MySharedNetworkMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<message_queue::OutgoingMessageQueue>()
            .init_resource::<message_queue::OutgoingMessageQueue>()
            .register_type::<message_container::MessageContainer>()
            .register_type::<message_types::NetworkMessageType>()
            .register_type::<message_targets::MessageTarget>()
            .add_plugins(message_data::MySharedMessageDataPlugin);
    }
}
