use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
pub struct TextDataWrapper {
    pub message: String,
}

impl TextDataWrapper {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}