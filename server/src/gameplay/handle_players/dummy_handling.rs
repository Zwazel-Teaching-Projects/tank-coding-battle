use bevy::prelude::*;
use shared::networking::{
    lobby_management::MyLobby,
    messages::{
        message_container::MoveTankCommandTrigger,
        message_data::{
            first_contact::ClientType,
            tank_messages::{move_tank::MoveTankCommand, MoveDirection},
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
) {
    let lobby = lobbies.get(trigger.entity()).expect("Failed to get lobby");
    // Get all players of type Dummy
    let dummies = lobby
        .players
        .iter()
        .filter(|(_, _, client_type)| client_type == &ClientType::Dummy)
        .map(|(_, entity, _)| *entity)
        .collect::<Vec<_>>();

    // Send a simulated movement command to all dummies
    commands.trigger_targets(
        MoveTankCommandTrigger {
            sender: None,
            message: MoveTankCommand {
                direction: MoveDirection::Forward,
                distance: 1.0,
            },
        },
        dummies,
    );
}
// TODO: Simulate receiving commands from clients, for movement and stuff
