use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use config::MyConfigPlugin;
use gameplay::MyGameplayPlugin;
use main_state::MyMainState;
use networking::MyNetworkingPlugin;

pub mod config;
pub mod gameplay;
pub mod main_state;
pub mod networking;

fn main() {
    App::new()
        .init_state::<MyMainState>()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            MyConfigPlugin,
            MyGameplayPlugin,
            MyNetworkingPlugin,
        ))
        .run();
}
