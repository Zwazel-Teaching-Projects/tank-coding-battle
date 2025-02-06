use std::io::Read;

use bevy::prelude::*;
use shared::networking::messages::message_container::MessageContainer;

use crate::{
    gameplay::handle_players::team_handling::InTeam,
    networking::{
        handle_clients::lib::{ClientDisconnectedTrigger, MyNetworkClient},
        lobby_management::{lobby_management::LobbyManagementSystemParam, InLobby, MyLobby},
    },
};

pub fn handle_reading_messages(
    mut commands: Commands,
    mut clients: Query<(
        Entity,
        &mut MyNetworkClient,
        Option<&InLobby>,
        Option<&InTeam>,
    )>,
    lobbies: Query<(Entity, &MyLobby)>,
    lobby_management: LobbyManagementSystemParam,
) {
    for (entity, mut network_client, in_lobby, in_team) in clients.iter_mut() {
        let addr = network_client.address;
        let stream = &mut network_client.stream;

        // First, read the 4-byte length prefix
        let mut len_buf = [0u8; 4];
        if let Err(e) = stream.read_exact(&mut len_buf) {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                continue;
            } else {
                error!("Error reading length prefix from {}: {}", addr, e);
                commands.trigger(ClientDisconnectedTrigger(entity));
                continue;
            }
        }
        let msg_len = u32::from_be_bytes(len_buf) as usize;

        if msg_len == 0 {
            // No message to read
            continue;
        }

        // Allocate a buffer to hold the entire message
        let mut buf = vec![0u8; msg_len];
        if let Err(e) = stream.read_exact(&mut buf) {
            error!(
                "Read error: failed to fill whole buffer from {}: {}",
                addr, e
            );
            commands.trigger(ClientDisconnectedTrigger(entity));
            continue;
        }

        // Convert buffer to UTF-8 string
        let received = match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => {
                error!("UTF-8 conversion error from {}: {}", addr, e);
                continue;
            }
        };

        // Deserialize the JSON into your MessageContainer
        match serde_json::from_str::<MessageContainer>(&received) {
            Ok(mut message_container) => {
                message_container.sender = Some(entity);

                info!(
                    "Received message from client \"{:?}\":\n{:?}",
                    addr, message_container
                );

                let result = message_container.trigger_message_received(&mut commands, &lobby_management);

                if let Err(e) = result {
                    error!(
                        "Failed to handle message from client \"{:?}\":\n{:?}",
                        addr, e
                    );
                }
            }
            Err(e) => {
                error!(
                    "Failed to parse JSON from {}: {}. Raw data: {}",
                    addr, e, received
                );
            }
        }
    }
}
