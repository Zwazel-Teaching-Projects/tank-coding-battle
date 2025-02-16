use bevy::prelude::*;
use shared::{
    game::game_state::PersonalizedClientGameState,
    networking::lobby_management::lobby_management::LobbyManagementSystemParam,
};

use crate::gameplay::triggers::UpdateClientGameStatesTrigger;

/// Based on game world, lobby state, and other things, update the personalized state of each client
/// So that they only know what they should know
pub fn update_client_states(
    trigger: Trigger<UpdateClientGameStatesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut states: Query<&mut PersonalizedClientGameState>,
) {
    let client_entity = trigger.entity();
    let lobby = trigger.lobby;
    let mut client_state = states
        .get_mut(client_entity)
        .expect("Failed to get client state");
    let lobby_state = lobby_management
        .get_lobby_gamestate(lobby)
        .expect("Failed to get lobby state");

    // Clearing all states, as we might not know what we knew before
    client_state.clear_states();

    // Adding our own transform to the state, as we definitely know it
    lobby_state
        .client_states
        .iter()
        .for_each(|(entity, state)| {
            if entity == &client_entity {
                client_state.personal_state = state.clone();
                return;
            }
        });

    client_state.tick = lobby_state.tick;

    // TODO: Handle other state updates!!!!
    info!("Updated client state for client: {}", client_entity);
}
