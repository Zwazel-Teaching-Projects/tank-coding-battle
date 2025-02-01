use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::main_state::MyMainState;

pub struct MyConfigPlugin;

impl Plugin for MyConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ServerConfig>::new(&["ron"]))
            .register_type::<MyConfigAsset>()
            .register_type::<ServerConfig>()
            .configure_loading_state(
                LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<MyConfigAsset>(),
            );
    }
}

#[derive(Debug, Default, Reflect, Resource, Clone, AssetCollection)]
#[reflect(Resource)]
pub struct MyConfigAsset {
    #[asset(path = "config/server_config.ron")]
    pub server: Handle<ServerConfig>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub tick_rate: f32,
}
