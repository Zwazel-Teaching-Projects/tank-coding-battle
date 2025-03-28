use bevy::prelude::*;
use shared::networking::lobby_management::MyLobby;

pub mod handle_collisions;

pub struct MyCollisionHandlingPlugin;

impl Plugin for MyCollisionHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_lobby);

        #[cfg(feature = "debug")]
        app.add_plugins(handle_collisions::debug::CollisionDebugPlugin);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(handle_collisions::unified_collision_system);
}
