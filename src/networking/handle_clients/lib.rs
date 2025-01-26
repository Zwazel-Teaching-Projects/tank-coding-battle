use std::net::SocketAddr;

use bevy::prelude::*;

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnected(pub SocketAddr);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnected(pub SocketAddr);
