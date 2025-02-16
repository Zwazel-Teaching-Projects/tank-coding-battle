use bevy::prelude::*;
use dummy_handling::DummyClientMarker;

pub mod dummy_handling;

pub struct HandlePlayersPlugin;

impl Plugin for HandlePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DummyClientMarker>();
    }
}
