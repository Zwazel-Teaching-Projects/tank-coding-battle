use bevy::prelude::*;
use gameplay::MyGameplayPlugin;
use networking::MyNetworkingPlugin;
use shared::MySharedPlugin;

pub mod gameplay;
pub mod networking;

pub struct MyServerPlugin;

impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(feature = "debug")]
            DefaultPlugins,
            #[cfg(not(feature = "debug"))]
            DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            )),
            MySharedPlugin,
            MyGameplayPlugin,
            MyNetworkingPlugin,
            #[cfg(feature = "debug")]
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ));
    }
}
