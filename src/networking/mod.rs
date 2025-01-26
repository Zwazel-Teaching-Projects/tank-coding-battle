use bevy::prelude::*;
use handle_clients::{lib::ClientDisconnected, HandleClientsPlugin};
use lib::{MyConnections, MyTcpListener};
use run_conditions::server_running;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub mod handle_clients;
pub mod handle_messages;
pub mod lib;
pub mod run_conditions;

use crate::config::{ConfigLoadState, MyConfig};

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            // We can store a list of active client connections.
            .insert_resource(MyConnections::default())
            .add_plugins(HandleClientsPlugin)
            .add_systems(OnEnter(ConfigLoadState::Loaded), setup_listener)
            .add_systems(Update, (handle_client_messages).run_if(server_running));
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

/// Example system that reads data from connected clients.
/// In a real project, you’d parse structured messages, handle disconnections, etc.
fn handle_client_messages(mut commands: Commands, mut connections: ResMut<MyConnections>) {
    let mut disconnected = Vec::new();

    for (addr, stream) in connections.streams.iter_mut() {
        // Non-blocking read attempt
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf) {
            Ok(0) => {
                // 0 = client closed connection
                info!("Client closed connection");
                disconnected.push(addr);
            }
            Ok(n) => {
                // We got `n` bytes
                if n > 0 {
                    let data = &buf[..n];
                    let received = String::from_utf8_lossy(data);
                    info!("Received from client: {}", received);

                    // Example: echo the message back
                    let _ = stream.write_all(b"Echo: ");
                    let _ = stream.write_all(data);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // WouldBlock is normal if we're using non-blocking
                // (if you set_nonblocking(true)).
                // Ignore for now
            }
            Err(e) => {
                // Some other read error
                eprintln!("Read error: {}", e);
                disconnected.push(addr);
            }
        }
    }

    // Remove any disconnected streams from the vector (in reverse order).
    for addr in disconnected.iter() {
        info!("Removing client connection with addr {}", addr);
        commands.trigger(ClientDisconnected(**addr));
    }
}
