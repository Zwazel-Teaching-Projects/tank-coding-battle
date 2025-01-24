use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;

use bevy_renet::netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::RenetServerPlugin;

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
            .add_systems(
                Update,
                (send_messages, receive_message_system, handle_events)
                    .run_if(resource_exists::<RenetServer>),
            );
    }
}

fn init_renet_server(mut commands: Commands, config: Res<MyConfig>) {
    let server = RenetServer::new(ConnectionConfig::default());
    commands.insert_resource(server);

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

fn send_messages(shared: Res<SharedGameState>, mut server: ResMut<RenetServer>) {
    let _channel_id = 0;

    // Send a message to all clients
    //server.broadcast_message(DefaultChannel::ReliableOrdered, "Hello, clients!");
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            // Handle received message
            println!("Received message from client {}: {:?}", client_id, message);
        }
    }
}

/// Process a single client connection.
fn handle_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
