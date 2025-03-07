use bevy::prelude::*;
use shared::networking::lobby_management::MyLobby;

pub mod setup_flags;

pub struct MySetupMapPlugin;

impl Plugin for MySetupMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_lobby);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(setup_flags::setup_flags);
}
