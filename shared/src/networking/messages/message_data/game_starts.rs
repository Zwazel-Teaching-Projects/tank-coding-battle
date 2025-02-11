use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset_handling::maps::MapDefinition;

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameStarts {
    pub tick_rate: u64,
    pub client_id: Entity,
    pub connected_clients: Vec<ConnectedClientConfig>,
    pub map_definition: MapDefinition,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedClientConfig {
    pub client_id: Entity,
    pub client_name: String,
    pub client_team: String,
}
