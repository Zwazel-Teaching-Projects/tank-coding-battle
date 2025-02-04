use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use gameplay::MyGameplayPlugin;
use networking::MyNetworkingPlugin;
use shared::MySharedPlugin;

pub mod gameplay;
pub mod networking;

pub struct MyServerPlugin;

impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            MySharedPlugin,
            MyGameplayPlugin,
            MyNetworkingPlugin,
        ));
    }
}
