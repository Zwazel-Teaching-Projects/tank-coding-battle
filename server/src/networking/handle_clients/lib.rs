use std::net::{SocketAddr, TcpStream};

use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct AwaitingFirstContact(pub Timer);

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MyLocalClient {
    pub network_client: Entity,
}

#[derive(Debug, Component)]
pub struct MyNetworkClient {
    pub name: Option<String>,
    pub address: SocketAddr,
    pub stream: TcpStream,
    pub my_local_client: Option<Entity>,
}

impl MyNetworkClient {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            name: None,
            address,
            stream,
            my_local_client: None,
        }
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnectedTrigger(pub Entity);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnectedTrigger(pub Entity);

#[derive(Event, Deref, DerefMut)]
pub struct ClientJoinedLobbyTrigger(pub Entity);
