use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use map_visualization::MyMapVisualizationPlugin;
use networking::MyNetworkingPlugin;
use shared::MySharedPlugin;

pub mod map_visualization;
pub mod networking;

pub struct MySpectatorClientPlugin;

impl Plugin for MySpectatorClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            MySharedPlugin,
            MyMapVisualizationPlugin,
            MyNetworkingPlugin,
        ));

        #[cfg(debug_assertions)]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
}
