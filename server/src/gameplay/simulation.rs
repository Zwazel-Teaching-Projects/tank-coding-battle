use bevy::prelude::*;
use shared::networking::lobby_management::MyLobby;

use crate::gameplay::triggers::NextSimulationStepDoneTrigger;

use super::triggers::StartNextSimulationStepTrigger;

pub fn run_next_simulation_tick(
    trigger: Trigger<StartNextSimulationStepTrigger>,
    lobbies: Query<&MyLobby>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobbies.get(lobby_entity).unwrap();

    info!(
        "Running simulation tick {} for lobby: {}",
        lobby.game_state.tick, lobby.lobby_name
    );

    commands.trigger_targets(NextSimulationStepDoneTrigger, lobby_entity);
}
