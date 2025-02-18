use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Reflect, Clone, PartialEq, Serialize, Deserialize, Default)]
#[reflect(Component)]
#[serde(rename_all = "camelCase")]
pub struct TankTransform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl From<Transform> for TankTransform {
    fn from(transform: Transform) -> Self {
        TankTransform {
            position: transform.translation,
            rotation: transform.rotation,
        }
    }
}

impl From<TankTransform> for Transform {
    fn from(tank_transform: TankTransform) -> Self {
        Transform {
            translation: tank_transform.position,
            rotation: tank_transform.rotation,
            ..Default::default()
        }
    }
}