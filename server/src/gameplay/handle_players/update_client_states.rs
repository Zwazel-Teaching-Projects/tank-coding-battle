use bevy::prelude::*;
use shared::{
    game::game_state::PersonalizedClientGameState,
    networking::lobby_management::lobby_management::LobbyManagementSystemParam,
};

use crate::gameplay::triggers::UpdateClientGameStatesTrigger;

pub fn update_client_states(
    trigger: Trigger<UpdateClientGameStatesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut states: Query<&mut PersonalizedClientGameState>,
) {
    let client_entity = trigger.entity();
    let lobby = trigger.lobby;
    let mut state = states
        .get_mut(client_entity)
        .expect("Failed to get client state");
    let lobby_state = lobby_management
        .get_lobby_gamestate(lobby)
        .expect("Failed to get lobby state");

    state.tick = lobby_state.tick;

    // TODO: Handle other state updates!!!!
    info!("Updated client state for client: {}", client_entity);
}
