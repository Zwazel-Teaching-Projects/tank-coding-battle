use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfigMessageData {
    pub tick_rate: u64,
    pub client_id: Entity,
}
