use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::asset_handling::maps::{MapDefinition, TeamConfig};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Resource)]
#[reflect(Resource)]
#[serde(rename_all = "camelCase")]
pub struct GameStarts {
    pub tick_rate: u64,
    pub client_id: Entity,
    pub connected_clients: Vec<ConnectedClientConfig>,
    pub team_configs: HashMap<String, TeamConfig>,
    pub map_definition: MapDefinition,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedClientConfig {
    pub client_id: Entity,
    pub client_name: String,
    pub client_team: String,
    pub assigned_spawn_point: usize,
}
