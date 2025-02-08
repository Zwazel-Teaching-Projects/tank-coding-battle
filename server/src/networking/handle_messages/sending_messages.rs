use std::io::Write;

use bevy::prelude::*;
use shared::networking::{
    lobby_management::lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
    messages::message_queue::OutMessageQueue,
};

use crate::{
    gameplay::triggers::SendOutgoingMessagesTrigger,
    networking::handle_clients::lib::MyNetworkClient,
};

pub fn sending_client_messages(
    trigger: Trigger<SendOutgoingMessagesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut connected_clients: Query<(&mut MyNetworkClient, &mut OutMessageQueue)>,
) {
    let lobby = trigger.entity();

    match lobby_management.targets_get_players_in_lobby(LobbyManagementArgument {
        lobby: Some(lobby),
        ..default()
    }) {
        Ok(players_in_lobby) => {
            for player in players_in_lobby {
                let (mut client, mut out_message_queue) = connected_clients
                    .get_mut(player)
                    .expect("Failed to get client");
                let stream = &mut client.stream;

                for message in out_message_queue.drain(..) {
                    let message =
                        serde_json::to_vec(&message).expect("Failed to serialize message");
                    let length = (message.len() as u32).to_le_bytes();

                    let _ = stream.write_all(&length).expect("Failed to send length");
                    let _ = stream.write_all(&message).expect("Failed to send message");
                }
            }
        }
        Err(err) => error!("Failed to get players in lobby: {}", err),
    }
}

// TODO
pub fn broadcast_lobby_messages(
    trigger: Trigger<SendOutgoingMessagesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut connected_clients: Query<(&mut MyNetworkClient, &mut OutMessageQueue)>,
) {
    let lobby = trigger.entity();

    match lobby_management.targets_get_players_in_lobby(LobbyManagementArgument {
        lobby: Some(lobby),
        ..default()
    }) {
        Ok(players_in_lobby) => {
            let lobby = lobby_management
                .get_lobby(lobby)
                .expect("Failed to get lobby");
            info!(
                "Sending out all messages that are in the Message Queues of players in lobby: {:?}",
                lobby.lobby_name
            );
            for player in players_in_lobby {
                let (mut client, mut out_message_queue) = connected_clients
                    .get_mut(player)
                    .expect("Failed to get client");
                let stream = &mut client.stream;

                for message in out_message_queue.drain(..) {
                    let message =
                        serde_json::to_vec(&message).expect("Failed to serialize message");
                    let length = (message.len() as u32).to_le_bytes();

                    let _ = stream.write_all(&length).expect("Failed to send length");
                    let _ = stream.write_all(&message).expect("Failed to send message");
                }
            }
        }
        Err(err) => error!("Failed to get players in lobby: {}", err),
    }
}
