use bevy::prelude::*;
use shared::networking::lobby_management::MyLobby;

pub mod handle_collisions;

pub struct MyCollisionHandlingPlugin;

impl Plugin for MyCollisionHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers_to_lobby);

        
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(handle_collisions::check_world_collision_and_apply_movement)
        .observe(handle_collisions::detect_pairwise_collisions);
}
