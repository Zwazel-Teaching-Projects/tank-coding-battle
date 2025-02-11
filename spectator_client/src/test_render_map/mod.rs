use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::GREEN,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use shared::{
    asset_handling::maps::{MapConfig, MapConfigSystemParam},
    main_state::MyMainState,
};

pub struct MyTestRenderMapPlugin;

impl Plugin for MyTestRenderMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapMeshMarker>()
            .add_systems(OnEnter(MyMainState::Ready), (create_map,));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Component, Default)]
#[reflect(Component)]
struct MapMeshMarker;

fn create_map(map_config: MapConfigSystemParam, mut commands: Commands) {
    let map_config = &map_config
        .get_map_config_from_name("test_map")
        .expect("Map not found")
        .map;

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
