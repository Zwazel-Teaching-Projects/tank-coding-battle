use std::net::{SocketAddr, TcpStream};

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct MyNetworkClient {
    pub name: Option<String>,
    pub address: SocketAddr,
    pub stream: TcpStream,
    pub room_id: Option<u32>,
    pub my_local_client: Option<Entity>,
}

impl MyNetworkClient {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            name: None,
            address,
            stream,
            room_id: None,
            my_local_client: None,
        }
    }
}