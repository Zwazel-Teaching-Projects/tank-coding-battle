use bevy::prelude::*;
use shared::{
    game::flag::{FlagBaseMarker, FlagMarker},
    networking::lobby_management::MyLobby,
};

pub mod collision_handler;
pub mod follow_carrier;
pub mod handle_flag_dropped;
pub mod handle_flag_picked_up;
pub mod reset_flags;
pub mod triggers;

pub struct MyCaptureTheFlagPlugin;

impl Plugin for MyCaptureTheFlagPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_lobby)
            .add_observer(add_observers_to_flag)
            .add_observer(add_observers_to_flag_base);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(follow_carrier::follow_carrier)
        .observe(handle_flag_dropped::flag_dropped)
        .observe(handle_flag_picked_up::flag_picked_up);
}

fn add_observers_to_flag(trigger: Trigger<OnAdd, FlagMarker>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(collision_handler::handle_collision_with_flag)
        .observe(reset_flags::reset_flag);
}

fn add_observers_to_flag_base(trigger: Trigger<OnAdd, FlagBaseMarker>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(collision_handler::handle_collision_with_flag_base);
}
