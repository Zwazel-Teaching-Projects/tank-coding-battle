use std::net::SocketAddr;

use bevy::prelude::*;

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnectedEvent(pub SocketAddr);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnectedTrigger(pub SocketAddr);
