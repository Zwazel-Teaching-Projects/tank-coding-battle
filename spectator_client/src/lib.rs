use bevy::prelude::*;

pub struct MySpectatorClientPlugin;

impl Plugin for MySpectatorClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins,));
    }
}
