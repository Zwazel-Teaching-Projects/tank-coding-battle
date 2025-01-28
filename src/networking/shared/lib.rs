use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message_type: NetworkMessageType,
    #[reflect(ignore)]
    pub message_data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Reflect, Default, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NetworkMessageType {
    #[default]
    GameStateUpdate,
}

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageTarget {
    #[default]
    Team,
    All,
    //Client(#[reflect(ignore, default = "default_client")] SocketAddr), // TODO: How to do in java?
}

/* fn default_client() -> SocketAddr {
    SocketAddr::new("127.0.0.1".parse().unwrap(), 9999)
} */
