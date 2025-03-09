use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FlagEventDataWrapper {
    pub flag_id: Entity,
    pub carrier_id: Entity,
}

impl FlagEventDataWrapper {
    pub fn new(flag_id: Entity, carrier_id: Entity) -> Self {
        Self {
            flag_id,
            carrier_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
pub struct FlagSimpleEventDataWrapper {
    pub flag_id: Entity,
}
