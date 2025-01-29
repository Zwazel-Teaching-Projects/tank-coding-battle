use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::gameplay::lib::GameState;

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message: NetworkMessageType,
}

#[derive(Serialize, Deserialize, Reflect, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "message_type")]
pub enum NetworkMessageType {
    GameStateUpdate(GameState),
    BotConfig,
}

impl Default for NetworkMessageType {
    fn default() -> Self {
        NetworkMessageType::GameStateUpdate(GameState::default())
    }
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
