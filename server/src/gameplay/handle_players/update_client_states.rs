use bevy::prelude::*;
use shared::{
    game::game_state::PersonalizedClientGameState,
    networking::lobby_management::{lobby_management::LobbyManagementSystemParam, InTeam},
};

use crate::gameplay::triggers::UpdateClientGameStatesTrigger;

/// Based on game world, lobby state, and other things, update the personalized state of each client
/// So that they only know what they should know
pub fn update_client_states(
    trigger: Trigger<UpdateClientGameStatesTrigger>,
    lobby_management: LobbyManagementSystemParam,
    clients: Query<&InTeam>,
    mut states: Query<&mut PersonalizedClientGameState>,
) {
    let client_entity = trigger.entity();
    let in_team = clients.get(client_entity).expect("Failed to get in team");
    let lobby = trigger.lobby;
    let mut client_state = states
        .get_mut(client_entity)
        .expect("Failed to get client state");
    let my_lobby = lobby_management
        .get_lobby(lobby)
        .expect("Failed to get lobby");
    // Get all teammates, filtering out myself
    let team_players = my_lobby
        .get_team(in_team)
        .expect("Failed to get team")
        .iter()
        .filter(|entity| **entity != client_entity)
        .collect::<Vec<_>>();

    let lobby_state = lobby_management
        .get_lobby_gamestate(lobby)
        .expect("Failed to get lobby state");

    // Clearing all states, as we might not know what we knew before
    // Only clears the non-persistent information (like transform)
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

    // Adding our teammates' states to the state, as we definitely know them
    team_players.iter().for_each(|entity| {
        lobby_state
            .client_states
            .iter()
            .for_each(|(state_entity, state)| {
                if state_entity == *entity {
                    client_state
                        .other_client_states
                        .insert(**entity, Some(state.clone()));
                    return;
                }
            });
    });

    // TODO: Add enemies' states to the state, as we might know them

    // Updating the tick
    client_state.tick = lobby_state.tick;
}
