use std::net::{SocketAddr, TcpStream};

use bevy::prelude::*;

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MyLocalClient {
    pub network_client: Entity,
}

#[derive(Debug, Component)]
pub struct MyNetworkClient {
    pub name: String,
    pub address: SocketAddr,
    pub stream: TcpStream,
    pub room_id: Option<u32>,
    pub my_local_client: Option<MyLocalClient>,
}

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnectedEvent(pub SocketAddr);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnectedTrigger(pub SocketAddr);
