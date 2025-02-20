use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A command to move a tank in a certain direction
/// You can move the tank in the given direction by the given distance
/// The distance is not allowed to be higher than the tank's maximum speed, but it can be lower
/// The speed/distance is the distance traveled in one tick
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MoveTankCommand {
    pub distance: f32,
}
