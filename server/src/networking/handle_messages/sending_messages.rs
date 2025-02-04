use std::io::Write;

use bevy::prelude::*;

use crate::networking::handle_clients::lib::MyNetworkClient;

use super::message_queue::OutgoingMessageQueue;

pub fn sending_messages(
    mut message_queue: ResMut<OutgoingMessageQueue>,
    mut connected_clients: Query<&mut MyNetworkClient>,
) {
    for message in message_queue.messages.drain(..) {
        let message = serde_json::to_vec(&message).expect("Failed to serialize message");
        let length = (message.len() as u32).to_le_bytes();

        // TODO: Based on target, only send to the correct clients

        for mut network_client in connected_clients.iter_mut() {
            let stream = &mut network_client.stream;
            let _ = stream.write_all(&length).expect("Failed to send length");
            let _ = stream.write_all(&message).expect("Failed to send message");
        }
    }
}
