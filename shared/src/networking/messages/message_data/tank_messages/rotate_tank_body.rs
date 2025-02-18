use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::RotationDirection;

/// A command to rotate the tank's body in a certain direction
/// The tank's body can be rotated in the given direction by the given angle
/// The angle is not allowed to be higher than the tank's maximum rotation speed, but it can be lower
/// The speed/angle is the angle rotated in one tick
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RotateTankBodyCommand {
    pub direction: RotationDirection,
    pub angle: f32,
}
