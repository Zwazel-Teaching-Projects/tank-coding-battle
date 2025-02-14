use bevy::prelude::*;
use create_map::{add_observers_to_client, listen_for_map_changes, MapMeshMarker};
use shared::{
    main_state::MyMainState, networking::messages::message_data::game_starts::GameStarts,
};
use visualize_markers::{draw_markers, MyMarkerGizmos};

pub mod create_map;
pub mod visualize_markers;

pub struct MyMapVisualizationPlugin;

impl Plugin for MyMapVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapMeshMarker>()
            .init_gizmo_group::<MyMarkerGizmos>()
            .add_systems(
                Update,
                ((
                    (listen_for_map_changes,).run_if(any_with_component::<MapMeshMarker>),
                    (draw_markers,),
                ).run_if(resource_exists::<GameStarts>),)
                    .run_if(in_state(MyMainState::Ready)),
            )
            .add_observer(add_observers_to_client);
    }
}
