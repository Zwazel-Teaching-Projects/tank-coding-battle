use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use bevy::prelude::*;

use bevy_renet::netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerConfig};
use bevy_renet::RenetServerPlugin;
use lib::MyConnectedClients;

use crate::config::{ConfigLoadState, MyConfig};
use crate::SharedGameState;

mod lib;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(
                OnEnter(ConfigLoadState::Loaded),
                (init_renet_server,).chain(),
            )
            .init_resource::<MyConnectedClients>();
    }
}

fn init_renet_server(mut commands: Commands, config: Res<MyConfig>) {
    let server_addr = format!("{}:{}", config.server_ip, config.server_port)
        .parse()
        .unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: config.teams.iter().map(|team| team.max_players).sum(),
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: bevy_renet::netcode::ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    commands.insert_resource(transport);

    info!("Server started on {}", server_addr);
}

fn send_messages(shared: Res<SharedGameState>, connected_clients: Res<MyConnectedClients>) {}

/// Process a single client connection.
fn handle_connection(shared: &SharedGameState) {}
