use bevy::prelude::*;
use shared::{game::game_state::LobbyGameState, networking::lobby_management::MyLobby};

use crate::gameplay::triggers::UpdateLobbyGameStateTrigger;

use super::triggers::StartNextSimulationStepTrigger;

pub fn process_tick_sim(
    trigger: Trigger<StartNextSimulationStepTrigger>,
    lobbies: Query<(&MyLobby, &LobbyGameState)>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let (lobby, game_state) = lobbies.get(lobby_entity).unwrap();

    info!(
        "Running simulation tick {} for lobby: {}",
        game_state.tick, lobby.lobby_name
    );

    commands.trigger_targets(UpdateLobbyGameStateTrigger, lobby_entity);
}
