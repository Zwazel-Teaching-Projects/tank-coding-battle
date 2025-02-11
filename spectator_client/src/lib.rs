use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use shared::MySharedPlugin;
use test_render_map::MyTestRenderMapPlugin;

pub mod test_render_map;

pub struct MySpectatorClientPlugin;

impl Plugin for MySpectatorClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            MySharedPlugin,
            MyTestRenderMapPlugin,
            PlayerPlugin,
        ));

        #[cfg(debug_assertions)]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
}
