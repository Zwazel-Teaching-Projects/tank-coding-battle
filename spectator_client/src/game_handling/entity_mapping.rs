use bevy::{ecs::entity::EntityHashMap, prelude::*};

#[derive(Debug, Resource, Reflect, Deref, DerefMut, Default)]
#[reflect(Resource)]
pub struct MyEntityMapping {
    pub mapping: EntityHashMap<Entity>,
}

impl EntityMapper for MyEntityMapping {
    /// Example implementation of EntityMapper where we map an entity to another entity if it exists
    /// in the underlying `EntityHashMap`, otherwise we just return the original entity.
    fn map_entity(&mut self, entity: Entity) -> Entity {
        self.mapping.get(&entity).copied().unwrap_or(entity)
    }
}
