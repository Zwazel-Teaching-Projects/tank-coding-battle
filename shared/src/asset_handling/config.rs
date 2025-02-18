use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{game::tank_types::TankType, main_state::MyMainState};

pub struct MyConfigPlugin;

impl Plugin for MyConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<ServerConfig>::new(&["server.ron"]),
            RonAssetPlugin::<ClientConfig>::new(&["client.ron"]),
            RonAssetPlugin::<TankConfigs>::new(&["tanks.ron"]),
        ))
        .register_type::<MyConfigAsset>()
        .register_type::<ServerConfig>()
        .register_type::<ClientConfig>()
        .register_type::<TankConfigs>()
        .register_type::<TankConfig>()
        .configure_loading_state(
            LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<MyConfigAsset>(),
        );
    }
}

#[derive(Debug, Default, Reflect, Resource, Clone, AssetCollection)]
#[reflect(Resource)]
struct MyConfigAsset {
    #[asset(path = "config/config.server.ron")]
    server: Handle<ServerConfig>,
    #[asset(path = "config/config.client.ron")]
    client: Handle<ClientConfig>,
    #[asset(path = "config/config.tanks.ron")]
    tank: Handle<TankConfigs>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub tick_rate: u64,
    pub timeout_first_contact: u64, // in milliseconds
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize)]
pub struct ClientConfig {
    pub ip: String,
    pub port: u16,
    pub map: String,
    pub name: String,
    pub lobby_name: String,
    pub fill_empty_slots_with_dummies: bool,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize, PartialEq)]
pub struct TankConfigs {
    pub tanks: HashMap<TankType, TankConfig>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TankConfig {
    pub speed: f32,
}

#[derive(SystemParam)]
pub struct ServerConfigSystemParam<'w> {
    config_asset: Res<'w, MyConfigAsset>,
    server_configs: Res<'w, Assets<ServerConfig>>,
}

impl<'w> ServerConfigSystemParam<'w> {
    pub fn server_config(&self) -> &ServerConfig {
        self.server_configs
            .get(self.config_asset.server.id())
            .expect("Server config not loaded")
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
            .expect("Client config not loaded")
    }
}

#[derive(SystemParam)]
pub struct TankConfigSystemParam<'w> {
    config_asset: Res<'w, MyConfigAsset>,
    tank_configs: Res<'w, Assets<TankConfigs>>,
}

impl<'w> TankConfigSystemParam<'w> {
    pub fn get_tank_type_config(&self, tank_type: TankType) -> Option<&TankConfig> {
        self.tank_configs
            .get(self.config_asset.tank.id())
            .and_then(|tank_configs| tank_configs.tanks.get(&tank_type))
    }

    pub fn tank_configs(&self) -> &TankConfigs {
        self.tank_configs
            .get(self.config_asset.tank.id())
            .expect("Tank configs not loaded")
    }
}
