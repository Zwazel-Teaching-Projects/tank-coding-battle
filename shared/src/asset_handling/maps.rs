use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
    mapped::AssetFileStem,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::main_state::MyMainState;

pub struct MyMapPlugin;

impl Plugin for MyMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TeamConfig>()
            .register_type::<MapConfig>()
            .configure_loading_state(
                LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<AllMapsAsset>(),
            )
            .add_plugins(RonAssetPlugin::<MapConfig>::new(&["map.ron"]));
    }
}

#[derive(Debug, Default, Clone, AssetCollection, Resource)]
pub struct AllMapsAsset {
    #[asset(path = "maps", collection(mapped, typed))]
    pub maps: HashMap<AssetFileStem, Handle<MapConfig>>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize, PartialEq)]
pub struct MapConfig {
    pub teams: Vec<TeamConfig>,
}

#[derive(Debug, Clone, Reflect, Default, Deserialize, PartialEq)]
pub struct TeamConfig {
    pub name: String,
    pub color: Color,
    pub max_players: usize,
}

#[derive(SystemParam)]
pub struct MapConfigSystemParam<'w> {
    maps_asset: Res<'w, AllMapsAsset>,
    map_configs: Res<'w, Assets<MapConfig>>,
}

impl<'w> MapConfigSystemParam<'w> {
    pub fn get_map_config(&self, map_name: &str) -> Option<&MapConfig> {
        let map_name = if map_name.ends_with(".map") {
            map_name.to_string()
        } else {
            format!("{}.map", map_name)
        };

        self.maps_asset
            .maps
            .iter()
            .find(|(stem, _)| stem.as_ref() == map_name)
            .and_then(|(_, handle)| self.map_configs.get(handle))
    }

    pub fn list_map_names(&self) -> Vec<String> {
        self.maps_asset
            .maps
            .iter()
            .map(|(stem, _)| stem.as_ref().to_string())
            .collect()
    }
}
