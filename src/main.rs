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
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        MyConfigPlugin,
        MyGameplayPlugin,
        MyNetworkingPlugin,
    ))
    .init_state::<MyMainState>();

    #[cfg(debug_assertions)]
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<MyMainState>,
    );

    app.run();
}
