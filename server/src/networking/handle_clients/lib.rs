use std::net::TcpStream;

use bevy::prelude::*;
use shared::networking::messages::message_queue::{ImmediateOutMessageQueue, OutMessageQueue};

#[derive(Debug, Component)]
#[require(OutMessageQueue, ImmediateOutMessageQueue)]
pub struct MyNetworkClient {
    pub name: Option<String>,
    pub stream: Option<TcpStream>,
}

impl MyNetworkClient {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            name: None,
            stream: Some(stream),
        }
    }

    pub fn get_address(&self) -> Option<String> {
        self.stream.as_ref().map(|s| s.peer_addr().unwrap().to_string())
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnectedTrigger(pub Entity);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnectedTrigger(pub Entity);
