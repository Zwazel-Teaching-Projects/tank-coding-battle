use bevy::prelude::*;

use crate::networking::{
    handle_clients::lib::{ClientConnectedEvent, MyNetworkClient},
    lib::MyTcpListener,
};

/// System that checks the channel for newly accepted connections,
pub fn accept_connections_system(
    mut commands: Commands,
    mut event: EventWriter<ClientConnectedEvent>,
    my_listener: Res<MyTcpListener>,
) {
    // Accept in a loop until we get a WouldBlock error
    loop {
        match my_listener.listener.accept() {
            Ok((stream, addr)) => {
                info!("New client from: {}", addr);
                // If you want, set the stream to non-blocking as well:
                stream.set_nonblocking(true).unwrap();

                let networked_client = commands.spawn(MyNetworkClient::new(addr, stream)).id();

                commands.trigger(ClientConnectedEvent(networked_client));
                event.send(ClientConnectedEvent(networked_client));
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
