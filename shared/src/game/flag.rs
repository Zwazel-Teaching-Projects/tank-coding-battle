use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
#[require(FlagState)]
pub struct FlagMarker(pub usize);

#[derive(Debug, Clone, Default, Reflect, Component, Serialize, Deserialize, PartialEq)]
#[reflect(Component)]
#[serde(rename_all = "PascalCase", tag = "state", content = "entityId")]
pub enum FlagState {
    #[default]
    InBase,
    Carried(Entity),
    Dropped,
}
