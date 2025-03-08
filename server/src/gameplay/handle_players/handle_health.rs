use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        player_handling::{Health, PlayerState, RespawnTimer},
        tank_types::TankType,
    },
    networking::lobby_management::MyLobby,
};

use crate::gameplay::triggers::{CheckHealthTrigger, MoveFlagsSimulationStepTrigger};

use super::handle_death::ClientDiedTrigger;

pub fn check_health_and_die(
    trigger: Trigger<CheckHealthTrigger>,
    lobby: Query<&MyLobby>,
    mut commands: Commands,
    mut players: Query<(&Health, &TankType, &mut PlayerState)>,
    tank_configs: TankConfigSystemParam,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby.get(lobby_entity).expect("Lobby not found");

    for (_, player_entity, _) in lobby.players.iter() {
        let (health, tank_type, mut player_state) =
            players.get_mut(*player_entity).expect("Player not found");
        let tank_config = tank_configs
            .get_tank_type_config(tank_type)
            .expect("Tank config not found");

        if health.health <= 0.0 && *player_state == PlayerState::Alive {
            *player_state = PlayerState::Dead;
            commands
                .entity(*player_entity)
                .insert(RespawnTimer(tank_config.respawn_timer));

            commands.trigger_targets(ClientDiedTrigger, *player_entity);
        }
    }

    commands.trigger_targets(MoveFlagsSimulationStepTrigger, lobby_entity);
}
