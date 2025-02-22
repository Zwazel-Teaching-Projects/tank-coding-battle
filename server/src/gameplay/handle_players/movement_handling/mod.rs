use bevy::prelude::*;
use shared::networking::lobby_management::MyLobby;

use crate::networking::handle_clients::lib::MyNetworkClient;

pub mod handle_tank_movement;
pub mod handle_tank_rotation;
pub mod handle_turret_rotation;

pub struct MyMovementHandlingPlugin;

impl Plugin for MyMovementHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_client)
            .add_observer(add_observers_to_lobby);
    }
}

fn add_observers_to_client(trigger: Trigger<OnAdd, MyNetworkClient>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(handle_tank_movement::handle_tank_movement)
        .observe(handle_tank_rotation::handle_tank_body_rotation)
        .observe(handle_turret_rotation::handle_tank_turret_rotation);
}

fn add_observers_to_lobby(_trigger: Trigger<OnAdd, MyLobby>, mut _commands: Commands) {}
