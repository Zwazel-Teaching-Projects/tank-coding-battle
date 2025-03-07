use bevy::prelude::*;
use shared::networking::lobby_management::MyLobby;

pub mod reset_flags;
pub mod triggers;

pub struct MyCaptureTheFlagPlugin;

impl Plugin for MyCaptureTheFlagPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_lobby);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(reset_flags::reset_flags);
}
