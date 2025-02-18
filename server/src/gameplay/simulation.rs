use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{game_state::LobbyGameState, player_handling::TankTransform, tank_types::TankType},
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
    tank_config: TankConfigSystemParam,
    mut transforms: Query<(&mut TankTransform, &TankType), With<DummyClientMarker>>,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby.get_lobby(lobby_entity).expect("Failed to get lobby");
    let tank_config = tank_config.tank_configs();

    for (_, entity, _) in lobby.players.iter() {
        if let Ok((mut transform, tank_type)) = transforms.get_mut(*entity) {
            let tank_config = tank_config
                .tanks
                .get(tank_type)
                .expect("Failed to get tank config");

            transform.position.x += tank_config.move_speed;
        }
    }
}
