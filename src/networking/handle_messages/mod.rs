use std::io::Read;

use bevy::prelude::*;
use handle_sending::sending_messages;
use lib::QueuedMessages;

use crate::networking::{
    handle_clients::lib::ClientDisconnectedTrigger, shared::lib::MessageContainer,
};

use super::{handle_clients::lib::MyNetworkClient, system_sets::MyNetworkingSet};

mod handle_sending;
pub mod lib;

pub struct HandleMessagesPlugin;

impl Plugin for HandleMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<QueuedMessages>()
            .init_resource::<QueuedMessages>()
            .add_systems(
                Update,
                (handle_client_messages,).in_set(MyNetworkingSet::ReadingMessages),
            )
            .add_systems(
                Update,
                sending_messages.in_set(MyNetworkingSet::SendingMessages),
            );
    }
}

/// Example system that reads data from connected clients.
/// In a real project, you’d parse structured messages, handle disconnections, etc.
fn handle_client_messages(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut MyNetworkClient)>,
) {
    for (entity, mut network_client) in clients.iter_mut() {
        // Non-blocking read attempt
        let addr = network_client.address;
        let stream = &mut network_client.stream;
        let mut len_buf = [0u8; 4];

        match stream.read_exact(&mut len_buf) {
            Ok(()) => {
                let msg_len = u32::from_be_bytes(len_buf) as usize;
                info!(
                    "Expecting message of {} bytes from client: {:?}",
                    msg_len, addr
                );

                let mut buf = vec![0u8; msg_len];
                match stream.read_exact(&mut buf) {
                    Ok(()) => {
                        // Convert the raw bytes into a UTF-8 string.
                        let received = match String::from_utf8(buf) {
                            Ok(s) => s,
                            Err(e) => {
                                error!("UTF-8 conversion error from {}: {}", addr, e);
                                continue;
                            }
                        };
                        info!("Received from client {}: {}", addr, received);

                        // Deserialize the JSON string into a MessageContainer.
                        match serde_json::from_str::<MessageContainer>(&received) {
                            Ok(message_container) => {
                                info!("Successfully parsed JSON: {:?}", message_container);
                                // Process the message_container as needed...
                            }
                            Err(e) => {
                                error!(
                                    "Failed to parse JSON from {}: {}. Raw data: {}",
                                    addr, e, received
                                );
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No more messages to read right now
                    }
                    Err(e) => {
                        error!("Error reading message body from {}: {}", addr, e);
                        commands.trigger(ClientDisconnectedTrigger(entity));
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No more messages to read right now
            }
            Err(e) => {
                // Some other read error
                eprintln!("Read error: {}\nKind: {:?}.", e, e.kind());
                eprintln!("Disconnecting client: {:?}", addr);
                commands.trigger(ClientDisconnectedTrigger(entity));
            }
        }
    }
}
