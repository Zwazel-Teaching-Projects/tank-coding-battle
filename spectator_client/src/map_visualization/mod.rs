use bevy::prelude::*;
use create_map::{listen_for_map_changes, MapMeshMarker};
use shared::{
    main_state::MyMainState, networking::messages::message_data::game_starts::GameStarts,
};
use visualize_markers::{draw_markers, MyMarkerGizmos};
use visualize_players::update_player_positions;

use crate::networking::MyNetworkStream;

pub mod create_map;
pub mod visualize_markers;
pub mod visualize_players;

pub struct MyMapVisualizationPlugin;

impl Plugin for MyMapVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapMeshMarker>()
            .init_gizmo_group::<MyMarkerGizmos>()
            .add_systems(
                Update,
                (
                    (
                        (listen_for_map_changes,).run_if(any_with_component::<MapMeshMarker>),
                        (draw_markers,),
                    )
                        .run_if(resource_exists::<GameStarts>),
                    update_player_positions,
                )
                    .run_if(in_state(MyMainState::Ready)),
            )
            .add_observer(add_observers_to_client);
    }
}

fn add_observers_to_client(trigger: Trigger<OnAdd, MyNetworkStream>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(create_map::create_map)
        .observe(visualize_players::create_player_visualisation);
}
