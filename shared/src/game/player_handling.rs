use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Clone, PartialEq, Default)]
#[reflect(Component)]
pub struct TankBodyMarker {
    pub turret: Option<Entity>,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub struct TankTurretMarker {
    pub body: Entity,
}
