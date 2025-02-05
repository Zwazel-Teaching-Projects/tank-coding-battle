use std::time::Duration;

use bevy::prelude::*;
use shared::asset_handling::config::ServerConfigSystemParam;

use crate::networking::{
    handle_clients::lib::{AwaitingFirstContact, ClientConnectedTrigger, MyNetworkClient},
    lib::MyTcpListener,
};

/// System that checks the channel for newly accepted connections,
pub fn accept_connections_system(
    mut commands: Commands,
    my_listener: Res<MyTcpListener>,
    server_config: ServerConfigSystemParam,
) {
    let config = server_config.server_config();
    // Accept in a loop until we get a WouldBlock error
    loop {
        match my_listener.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true).unwrap();

                let networked_client = commands
                    .spawn((
                        MyNetworkClient::new(addr, stream),
                        AwaitingFirstContact(Timer::new(
                            Duration::from_millis(config.timeout_first_contact),
                            TimerMode::Once,
                        )),
                    ))
                    .id();

                commands.trigger(ClientConnectedTrigger(networked_client));
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
