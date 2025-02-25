use bevy::prelude::*;

pub mod visualize_colliders;

pub struct MyServerDebugPlugin;

impl Plugin for MyServerDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((bevy_inspector_egui::quick::WorldInspectorPlugin::new(),));
    }
}
