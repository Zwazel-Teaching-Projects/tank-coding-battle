use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone)]
pub struct FirstContactData {
    pub name: String,
    pub lobby_id: String,
}
