use bevy::prelude::*;
use gameplay::MyGameplayPlugin;
use networking::MyNetworkingPlugin;
use shared::MySharedPlugin;

#[cfg(feature = "debug")]
pub mod debug;
pub mod gameplay;
pub mod networking;

pub struct MyServerPlugin;

impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(feature = "debug")]
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Server".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            #[cfg(feature = "release")]
            DefaultPlugins
                .set(bevy::app::ScheduleRunnerPlugin::run_loop(
                    std::time::Duration::from_secs_f64(1.0 / 60.0),
                ))
                .set(bevy::log::LogPlugin {
                    custom_layer: shared::release_logging::custom_log_layer,
                    ..default()
                }),
            MySharedPlugin,
            MyGameplayPlugin,
            MyNetworkingPlugin,
            #[cfg(feature = "debug")]
            debug::MyServerDebugPlugin,
        ));
    }
}
