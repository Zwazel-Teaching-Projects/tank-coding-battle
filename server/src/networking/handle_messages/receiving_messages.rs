use std::io::Read;

use bevy::prelude::*;
use shared::networking::{
    lobby_management::{
        lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
        InLobby, InTeam,
    },
    messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_queue::{ImmediateOutMessageQueue, OutMessageQueue},
    },
};

use crate::networking::handle_clients::lib::{ClientDisconnectedTrigger, MyNetworkClient};

pub fn handle_reading_messages(
    mut commands: Commands,
    mut clients: Query<(
        Entity,
        &mut MyNetworkClient,
        Option<&InLobby>,
        Option<&InTeam>,
    )>,
    mut outgoing_message_queues: Query<&mut OutMessageQueue>,
    mut immediate_message_queues: Query<&mut ImmediateOutMessageQueue>,
    mut lobby_management: LobbyManagementSystemParam,
) {
    for (sender, mut network_client, in_lobby, in_team) in clients.iter_mut() {
        let addr = network_client.get_address();
        if let Some(stream) = &mut network_client.stream {
            // Read the 4-byte length prefix for the payload length
            let mut len_buf = [0u8; 4];
            if let Err(e) = stream.read_exact(&mut len_buf) {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                } else {
                    error!("Error reading length prefix from {:?}: {}", addr, e);
                    commands.trigger(ClientDisconnectedTrigger(sender));
                    continue;
                }
            }
            let msg_len = u32::from_be_bytes(len_buf) as usize;
            if msg_len == 0 {
                // No message to read
                continue;
            }

            // Read the actual message payload into a buffer
            let mut buf = vec![0u8; msg_len];
            if let Err(e) = stream.read_exact(&mut buf) {
                error!(
                    "Read error: failed to fill whole buffer from {:?}: {}",
                    addr, e
                );
                commands.trigger(ClientDisconnectedTrigger(sender));
                continue;
            }

            // Convert buffer to UTF-8 string
            let received = match String::from_utf8(buf) {
                Ok(s) => s,
                Err(e) => {
                    error!("UTF-8 conversion error from {:?}: {}", addr, e);
                    continue;
                }
            };

            // Deserialize the JSON into an array of MessageContainers
            match serde_json::from_str::<Vec<MessageContainer>>(&received) {
                Ok(mut messages) => {
                    // Clear duplicate messages of types marked as "unique". only keeping the latest one.
                    clear_duplicate_unique_messages(&mut messages);

                    for message_container in messages.iter_mut() {
                        message_container.sender = Some(sender);
                        // If we're in the lobby, add all messages to the lobby's message queue, so we can process them in the correct moment. expecting all non-server-only messages
                        if let Some(in_lobby) = in_lobby {
                            // Set the received tick to the current tick of the lobby
                            message_container.tick_received = lobby_management
                                .get_lobby_gamestate(**in_lobby)
                                // TODO Replace with adding error to queue, not panicking
                                .expect("Failed to get lobby game state")
                                .tick;
                            message_container.tick_to_be_processed_at =
                                message_container.tick_received + 1;

                            // Add message to the lobby's message queue
                            lobby_management
                                .get_lobby_mut(**in_lobby)
                                .expect("Failed to get lobby messages")
                                .1
                                .messages
                                // TODO Replace with adding error to queue, not panicking
                                .push_back(message_container.clone());
                        } else {
                            // If we're not in the lobby, add the message to the immediate message queue. expecting server only messages
                            let lobby_arg = LobbyManagementArgument {
                                lobby: in_lobby.map(|l| **l),
                                sender: Some(sender),
                                target_player: match message_container.target {
                                    MessageTarget::Client(e) => Some(e),
                                    _ => None,
                                },
                                team_name: in_team.map(|t| t.0.clone()),
                                sender_state: None,
                            };

                            let result = message_container.trigger_message_received(
                                &mut commands,
                                &lobby_management,
                                lobby_arg,
                                &mut outgoing_message_queues,
                            );

                            if let Err(e) = result {
                                error!(
                                    "Failed to handle message from client \"{:?}\":\n{:?}",
                                    addr, e
                                );

                                let mut error_queue = immediate_message_queues
                                    .get_mut(sender)
                                    // TODO Replace with adding error to queue, not panicking
                                    .expect("Failed to get outgoing message queue from sender");
                                error_queue.push_back(MessageContainer::new(
                                    MessageTarget::Client(sender),
                                    NetworkMessageType::MessageError(e),
                                ));
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to parse JSON array from {:?}: {}. Raw data: {}",
                        addr, e, received
                    );
                    // TODO add error message to queue
                }
            }
        } else {
            // No Stream, it's a dummy client
        }
    }
}

// Clear duplicate messages of types marked as "unique". only keeping the latest one.
// call .is_unique() on the message container directly.
// store the .message.message (is an enum). then delete any occurring duplicates.
// So we have to iterate backwards, so we first get the latest message and then remove the older ones.
fn clear_duplicate_unique_messages(messages: &mut Vec<MessageContainer>) {
    let mut unique_messages = Vec::new();
    for message in messages.iter().rev() {
        if message.is_unique() {
            if !unique_messages
                .iter()
                .any(|m: &MessageContainer| m.message == message.message)
            {
                unique_messages.push(message.clone());
            }
        } else {
            unique_messages.push(message.clone());
        }
    }
    unique_messages.reverse();
    *messages = unique_messages;
}
