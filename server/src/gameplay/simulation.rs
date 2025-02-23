use bevy::prelude::*;

use crate::gameplay::triggers::{FinishedNextSimulationStepTrigger, UpdateLobbyGameStateTrigger};

use super::triggers::StartNextSimulationStepTrigger;

pub fn process_tick_sim(trigger: Trigger<StartNextSimulationStepTrigger>, mut commands: Commands) {
    let lobby_entity = trigger.entity();

    commands.trigger_targets(FinishedNextSimulationStepTrigger, lobby_entity);
}

pub fn process_tick_sim_finished(
    trigger: Trigger<FinishedNextSimulationStepTrigger>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();

    commands.trigger_targets(UpdateLobbyGameStateTrigger, lobby_entity);
}
