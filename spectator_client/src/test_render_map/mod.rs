use bevy::prelude::*;
use shared::{asset_handling::maps::MapConfigSystemParam, main_state::MyMainState};

pub struct MyTestRenderMapPlugin;

impl Plugin for MyTestRenderMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyMainState::Ready), create_map);
    }
}

fn create_map(map_config: MapConfigSystemParam) {
    let map_config = &map_config
        .get_map_config("test_map")
        .expect("Map not found")
        .map;

    println!("Map: {:?}", map_config);
}
