use bevy::prelude::*;
use game::MySharedGamePlugin;
use networking::MySharedNetworkingPlugin;

pub mod game;
pub mod networking;

pub struct MySharedPlugin;

impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MySharedGamePlugin, MySharedNetworkingPlugin));
    }
}
