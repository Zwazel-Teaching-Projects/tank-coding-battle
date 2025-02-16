use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use game_handling::MyGameHandlingPlugin;
use game_state::MyGameState;
use map_visualization::MyMapVisualizationPlugin;
use networking::MyNetworkingPlugin;
use shared::MySharedPlugin;
use ui::MyUiPlugin;

pub mod game_handling;
pub mod game_state;
pub mod map_visualization;
pub mod networking;
pub mod ui;

pub struct MySpectatorClientPlugin;

impl Plugin for MySpectatorClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            BillboardPlugin,
            MySharedPlugin,
            MyMapVisualizationPlugin,
            MyNetworkingPlugin,
            MyUiPlugin,
            MyGameHandlingPlugin,
        ))
        .add_sub_state::<MyGameState>()
        .enable_state_scoped_entities::<MyGameState>();

        #[cfg(debug_assertions)]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
}
