use bevy::prelude::*;
use dummy_handling::DummyClientMarker;
use shared::networking::lobby_management::MyLobby;

use crate::networking::handle_clients::lib::MyNetworkClient;

pub mod dummy_handling;
pub mod handle_projectiles;
pub mod handle_shooting;
pub mod insert_turret;
pub mod movement_handling;
pub mod update_client_states;

pub struct HandlePlayersPlugin;

impl Plugin for HandlePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DummyClientMarker>()
            .add_plugins((movement_handling::MyMovementHandlingPlugin,))
            .add_observer(add_observers_to_client)
            .add_observer(add_observers_to_lobby)
            .add_observer(insert_turret::insert_turret)
            .add_observer(dummy_handling::add_observers_to_dummies)
            .add_observer(dummy_handling::add_dummy_simulation_observers_to_lobby)
            .add_observer(handle_shooting::set_timer_for_shooting);
    }
}

fn add_observers_to_client(trigger: Trigger<OnAdd, MyNetworkClient>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(update_client_states::update_client_states)
        .observe(handle_shooting::handle_tank_shooting_command);
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(handle_shooting::tick_shoot_cooldowns)
        .observe(handle_projectiles::move_projectiles)
        .observe(handle_projectiles::handle_despawn_timer);
}
