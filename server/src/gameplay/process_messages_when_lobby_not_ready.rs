use bevy::prelude::*;
use shared::networking::{
    lobby_management::{
        lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
        InTeam, LobbyState,
    },
    messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_queue::{ImmediateOutMessageQueue, OutMessageQueue},
    },
};

pub fn process_messages_before_lobby_is_ready(
    mut lobby_management: LobbyManagementSystemParam,
    mut commands: Commands,
    mut outgoing_message_queues: Query<&mut OutMessageQueue>,
    mut immediate_message_queues: Query<&mut ImmediateOutMessageQueue>,
    client: Query<&InTeam>,
) {
    // Get all lobbies (they're stored in a hashmap<string,entity>), i only want entity. owned
    let lobbies = lobby_management
        .lobby_resource
        .lobbies
        .iter()
        .map(|(_lobby_name, lobby_entity)| *lobby_entity)
        .collect::<Vec<Entity>>();

    for lobby_entity in lobbies.iter() {
        match lobby_management
            .get_lobby(*lobby_entity)
            .expect("Lobby not found")
            .1
            .state
        {
            LobbyState::InProgress => {
                // If the lobby is in progress, we don't want to process messages
                continue;
            }
            _ => {}
        }

        let messages_to_process = lobby_management
            .get_lobby_mut(*lobby_entity)
            .expect("Lobby not found")
            .1
            .messages
            .drain(..)
            .collect::<Vec<_>>();

        let (_, lobby, _) = lobby_management
            .get_lobby(*lobby_entity)
            .expect("Lobby not found");

        for message_container in messages_to_process {
            match message_container.message {
                NetworkMessageType::StartGame(_) => {
                    // received messages that are allowed before lobby is ready
                }
                _ => {
                    // Received unhandled message, skip it
                    warn!(
                                "Received not allowed message before lobby \"{}\" is ready:\n\t{:?}\n\tdropping it",
                                lobby.lobby_name, message_container
                            );

                    continue;
                }
            }

            let lobby_arg = LobbyManagementArgument {
                lobby: Some(*lobby_entity),
                sender: message_container.sender,
                target_player: match message_container.target {
                    MessageTarget::Client(e) => Some(e),
                    _ => None,
                },
                team_name: client
                    .get(message_container.sender.unwrap())
                    .ok()
                    .map(|t| t.0.clone()),
                sender_state: None,
            };

            let result = message_container.trigger_message_received(
                &mut commands,
                &lobby_management,
                lobby_arg,
                &mut outgoing_message_queues,
            );

            if let Err(e) = result {
                error!("Failed to handle message before lobby is ready:\n\tError: {:?}\n\tMessage: {:?}", e, message_container);

                let sender = message_container.sender.expect("Failed to get sender");
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
