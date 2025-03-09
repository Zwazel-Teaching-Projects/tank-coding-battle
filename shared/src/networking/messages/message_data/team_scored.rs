use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamScoredData {
    pub scorer: Entity,
    pub team: String,
    pub score: u32,
}
