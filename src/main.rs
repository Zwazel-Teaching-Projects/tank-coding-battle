use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

fn main() {
    let mut app = App::new();

    // if feature "server" is enabled
    #[cfg(feature = "server")]
    app.add_plugins((
        DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
        server::MyServerPlugin,
    ));

    app.run();
}
