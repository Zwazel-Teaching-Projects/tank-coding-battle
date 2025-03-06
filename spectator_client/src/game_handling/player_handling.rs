use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::components::WantedTransform,
        player_handling::{Health, PlayerState, ShootCooldown, TankBodyMarker, TankTurretMarker},
    },
    networking::messages::message_container::GameStateTrigger,
};

use crate::VisualOffset;

use super::entity_mapping::MyEntityMapping;

pub fn update_player_state_on_game_state_update(
    game_state: Trigger<GameStateTrigger>,
    mut entity_mapping: ResMut<MyEntityMapping>,
    mut tank_body: Query<
        (
            &mut Transform,
            &mut WantedTransform,
            &mut Health,
            &mut ShootCooldown,
            &mut PlayerState,
            &TankBodyMarker,
            &VisualOffset,
        ),
        Without<TankTurretMarker>,
    >,
    mut tank_turret: Query<(&mut Transform, &mut WantedTransform), With<TankTurretMarker>>,
) {
    let game_state = &(**game_state.event());

    game_state.client_states.iter().for_each(
        |(server_side_client_entity, server_side_client_state)| {
            let client_side_entity = entity_mapping.map_entity(*server_side_client_entity);
            if let Ok((
                mut current_body_transform,
                mut next_target_body_transform,
                mut health,
                mut shoot_cooldown,
                mut player_state,
                tank_body,
                body_visual_offset,
            )) = tank_body.get_mut(client_side_entity)
            {
                health.health = server_side_client_state
                    .as_ref()
                    .expect("Client state is missing")
                    .current_health;
                shoot_cooldown.ticks_left = server_side_client_state
                    .as_ref()
                    .expect("Client state is missing")
                    .shoot_cooldown;
                *player_state = server_side_client_state
                    .as_ref()
                    .expect("Client state is missing")
                    .state
                    .as_ref()
                    .expect("Player state is missing")
                    .clone();

                // TRANSFORM UPDATES
                let mut new_body_transform = server_side_client_state
                    .as_ref()
                    .expect("Client state is missing")
                    .transform_body
                    .clone()
                    .expect("Position is missing");
                new_body_transform.translation -= body_visual_offset.0;

                // Setting the actual current body transform to the next target body transform (the one that was previously set)
                current_body_transform.translation = next_target_body_transform.translation;
                current_body_transform.rotation = next_target_body_transform.rotation;

                // Setting the next target body transform to the new body transform, so we can interpolate to it
                next_target_body_transform.translation = new_body_transform.translation;
                next_target_body_transform.rotation = new_body_transform.rotation;

                let (mut current_turret_transform, mut next_target_turret_transform) = tank_turret
                    .get_mut(tank_body.turret.expect("Failed to get turret entity"))
                    .expect("Failed to get turret");

                let new_turret_transform = server_side_client_state
                    .as_ref()
                    .expect("Client state is missing")
                    .transform_turret
                    .clone()
                    .expect("Position is missing");

                current_turret_transform.translation = next_target_turret_transform.translation;
                current_turret_transform.rotation = next_target_turret_transform.rotation;

                next_target_turret_transform.translation = new_turret_transform.translation;
                next_target_turret_transform.rotation = new_turret_transform.rotation;
            } else {
                warn!(
                    "Failed to get tank body for server side client entity {:?}",
                    server_side_client_entity
                );
                return;
            }
        },
    );
}
