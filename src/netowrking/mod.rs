use bevy::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::config::{ConfigLoadState, MyConfig};

// Store active, accepted client connections.
#[derive(Resource, Default)]
struct Connections {
    streams: Vec<TcpStream>,
}

// A simple resource holding the listener
#[derive(Resource)]
pub struct MyTcpListener {
    pub listener: TcpListener,
}

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            // We can store a list of active client connections.
            .insert_resource(Connections::default())
            .add_systems(OnEnter(ConfigLoadState::Loaded), setup_listener)
            .add_systems(
                Update,
                (accept_connections_system, handle_client_messages)
                    .run_if(resource_exists::<MyTcpListener>),
            );
    }
}

fn setup_listener(mut commands: Commands, config: Res<MyConfig>) {
    // Bind to local TCP port 9999
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

/// System that checks the channel for newly accepted connections,
fn accept_connections_system(
    my_listener: Res<MyTcpListener>,
    mut connections: ResMut<Connections>,
) {
    // Accept in a loop until we get a WouldBlock error
    loop {
        match my_listener.listener.accept() {
            Ok((stream, addr)) => {
                println!("New client from: {}", addr);
                // If you want, set the stream to non-blocking as well:
                // stream.set_nonblocking(true).unwrap();
                connections.streams.push(stream);
            }
            Err(e) => {
                use std::io::ErrorKind;
                match e.kind() {
                    ErrorKind::WouldBlock => {
                        // No more incoming connections right now
                        break;
                    }
                    _ => {
                        // Some other error, e.g. connection reset, etc.
                        eprintln!("Accept error: {}", e);
                        break;
                    }
                }
            }
        }
    }
}

/// Example system that reads data from connected clients.
/// In a real project, you’d parse structured messages, handle disconnections, etc.
fn handle_client_messages(mut connections: ResMut<Connections>) {
    let mut disconnected = Vec::new();

    for (index, stream) in connections.streams.iter_mut().enumerate() {
        // Non-blocking read attempt
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf) {
            Ok(0) => {
                // 0 = client closed connection
                info!("Client closed connection");
                disconnected.push(index);
            }
            Ok(n) => {
                info!("Read {} bytes", n);
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
                disconnected.push(index);
            }
        }
    }

    // Remove any disconnected streams from the vector (in reverse order).
    for &idx in disconnected.iter().rev() {
        info!("Removing client connection at index {}", idx);
        connections.streams.remove(idx);
    }
}
