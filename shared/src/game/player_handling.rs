use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Clone, PartialEq, Default)]
#[reflect(Component)]
#[require(ShootCooldown)]
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