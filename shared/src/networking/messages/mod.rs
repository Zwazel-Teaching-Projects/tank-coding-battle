use bevy::prelude::*;

pub mod message_container;
pub mod message_data;
pub mod message_targets;

pub struct MySharedNetworkMessagesPlugin;

impl Plugin for MySharedNetworkMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<message_container::MessageContainer>()
            .register_type::<message_container::NetworkMessageType>()
            .register_type::<message_targets::MessageTarget>()
            .add_plugins(message_data::MySharedMessageDataPlugin);
    }
}
