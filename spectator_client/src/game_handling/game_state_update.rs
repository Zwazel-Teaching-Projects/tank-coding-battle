use bevy::prelude::*;
use shared::{
    game::player_handling::{TankBodyMarker, TankTransform, TankTurretMarker},
    networking::messages::message_container::GameStateTrigger,
};

use super::entity_mapping::MyEntityMapping;

pub fn game_state_updated(
    game_state: Trigger<GameStateTrigger>,
    mut tank_body: Query<
        (&mut TankTransform, &MyEntityMapping, &TankBodyMarker),
        Without<TankTurretMarker>,
    >,
    mut tank_turret: Query<&mut TankTransform, With<TankTurretMarker>>,
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
                        current_body_transform.position = new_body_transform.position;
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

                        current_turret_transform.position = new_turret_transform.position;
                        current_turret_transform.rotation = new_turret_transform.rotation;

                        return;
                    }
                },
            );
        });
}
