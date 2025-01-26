use std::net::{SocketAddr, TcpListener, TcpStream};

use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Deref, DerefMut)]
pub struct MyClient {
    #[deref]
    pub stream: TcpStream,
    pub name: String,
}

impl MyClient {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            name: "NAME NOT SET".to_string(),
        }
    }
}

// Store active, accepted client connections.
#[derive(Resource, Default)]
pub struct MyConnectedClients {
    pub streams: HashMap<SocketAddr, MyClient>,
}

// A simple resource holding the listener
#[derive(Resource)]
pub struct MyTcpListener {
    pub listener: TcpListener,
}
