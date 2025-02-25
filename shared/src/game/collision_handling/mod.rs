use bevy::prelude::*;

pub mod components;
pub mod triggers;
pub mod structs;

pub struct MyCollisionHandlingPlugin;

impl Plugin for MyCollisionHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Collider>()
            .register_type::<components::CollisionLayer>()
            .register_type::<components::WantedTransform>()
            .add_observer(components::insert_transform_for_wanted_transform);
        // TODO: Send triggers. the triggered entity is the one colliding with something, the trigger contains the entity it collided with.
        // If it collides with the world, we need to handle this a bit different.
        // Each entity than observes for these triggers, and handles how it should react to them.
        // This way we can have a single system that handles all collisions, and the entities themselves decide how they should react.
        // Store the wanted position and the current position of the entity, and let the collision system handle the rest, stopping the entity from moving if it collides with something.
    }
}
