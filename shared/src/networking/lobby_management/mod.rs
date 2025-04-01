use std::time::Duration;

use bevy::prelude::*;

pub mod lobby_management;

pub struct MyLobbyManagementPlugin;

impl Plugin for MyLobbyManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InTeam>()
            .register_type::<AwaitingFirstContact>();
    }
}

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct AwaitingFirstContact(pub Timer);

impl AwaitingFirstContact {
    pub fn new(time_millis: u64) -> Self {
        Self(Timer::new(
            Duration::from_millis(time_millis),
            TimerMode::Once,
        ))
    }
}

#[derive(Debug, Default, Reflect, Clone, Component, Deref, DerefMut, PartialEq)]
#[reflect(Component)]
pub struct InTeam(pub String);
