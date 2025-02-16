use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct MyEntityMapping {
    pub server_entity: Entity,
    pub client_entity: Entity,
}
