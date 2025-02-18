use bevy::prelude::*;

use crate::networking::handle_clients::lib::MyNetworkClient;

pub mod handle_tank_movement;

pub struct MyMovementHandlingPlugin;

impl Plugin for MyMovementHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_client);
    }
}

fn add_observers_to_client(trigger: Trigger<OnAdd, MyNetworkClient>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(handle_tank_movement::handle_tank_movement);
}
