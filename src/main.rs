#![cfg_attr(
    all(not(debug_assertions), feature = "spectator_client"),
    windows_subsystem = "windows"
)]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // if feature "server" is enabled
    #[cfg(feature = "server")]
    app.add_plugins((server::MyServerPlugin,));

    // if feature "client" is enabled
    #[cfg(feature = "spectator_client")]
    app.add_plugins((spectator_client::MySpectatorClientPlugin,));

    app.run();
}
