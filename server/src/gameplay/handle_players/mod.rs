use bevy::prelude::*;
use dummy_handling::DummyClientMarker;

use crate::networking::handle_clients::lib::MyNetworkClient;

pub mod dummy_handling;
pub mod movement_handling;
pub mod update_client_states;

pub struct HandlePlayersPlugin;

impl Plugin for HandlePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DummyClientMarker>()
            .add_plugins((movement_handling::MyMovementHandlingPlugin,))
            .add_observer(add_observers_to_client)
            .add_observer(dummy_handling::add_observers_to_dummies)
            .add_observer(dummy_handling::add_dummy_simulation_observers_to_lobby);
    }
}

fn add_observers_to_client(trigger: Trigger<OnAdd, MyNetworkClient>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(update_client_states::update_client_states);
}
