use bevy::prelude::*;
use shared::MySharedPlugin;
use test_render_map::MyTestRenderMapPlugin;

pub mod test_render_map;

pub struct MySpectatorClientPlugin;

impl Plugin for MySpectatorClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, MySharedPlugin, MyTestRenderMapPlugin));
    }
}
