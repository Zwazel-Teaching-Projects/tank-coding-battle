use bevy::prelude::*;
use shared::{
    game::{game_state::LobbyGameState, player_handling::TankTransform},
    networking::lobby_management::{lobby_management::LobbyManagementSystemParam, MyLobby},
};

use crate::gameplay::triggers::UpdateLobbyGameStateTrigger;

use super::{
    handle_players::dummy_handling::DummyClientMarker, triggers::StartNextSimulationStepTrigger,
};

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

pub fn move_dummies(
    trigger: Trigger<StartNextSimulationStepTrigger>,
    lobby: LobbyManagementSystemParam,
    mut transforms: Query<&mut TankTransform, With<DummyClientMarker>>,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby.get_lobby(lobby_entity).expect("Failed to get lobby");

    for (_, entity, _) in lobby.players.iter() {
        if let Ok(mut transform) = transforms.get_mut(*entity) {
            transform.position.x += 1.0;
        }
    }
}
