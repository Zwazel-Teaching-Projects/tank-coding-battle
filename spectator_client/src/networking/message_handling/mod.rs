use bevy::prelude::*;
use shared::networking::networking_system_sets::MyNetworkingSet;

pub mod reading_messages;
pub mod sending_messages;

pub struct MyMessageHandlingPlugin;

impl Plugin for MyMessageHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sending_messages::sending_messages.in_set(MyNetworkingSet::SendingMessages),
                reading_messages::reading_messages.in_set(MyNetworkingSet::ReadingMessages),
            ),
        );
    }
}
