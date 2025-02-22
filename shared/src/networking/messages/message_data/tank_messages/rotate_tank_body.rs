use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A command to rotate the tank's body in a certain direction
/// The tank's body can be rotated in the given direction by the given angle in radians
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RotateTankBodyCommand {
    pub angle: f32,
}
