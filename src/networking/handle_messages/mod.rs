use std::io::{Read, Write};

use bevy::prelude::*;
use handle_sending::sending_messages;
use lib::QueuedMessages;

use crate::networking::handle_clients::lib::ClientDisconnectedTrigger;

use super::{lib::MyConnectedClients, system_sets::MyNetworkingSet};

mod handle_sending;
pub mod lib;

pub struct HandleMessagesPlugin;

impl Plugin for HandleMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<QueuedMessages>()
            .init_resource::<QueuedMessages>()
            .add_systems(
                Update,
                (handle_client_messages,).in_set(MyNetworkingSet::ReadingMessages),
            )
            .add_systems(
                Update,
                sending_messages.in_set(MyNetworkingSet::SendingMessages),
            );
    }
}

/// Example system that reads data from connected clients.
/// In a real project, you’d parse structured messages, handle disconnections, etc.
fn handle_client_messages(mut commands: Commands, mut connections: ResMut<MyConnectedClients>) {
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
        commands.trigger(ClientDisconnectedTrigger(**addr));
    }
}
