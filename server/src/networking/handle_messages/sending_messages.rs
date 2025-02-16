use std::io::Write;

use bevy::prelude::*;
use shared::networking::{
    lobby_management::lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
    messages::message_queue::{ImmediateOutMessageQueue, OutMessageQueue},
};

use crate::{
    gameplay::triggers::SendOutgoingMessagesTrigger,
    networking::handle_clients::lib::MyNetworkClient,
};

pub fn sending_immediate_messages(
    mut connected_clients: Query<
        (&mut MyNetworkClient, &mut ImmediateOutMessageQueue),
        Changed<ImmediateOutMessageQueue>,
    >,
) {
    for (mut client, mut immediate_message_queue) in connected_clients.iter_mut() {
        let messages: Vec<_> = immediate_message_queue.drain(..).collect();
        if let Some(stream) = &mut client.stream {
            if !messages.is_empty() {
                let messages = serde_json::to_vec(&messages).expect("Failed to serialize messages");
                let length = (messages.len() as u32).to_le_bytes();

                let _ = stream.write_all(&length).expect("Failed to send length");
                let _ = stream
                    .write_all(&messages)
                    .expect("Failed to send messages");
            }
        } else {
            // Is a dummy client
        }
    }
}

pub fn sending_client_messages(
    trigger: Trigger<SendOutgoingMessagesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut connected_clients: Query<(&mut MyNetworkClient, &mut OutMessageQueue)>,
) {
    let lobby = trigger.entity();

    match lobby_management.targets_get_players_and_spectators_in_lobby(LobbyManagementArgument {
        lobby: Some(lobby),
        ..default()
    }) {
        Ok(clients_in_lobby) => {
            let game_state = lobby_management
                .get_lobby_gamestate(lobby)
                .expect("Failed to get game state");
            for player in clients_in_lobby {
                let (mut client, mut out_message_queue) = connected_clients
                    .get_mut(player)
                    .expect("Failed to get client");

                let mut messages: Vec<_> = out_message_queue.drain(..).collect();
                if let Some(stream) = &mut client.stream {
                    for message in &mut messages {
                        message.tick_sent = game_state.tick;
                    }

                    if !messages.is_empty() {
                        info!("Sending messages to player: {:?}", messages);

                        let messages =
                            serde_json::to_vec(&messages).expect("Failed to serialize messages");

                        let length = (messages.len() as u32).to_le_bytes();

                        let _ = stream.write_all(&length).expect("Failed to send length");
                        let _ = stream
                            .write_all(&messages)
                            .expect("Failed to send messages");
                    }
                } else {
                    // Is a dummy client
                }
            }
        }
        Err(err) => error!("Failed to get players in lobby: {}", err),
    }
}
