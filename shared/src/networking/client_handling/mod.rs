use bevy::prelude::*;

pub mod networked_client;

pub struct MySharedClientHandlingPlugin;

impl Plugin for MySharedClientHandlingPlugin {
    fn build(&self, app: &mut App) {}
}
