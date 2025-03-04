use bevy::prelude::*;
use shared::{
    game::player_handling::PlayerState,
    networking::{
        lobby_management::{
            lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
            InTeam,
        },
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_queue::{ImmediateOutMessageQueue, OutMessageQueue},
        },
    },
};

use crate::gameplay::triggers::MovePorjectilesSimulationStepTrigger;

use super::triggers::CollectAndTriggerMessagesTrigger;

pub fn process_lobby_messages(
    trigger: Trigger<CollectAndTriggerMessagesTrigger>,
    mut lobby_management: LobbyManagementSystemParam,
    mut commands: Commands,
    mut outgoing_message_queues: Query<&mut OutMessageQueue>,
    mut immediate_message_queues: Query<&mut ImmediateOutMessageQueue>,
    client: Query<(&InTeam, Option<&PlayerState>)>,
) {
    let lobby_entity = trigger.entity();

    let messages_to_process = lobby_management
        .get_lobby_mut(lobby_entity)
        .expect("Lobby not found")
        .messages
        .drain(..)
        .collect::<Vec<_>>();

    let current_lobby_state = lobby_management
        .get_lobby_gamestate(lobby_entity)
        .expect("Lobby Gamestate not found");

    for message_container in messages_to_process {
        if message_container.tick_to_be_processed_at > current_lobby_state.tick {
            warn!(
                "Dropping message because it's too old: {:?}",
                message_container
            );
            continue;
        }

        let (client_team_name, player_state) = client
            .get(message_container.sender.expect("Message sender not found"))
            .expect("Client not found");
        let sender = message_container.sender.expect("Message sender not found");
        let lobby_arg = LobbyManagementArgument {
            lobby: Some(lobby_entity),
            sender: Some(sender),
            target_player: match message_container.target {
                MessageTarget::Client(e) => Some(e),
                _ => None,
            },
            team_name: Some(client_team_name.0.clone()),
            sender_state: player_state.cloned(),
        };

        let result = message_container.trigger_message_received(
            &mut commands,
            &lobby_management,
            lobby_arg,
            &mut outgoing_message_queues,
        );

        if let Err(e) = result {
            error!(
                "Failed to handle message before lobby is ready:\n\tError: {:?}\n\tMessage: {:?}",
                e, message_container
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

    commands.trigger_targets(MovePorjectilesSimulationStepTrigger, lobby_entity);
}
