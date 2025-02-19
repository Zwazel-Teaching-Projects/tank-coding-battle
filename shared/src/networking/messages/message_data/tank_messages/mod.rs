use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod move_tank;
pub mod rotate_tank_body;
pub mod rotate_tank_turret;

pub struct MyTankMessagesPlugin;

impl Plugin for MyTankMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MoveDirection>()
            .register_type::<RotationDirection>()
            .register_type::<move_tank::MoveTankCommand>()
            .register_type::<rotate_tank_body::RotateTankBodyCommand>()
            .register_type::<rotate_tank_turret::RotateTankTurretCommand>();
    }
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Component, Default)]
#[reflect(Component)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoveDirection {
    #[default]
    Forward,
    Backward,
}

impl MoveDirection {
    pub fn to_movement(&self) -> f32 {
        match self {
            MoveDirection::Forward => 1.0,
            MoveDirection::Backward => -1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Component, Default)]
#[reflect(Component)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RotationDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

impl RotationDirection {
    pub fn to_radians(&self) -> f32 {
        match self {
            RotationDirection::Clockwise => -1.0,
            RotationDirection::CounterClockwise => 1.0,
        }
    }
}
