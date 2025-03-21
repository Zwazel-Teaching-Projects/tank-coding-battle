use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use game_handling::{entity_mapping::MyEntityMapping, MyGameHandlingPlugin};
use game_state::MyGameState;
use map_visualization::MyMapVisualizationPlugin;
use networking::MyNetworkingPlugin;
use shared::{networking::messages::message_data::game_starts::GameStarts, MySharedPlugin};
use ui::MyUiPlugin;

pub mod game_handling;
pub mod game_state;
pub mod map_visualization;
pub mod networking;
pub mod ui;

pub struct MySpectatorClientPlugin;

impl Plugin for MySpectatorClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Client".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(bevy::log::LogPlugin {
                    #[cfg(feature = "release")]
                    custom_layer: shared::release_logging::custom_log_layer,
                    ..default()
                }),
            PlayerPlugin,
            BillboardPlugin,
            MySharedPlugin,
            MyMapVisualizationPlugin,
            MyNetworkingPlugin,
            MyUiPlugin,
            MyGameHandlingPlugin,
        ))
        .add_systems(
            Update,
            change_camera_transform.run_if(resource_added::<GameStarts>),
        )
        .add_sub_state::<MyGameState>()
        .enable_state_scoped_entities::<MyGameState>()
        .register_type::<MyEntityMapping>()
        .init_resource::<MyEntityMapping>();

        #[cfg(feature = "debug")]
        {
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }
    }
}

fn change_camera_transform(
    game_config: Res<GameStarts>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut camera_transform = query
        .get_single_mut()
        .expect("There should be a single camera");
    let map_center = game_config.map_definition.get_center_of_map();
    let map_size = (
        game_config.map_definition.width,
        game_config.map_definition.depth,
    );
    camera_transform.translation = Vec3::new(
        map_center.x,
        map_center.y + map_size.1 as f32,
        map_center.z + map_size.0 as f32,
    );

    let center_of_map = game_config.map_definition.get_center_of_map();
    camera_transform.look_at(center_of_map, Vec3::Y);
}
