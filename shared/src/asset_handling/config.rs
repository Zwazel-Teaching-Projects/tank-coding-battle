use bevy::{ecs::system::SystemParam, prelude::*};
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
        app.add_plugins((
            RonAssetPlugin::<ServerConfig>::new(&["ron"]),
            RonAssetPlugin::<ClientConfig>::new(&["ron"]),
        ))
        .register_type::<MyConfigAsset>()
        .register_type::<ServerConfig>()
        .register_type::<ClientConfig>()
        .configure_loading_state(
            LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<MyConfigAsset>(),
        );
    }
}

#[derive(Debug, Default, Reflect, Resource, Clone, AssetCollection)]
#[reflect(Resource)]
struct MyConfigAsset {
    #[asset(path = "config/server_config.ron")]
    server: Handle<ServerConfig>,
    #[asset(path = "config/spectator_client_config.ron")]
    client: Handle<ClientConfig>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub tick_rate: u32,
    pub timeout_first_contact: u64, // in milliseconds
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize)]
pub struct ClientConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(SystemParam)]
pub struct ServerConfigSystemParam<'w> {
    config_asset: Res<'w, MyConfigAsset>,
    server_configs: Res<'w, Assets<ServerConfig>>,
}

impl<'w> ServerConfigSystemParam<'w> {
    pub fn get_server_config(&self) -> Option<&ServerConfig> {
        self.server_configs.get(self.config_asset.server.id())
    }

    pub fn server_config(&self) -> &ServerConfig {
        self.server_configs
            .get(self.config_asset.server.id())
            .unwrap()
    }
}

#[derive(SystemParam)]
pub struct ClientConfigSystemParam<'w> {
    config_asset: Res<'w, MyConfigAsset>,
    client_configs: Res<'w, Assets<ClientConfig>>,
}

impl<'w> ClientConfigSystemParam<'w> {
    pub fn get_client_config(&self) -> Option<&ClientConfig> {
        self.client_configs.get(self.config_asset.client.id())
    }

    pub fn client_config(&self) -> &ClientConfig {
        self.client_configs
            .get(self.config_asset.client.id())
            .unwrap()
    }
}
