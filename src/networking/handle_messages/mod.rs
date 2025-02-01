use std::io::{Read, Write};

use bevy::prelude::*;
use handle_sending::sending_messages;
use lib::QueuedMessages;

use crate::networking::handle_clients::lib::ClientDisconnectedTrigger;

use super::{handle_clients::lib::MyNetworkClient, system_sets::MyNetworkingSet};

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
fn handle_client_messages(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut MyNetworkClient)>,
) {
    for (entity, mut network_client) in clients.iter_mut() {
        // Non-blocking read attempt
        let addr = network_client.address;
        let stream = &mut network_client.stream;
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf) {
            Ok(0) => {
                // 0 = client closed connection
                info!("Client disconnected: {:?}", addr);
                commands.trigger(ClientDisconnectedTrigger(entity));
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
                // No more data to read right now
            }
            Err(e) => {
                // Some other read error
                eprintln!("Read error: {}, {:?}.", e, e.kind());
                eprintln!("Disconnecting client: {:?}", addr);
                commands.trigger(ClientDisconnectedTrigger(entity));
            }
        }
    }
}
