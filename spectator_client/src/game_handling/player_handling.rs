use bevy::prelude::*;
use shared::{
    game::player_handling::{TankBodyMarker, TankTurretMarker},
    networking::messages::message_container::GameStateTrigger,
};

use super::entity_mapping::MyEntityMapping;

pub fn move_players_on_game_state_update(
    game_state: Trigger<GameStateTrigger>,
    mut tank_body: Query<
        (&mut Transform, &MyEntityMapping, &TankBodyMarker),
        Without<TankTurretMarker>,
    >,
    mut tank_turret: Query<&mut Transform, With<TankTurretMarker>>,
) {
    let game_state = &(**game_state.event());

    game_state
        .client_states
        .iter()
        .for_each(|(client_entity, client_state)| {
            tank_body.iter_mut().for_each(
                |(mut current_body_transform, entity_mapping, tank_body)| {
                    if entity_mapping.server_entity == *client_entity {
                        let new_body_transform = client_state
                            .as_ref()
                            .expect("Client state is missing")
                            .transform_body
                            .clone()
                            .expect("Position is missing");
                        current_body_transform.translation = new_body_transform.translation;
                        current_body_transform.rotation = new_body_transform.rotation;

                        let new_turret_transform = client_state
                            .as_ref()
                            .expect("Client state is missing")
                            .transform_turret
                            .clone()
                            .expect("Position is missing");
                        let mut current_turret_transform = tank_turret
                            .get_mut(tank_body.turret.expect("Failed to get turret entity"))
                            .expect("Failed to get turret");

                        current_turret_transform.translation = new_turret_transform.translation;
                        current_turret_transform.rotation = new_turret_transform.rotation;

                        return;
                    }
                },
            );
        });
}
