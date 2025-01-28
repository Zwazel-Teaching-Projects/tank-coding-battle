use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Default, Reflect)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message_type: NetworkMessageTypes,
    #[reflect(ignore)]
    pub message_data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Reflect, Default)]
pub enum NetworkMessageTypes {
    #[default]
    CommandsRequest,
    Message,
}

#[derive(Serialize, Deserialize, Default, Reflect)]
pub enum MessageTarget {
    #[default]
    Team,
    All,
    Client(#[reflect(ignore, default = "default_client")] SocketAddr),
}

fn default_client() -> SocketAddr {
    SocketAddr::new("127.0.0.1".parse().unwrap(), 9999)
}
