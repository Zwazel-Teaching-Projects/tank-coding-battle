use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Resource, Reflect, Serialize, Deserialize, Clone, PartialEq)]
#[reflect(Resource)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub tick: u64,
}
