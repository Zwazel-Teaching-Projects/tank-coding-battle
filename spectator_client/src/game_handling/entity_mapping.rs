use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct MyEntityMapping {
    pub server_entity: Entity,
}
