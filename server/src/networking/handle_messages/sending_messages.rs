use std::io::Write;

use bevy::prelude::*;
use shared::networking::lobby_management::lobby_management::{
    LobbyManagementArgument, LobbyManagementSystemParam,
};

use crate::{
    gameplay::triggers::SendOutgoingMessagesTrigger,
    networking::handle_clients::lib::MyNetworkClient,
};

pub fn sending_messages(
    trigger: Trigger<SendOutgoingMessagesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut connected_clients: Query<&mut MyNetworkClient>,
) {
    let lobby = trigger.entity();

    match lobby_management.get_players_in_lobby(LobbyManagementArgument {
        lobby: Some(lobby),
        ..default()
    }) {
        Ok(players_in_lobby) => {
            let (_, lobby) = lobby_management
                .get_lobby(LobbyManagementArgument {
                    lobby: Some(lobby),
                    ..default()
                })
                .expect("Failed to get lobby");
            info!(
                "Adding messages to Message qeueues of players in lobby: {:?}",
                lobby.lobby_name
            );
            for player in players_in_lobby {
                let mut client = connected_clients
                    .get_mut(player)
                    .expect("Failed to get client");
                let message_queue =
                    &mut client.outgoing_messages_queue.drain(..).collect::<Vec<_>>();
                let stream = &mut client.stream;

                for message in message_queue.drain(..) {
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
