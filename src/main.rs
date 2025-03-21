#![cfg_attr(
    all(feature = "spectator_client", not(feature = "debug")),
    windows_subsystem = "windows"
)]

use bevy::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    // if feature "server" is enabled
    #[cfg(feature = "server")]
    app.add_plugins((server::MyServerPlugin,));

    // if feature "client" is enabled
    #[cfg(feature = "spectator_client")]
    app.add_plugins((spectator_client::MySpectatorClientPlugin,));

    // If server or client is enabled but not debug, add "fern" logger
    #[cfg(any(feature = "server", feature = "spectator_client"))]
    #[cfg(not(feature = "debug"))]
    fern::Dispatch::new()
        .level(log::LevelFilter::Error)
        .chain(fern::log_file("error.log")?)
        .apply()?;

    app.run();

    Ok(())
}
