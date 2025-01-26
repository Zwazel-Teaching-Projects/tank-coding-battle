use std::net::{SocketAddr, TcpListener, TcpStream};

use bevy::{prelude::*, utils::HashMap};

// Store active, accepted client connections.
#[derive(Resource, Default)]
pub struct MyConnections {
    pub streams: HashMap<SocketAddr, TcpStream>,
}

// A simple resource holding the listener
#[derive(Resource)]
pub struct MyTcpListener {
    pub listener: TcpListener,
}
