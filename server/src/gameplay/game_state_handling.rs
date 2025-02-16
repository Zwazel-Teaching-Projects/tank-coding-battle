use bevy::prelude::*;
use shared::networking::{
    lobby_management::lobby_management::LobbyManagementSystemParam,
    messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_queue::OutMessageQueue,
    },
};

use super::triggers::{NextSimulationStepDoneTrigger, SendOutgoingMessagesTrigger};

pub fn add_current_game_state_to_message_queue(
    trigger: Trigger<NextSimulationStepDoneTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut out_message_queues: Query<&mut OutMessageQueue>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby");
    let lobby_state = lobby_management
        .get_lobby_gamestate(lobby_entity)
        .expect("Failed to get lobby game state");

    // Sending the game state to all players
    // TODO: Players should get a personalized game state
    for (_, player_entity, _) in lobby.players.iter() {
        let mut out_message_queue = out_message_queues
            .get_mut(*player_entity)
            .expect("Failed to get client");

        let message = MessageContainer::new(
            MessageTarget::Client(*player_entity),
            NetworkMessageType::GameState(lobby_state.clone().into()),
        );

        // Make sure the game state is sent before any other messages
        out_message_queue.push_front(message);
    }

    // Sending the (global) game state to all spectators
    for spectator_entity in lobby.spectators.iter() {
        let mut out_message_queue = out_message_queues
            .get_mut(*spectator_entity)
            .expect("Failed to get client");

        let message = MessageContainer::new(
            MessageTarget::Client(*spectator_entity),
            NetworkMessageType::GameState(lobby_state.clone().into()),
        );

        // Make sure the game state is sent before any other messages
        out_message_queue.push_front(message);
    }

    commands.trigger_targets(SendOutgoingMessagesTrigger, lobby_entity);
}
