use std::io::Write;

use bevy::prelude::*;
use shared::networking::messages::message_queue::ImmediateOutMessageQueue;

use crate::networking::MyNetworkStream;

pub fn sending_messages(
    mut client: Query<
        (&mut MyNetworkStream, &mut ImmediateOutMessageQueue),
        Changed<ImmediateOutMessageQueue>,
    >,
) {
    for (mut stream, mut immediate_message_queue) in client.iter_mut() {
        // Collect all messages into a vector
        let messages: Vec<_> = immediate_message_queue.drain(..).collect();
        // Serialize the vector of messages
        let message_bytes = serde_json::to_vec(&messages).expect("Failed to serialize messages");
        let length = (message_bytes.len() as u32).to_be_bytes();
        let _ = stream.write_all(&length).expect("Failed to send length");
        let _ = stream
            .write_all(&message_bytes)
            .expect("Failed to send messages");
    }
}
