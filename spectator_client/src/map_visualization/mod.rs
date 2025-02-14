use bevy::prelude::*;
use create_map::{add_observers_to_client, listen_for_map_changes, MapMeshMarker};
use shared::main_state::MyMainState;

pub mod create_map;

pub struct MyMapVisualizationPlugin;

impl Plugin for MyMapVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapMeshMarker>()
            .add_systems(
                Update,
                (listen_for_map_changes,)
                    .run_if(in_state(MyMainState::Ready).and(any_with_component::<MapMeshMarker>)),
            )
            .add_observer(add_observers_to_client);
    }
}
