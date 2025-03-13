use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::game::collision_handling::components::Collider;

pub mod visualize_colliders;

pub struct MyServerDebugPlugin;

impl Plugin for MyServerDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldInspectorPlugin::new(), PlayerPlugin))
            .init_gizmo_group::<visualize_colliders::MyColliderGizmos>()
            .add_systems(
                Update,
                (
                    visualize_colliders::visualize_colliders.run_if(any_with_component::<Collider>),
                    visualize_colliders::visualize_world,
                ),
            );
    }
}
