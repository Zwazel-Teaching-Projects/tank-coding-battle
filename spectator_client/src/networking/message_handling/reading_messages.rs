use std::io::Read;

use bevy::prelude::*;
use shared::networking::messages::message_container::MessageContainer;

use crate::networking::MyNetworkStream;

pub fn reading_messages(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut MyNetworkStream)>,
) {
    for (entity, mut stream) in clients.iter_mut() {
        // First, read the 4-byte length prefix
        let mut len_buf = [0u8; 4];
        if let Err(e) = stream.read_exact(&mut len_buf) {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                continue;
            } else {
                error!("Error reading length prefix: {}", e);
                continue;
            }
        }
        let msg_len = u32::from_le_bytes(len_buf) as usize;

        if msg_len == 0 {
            // No message to read
            continue;
        }

        // Allocate a buffer to hold the entire message
        let mut buf = vec![0u8; msg_len];
        if let Err(e) = stream.read_exact(&mut buf) {
            error!("Read error: failed to fill whole buffer: {}", e);
            continue;
        }

        // Convert the buffer to a string
        let message = match String::from_utf8(buf) {
            Ok(message) => message,
            Err(e) => {
                error!("Failed to convert buffer to string: {}", e);
                continue;
            }
        };

        info!("Received message (JSON): {}", message);

        // Deserialize the message into a MessageContainer
        match serde_json::from_str::<Vec<MessageContainer>>(&message) {
            Ok(message_containers) => {
                for message_container in message_containers {
                    match message_container.trigger_message_received_client(&mut commands, entity) {
                        Ok(_) => {
                            info!("Received message (Deserialized): {:?}", message_container);
                        }
                        Err(e) => {
                            error!("Failed to handle message: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to deserialize message: {}", e);
            }
        }
    }
}
