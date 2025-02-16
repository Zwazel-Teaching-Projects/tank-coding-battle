use std::net::TcpStream;

use bevy::prelude::*;
use shared::networking::messages::message_queue::{ImmediateOutMessageQueue, OutMessageQueue};

#[derive(Debug, Component)]
#[require(OutMessageQueue, ImmediateOutMessageQueue)]
pub struct MyNetworkClient {
    pub name: Option<String>,
    pub assigned_spawn_point: Option<usize>,
    pub stream: Option<TcpStream>,
}

impl MyNetworkClient {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            name: None,
            stream: Some(stream),
            assigned_spawn_point: None,
        }
    }

    pub fn new_dummy(name: String) -> Self {
        Self {
            name: Some(name),
            stream: None,
            assigned_spawn_point: None,
        }
    }

    pub fn get_address(&self) -> Option<String> {
        self.stream
            .as_ref()
            .map(|s| s.peer_addr().unwrap().to_string())
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnectedTrigger(pub Entity);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnectedTrigger(pub Entity);
