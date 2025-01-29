use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

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
        DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
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
