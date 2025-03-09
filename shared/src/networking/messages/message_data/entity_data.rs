use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
pub struct EntityDataWrapper {
    pub entity_id: Entity,
}

impl EntityDataWrapper {
    pub fn new(entity_id: Entity) -> Self {
        Self { entity_id }
    }
}
