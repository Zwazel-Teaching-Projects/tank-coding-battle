use bevy::prelude::*;
use shared::{
    asset_handling::config::spectator_client_config::ClientConfigSystemParam,
    networking::messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_data::first_contact::{ClientType, FirstContactData},
        message_queue::ImmediateOutMessageQueue,
    },
};

use super::MyNetworkStream;

pub fn send_first_contact(
    trigger: Trigger<OnAdd, MyNetworkStream>,
    client_config: ClientConfigSystemParam,
    mut message: Query<&mut ImmediateOutMessageQueue>,
) {
    let client = trigger.entity();
    let client_config = client_config.client_config();

    if let Ok(mut message) = message.get_mut(client) {
        message.push_front(MessageContainer::new(
            MessageTarget::ServerOnly,
            NetworkMessageType::FirstContact(FirstContactData {
                client_type: ClientType::Spectator,
                bot_name: client_config.name.clone(),
                map_name: Some(client_config.map.clone()),
                lobby_name: client_config.lobby_name.clone(),
                ..default()
            }),
        ));
    }
}
