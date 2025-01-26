use bevy::prelude::*;
use handle_clients::HandleClientsPlugin;
use handle_messages::HandleMessagesPlugin;
use lib::{MyConnections, MyTcpListener};
use std::net::TcpListener;

pub mod handle_clients;
pub mod handle_messages;
pub mod lib;
pub mod run_conditions;
pub mod system_sets;

use crate::config::{ConfigLoadState, MyConfig};

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyConnections::default())
            .add_plugins((HandleClientsPlugin, HandleMessagesPlugin))
            .add_systems(OnEnter(ConfigLoadState::Loaded), setup_listener);
    }
}

fn setup_listener(mut commands: Commands, config: Res<MyConfig>) {
    let listener = TcpListener::bind(format!("{:}:{:}", config.server_ip, config.server_port))
        .expect(
            format!(
                "Failed to bind to port {} on {}",
                config.server_port, config.server_ip
            )
            .as_str(),
        );
    info!("TCP server listening on {}", listener.local_addr().unwrap());

    // Set to non-blocking so `accept()` won't block the main thread
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking mode");

    commands.insert_resource(MyTcpListener { listener });
}
