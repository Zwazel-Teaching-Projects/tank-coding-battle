use bevy::prelude::*;

use crate::networking::MyNetworkStream;

pub mod game_starts;

pub struct MyGameHandlingPlugin;

impl Plugin for MyGameHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_observers);
    }
}

fn add_observers(trigger: Trigger<OnAdd, MyNetworkStream>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(game_starts::game_starts);
}
