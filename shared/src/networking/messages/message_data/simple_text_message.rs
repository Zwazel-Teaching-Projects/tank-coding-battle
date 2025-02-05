use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTextMessage {
    pub message: String,
}
