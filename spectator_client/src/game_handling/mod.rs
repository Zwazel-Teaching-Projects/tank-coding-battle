use bevy::prelude::*;
use entity_mapping::MyEntityMapping;

use crate::networking::MyNetworkStream;

pub mod entity_mapping;
pub mod game_starts;
pub mod game_state_update;

pub struct MyGameHandlingPlugin;

impl Plugin for MyGameHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyEntityMapping>()
            .add_observer(add_observers);
    }
}

fn add_observers(trigger: Trigger<OnAdd, MyNetworkStream>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(game_starts::game_starts)
        .observe(game_state_update::game_state_updated);
}
