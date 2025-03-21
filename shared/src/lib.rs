use asset_handling::MyAssetHandlingPlugin;
use bevy::prelude::*;
use game::MySharedGamePlugin;
use main_state::MyMainState;
use networking::MySharedNetworkingPlugin;

pub mod asset_handling;
pub mod game;
pub mod main_state;
pub mod networking;

pub struct MySharedPlugin;

impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MySharedGamePlugin,
            MySharedNetworkingPlugin,
            MyAssetHandlingPlugin,
        ))
        .init_state::<MyMainState>();

        #[cfg(feature = "debug")]
        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<MyMainState>,
        );
    }
}

#[cfg(feature = "release")]
pub mod release_logging {
    use bevy::log::BoxedLayer;
    use std::sync::OnceLock;
    use tracing_appender::{non_blocking::WorkerGuard, rolling};

    use bevy::log::tracing_subscriber::Layer;
    use bevy::prelude::*;

    static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

    pub fn custom_log_layer(_app: &mut App) -> Option<BoxedLayer> {
        let log_dir = if cfg!(feature = "server") {
            "server_logs"
        } else if cfg!(feature = "spectator_client") {
            "spectator_client_logs"
        } else {
            "logs"
        };
        let file_appender = rolling::daily(log_dir, "app.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let _ = LOG_GUARD.set(guard);
        Some(
            bevy::log::tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_file(true)
                .with_line_number(true)
                .boxed(),
        )
    }
}
