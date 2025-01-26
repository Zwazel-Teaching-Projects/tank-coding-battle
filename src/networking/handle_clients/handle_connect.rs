use bevy::prelude::*;

use crate::networking::{
    handle_clients::lib::ClientConnected,
    lib::{MyClient, MyConnectedClients, MyTcpListener},
};

/// System that checks the channel for newly accepted connections,
pub fn accept_connections_system(
    mut commands: Commands,
    my_listener: Res<MyTcpListener>,
    mut connections: ResMut<MyConnectedClients>,
) {
    // Accept in a loop until we get a WouldBlock error
    loop {
        match my_listener.listener.accept() {
            Ok((stream, addr)) => {
                println!("New client from: {}", addr);
                // If you want, set the stream to non-blocking as well:
                // stream.set_nonblocking(true).unwrap();
                connections.streams.insert(addr, MyClient::new(stream));
                commands.trigger(ClientConnected(addr));
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
