use bevy::prelude::*;
use shared::{asset_handling::config::ServerConfigSystemParam, networking::lobby_management::{remove_player_from_lobby, AwaitingFirstContact}};

use crate::networking::{
    handle_clients::lib::{ClientConnectedTrigger, MyNetworkClient},
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
                        AwaitingFirstContact::new(config.timeout_first_contact),
                    ))
                    .observe(remove_player_from_lobby)
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
