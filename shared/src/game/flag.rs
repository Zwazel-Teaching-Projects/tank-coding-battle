use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct FlagBaseMarker {
    pub my_flag: Entity,
    pub flag_in_base: bool,
}

#[derive(Debug, Clone, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
#[require(FlagState)]
pub struct FlagMarker {
    pub base: Entity,
}

#[derive(Debug, Clone, Default, Reflect, Component, Serialize, Deserialize, PartialEq)]
#[reflect(Component)]
#[serde(rename_all = "PascalCase", tag = "state", content = "entityId")]
pub enum FlagState {
    #[default]
    InBase,
    Carried(Entity),
    Dropped,
}
