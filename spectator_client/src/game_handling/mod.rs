use bevy::prelude::*;
use entity_mapping::MyEntityMapping;

use crate::{game_state::MyGameState, networking::MyNetworkStream};

pub mod despawn_delayed;
pub mod entity_mapping;
pub mod game_starts;
pub mod player_handling;
pub mod projectile_handling;
pub mod smooth_transform_handling;

pub struct MyGameHandlingPlugin;

impl Plugin for MyGameHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyEntityMapping>()
            .register_type::<DelayedDespawn>()
            .add_observer(add_observers)
            .add_systems(
                Update,
                smooth_transform_handling::interpolate_transforms
                    .run_if(in_state(MyGameState::GameStarted)),
            );
    }
}

fn add_observers(trigger: Trigger<OnAdd, MyNetworkStream>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(game_starts::game_starts)
        .observe(player_handling::update_player_target_transform_on_game_state_update)
        .observe(projectile_handling::handle_projectile_on_game_state_update)
        .observe(despawn_delayed::despawn_delayed_entites);
}

/// Marks an entity to be despawned after at the next game update tick received.
#[derive(Debug, Default, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct DelayedDespawn(pub u64);
