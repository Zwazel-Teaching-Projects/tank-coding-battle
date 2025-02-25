use bevy::prelude::*;

pub mod components;
pub mod structs;
pub mod triggers;

pub struct MyCollisionHandlingPlugin;

impl Plugin for MyCollisionHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::Collider>()
            .register_type::<components::CollisionLayer>()
            .register_type::<components::WantedTransform>()
            .register_type::<structs::Obb3d>()
            .add_observer(components::insert_transform_for_wanted_transform);
    }
}
