use bevy::prelude::*;
use messages::MySharedNetworkMessagesPlugin;

pub mod messages;

pub struct MySharedNetworkingPlugin;

impl Plugin for MySharedNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MySharedNetworkMessagesPlugin,));
    }
}
