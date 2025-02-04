use bevy::prelude::*;

#[derive(Debug, Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Room {
    pub id: u32,
    pub clients: Vec<Entity>,
}