use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Reflect, Component, Clone, PartialEq, Default, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq, Default)]
#[reflect(Component)]
#[require(ShootCooldown, Transform, PlayerState)]
pub struct TankBodyMarker {
    pub turret: Option<Entity>,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub struct TankTurretMarker {
    pub body: Entity,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub struct ShootCooldown {
    pub ticks_left: u32,
    pub ticks_cooldown: u32,
}

impl Default for ShootCooldown {
    fn default() -> Self {
        Self {
            ticks_left: 0,
            ticks_cooldown: 0,
        }
    }
}
