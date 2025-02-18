use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::tank_types::TankType,
    networking::{
        lobby_management::MyLobby,
        messages::{
            message_container::{MoveTankCommandTrigger, RotateTankBodyCommandTrigger},
            message_data::tank_messages::{
                move_tank::MoveTankCommand, rotate_tank_body::RotateTankBodyCommand, MoveDirection,
                RotationDirection,
            },
        },
    },
};

use crate::gameplay::triggers::CollectAndTriggerMessagesTrigger;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct DummyClientMarker;

pub fn add_observers_to_dummies(
    _trigger: Trigger<OnAdd, DummyClientMarker>,
    mut _commands: Commands,
) {
}

pub fn add_dummy_simulation_observers_to_lobby(
    trigger: Trigger<OnAdd, MyLobby>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).observe(simulate_movement);
}

pub fn simulate_movement(
    trigger: Trigger<CollectAndTriggerMessagesTrigger>,
    lobbies: Query<&MyLobby>,
    mut commands: Commands,
    dummy_clients: Query<&TankType, With<DummyClientMarker>>,
    tank_config: TankConfigSystemParam,
) {
    let lobby = lobbies.get(trigger.entity()).expect("Failed to get lobby");
    // Get all players of type Dummy
    for (_, player, _) in lobby.players.iter() {
        if let Ok(tank_type) = dummy_clients.get(*player) {
            let tank_config = tank_config
                .get_tank_type_config(tank_type)
                .expect("Failed to get tank config");

            /* // Simulate movement (always forward)
            commands.trigger_targets(
                MoveTankCommandTrigger {
                    sender: None,
                    message: MoveTankCommand {
                        direction: MoveDirection::Forward,
                        distance: tank_config.move_speed,
                    },
                },
                *player,
            ); */

            // Simulate movement (randomly)
            if rand::random::<bool>() {
                let direction = if rand::random() {
                    MoveDirection::Forward
                } else {
                    MoveDirection::Backward
                };
                commands.trigger_targets(
                    MoveTankCommandTrigger {
                        sender: None,
                        message: MoveTankCommand {
                            direction,
                            distance: tank_config.move_speed,
                        },
                    },
                    *player,
                );
            }

            // Simulate rotation (randomly)
            if rand::random::<bool>() {
                let direction = if rand::random() {
                    RotationDirection::Clockwise
                } else {
                    RotationDirection::CounterClockwise
                };
                commands.trigger_targets(
                    RotateTankBodyCommandTrigger {
                        sender: None,
                        message: RotateTankBodyCommand {
                            direction,
                            angle: tank_config.body_rotation_speed,
                        },
                    },
                    *player,
                );
            }
        }
    }
}
