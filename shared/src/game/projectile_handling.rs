use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub struct ProjectileMarker {
    pub speed: f32,
    pub damage: f32,
    pub owner: Entity,
}
