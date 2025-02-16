use bevy::prelude::*;
use shared::{
    game::player_handling::TankTransform, networking::messages::message_container::GameStateTrigger,
};

use super::entity_mapping::MyEntityMapping;

pub fn game_state_updated(
    game_state: Trigger<GameStateTrigger>,
    mut clients: Query<(&mut TankTransform, &MyEntityMapping)>,
) {
    let game_state = &(**game_state.event());

    game_state
        .client_states
        .iter()
        .for_each(|(client_entity, client_state)| {
            clients
                .iter_mut()
                .for_each(|(mut tank_transform, entity_mapping)| {
                    if entity_mapping.server_entity == *client_entity {
                        let transform = client_state.position.clone().expect("Position is missing");
                        tank_transform.position = transform.position;
                        tank_transform.rotation = transform.rotation;

                        return;
                    }
                });
        });
}
