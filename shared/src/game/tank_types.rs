use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Reflect, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Component,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[reflect(Component)]
pub enum TankType {
    #[default]
    LightTank,
    HeavyTank,
    SelfPropelledArtillery,
}
