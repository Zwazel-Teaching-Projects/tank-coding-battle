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
    lobby_management: LobbyManagementSystemParam,
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
                    for message_container in messages.iter_mut() {
                        message_container.sender = Some(sender);
                        if let Some(in_lobby) = in_lobby {
                            message_container.tick_received = lobby_management
                                .get_lobby_gamestate(**in_lobby)
                                // TODO Replace with adding error to queue, not panicking
                                .expect("Failed to get lobby game state")
                                .tick;
                        }

                        info!(
                            "Received message from client \"{:?}\":\n{:?}",
                            addr, message_container
                        );

                        let lobby_arg = LobbyManagementArgument {
                            lobby: in_lobby.map(|l| **l),
                            sender: Some(sender),
                            target_player: match message_container.target {
                                MessageTarget::Client(e) => Some(e),
                                _ => None,
                            },
                            team_name: in_team.map(|t| t.0.clone()),
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
