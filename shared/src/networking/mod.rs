use bevy::prelude::*;
use client_handling::MySharedClientHandlingPlugin;
use messages::MySharedNetworkMessagesPlugin;

pub mod client_handling;
pub mod messages;

pub struct MySharedNetworkingPlugin;

impl Plugin for MySharedNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MySharedClientHandlingPlugin, MySharedNetworkMessagesPlugin));
    }
}
