use bevy::prelude::*;

#[derive(Debug, Event, Reflect)]
pub struct CollidedWithTrigger {
    pub entity: Entity,
}
