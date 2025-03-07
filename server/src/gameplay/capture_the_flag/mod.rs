use bevy::prelude::*;
use shared::{game::flag::FlagMarker, networking::lobby_management::MyLobby};

pub mod collision_handler;
pub mod reset_flags;
pub mod triggers;

pub struct MyCaptureTheFlagPlugin;

impl Plugin for MyCaptureTheFlagPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_lobby)
            .add_observer(add_observers_to_flag);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(reset_flags::reset_flags);
}

fn add_observers_to_flag(trigger: Trigger<OnAdd, FlagMarker>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(collision_handler::handle_collision_with_flag);
}
