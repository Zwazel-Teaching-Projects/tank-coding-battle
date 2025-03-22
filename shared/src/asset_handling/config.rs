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

use crate::{
    game::{collision_handling::structs::Side, tank_types::TankType},
    main_state::MyMainState,
};

pub struct MyConfigPlugin;

impl Plugin for MyConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RonAssetPlugin::<TankConfigs>::new(&["tanks.ron"]),))
            .register_type::<MyConfigAsset>()
            .register_type::<TankConfigs>()
            .register_type::<TankConfig>()
            .configure_loading_state(
                LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<MyConfigAsset>(),
            );

        #[cfg(feature = "server")]
        app.add_plugins(server_config::ServerConfigPlugin);

        #[cfg(feature = "spectator_client")]
        app.add_plugins(spectator_client_config::ClientConfigPlugin);
    }
}

#[derive(Debug, Default, Reflect, Resource, Clone, AssetCollection)]
#[reflect(Resource)]
struct MyConfigAsset {
    #[cfg(feature = "server")]
    #[asset(path = "config/config.server.ron")]
    server: Handle<server_config::ServerConfig>,
    #[cfg(feature = "spectator_client")]
    #[asset(path = "config/config.client.ron")]
    client: Handle<spectator_client_config::ClientConfig>,
    #[asset(path = "config/config.tanks.ron")]
    tank: Handle<TankConfigs>,
}

#[cfg(feature = "server")]
pub mod server_config {
    use bevy::{ecs::system::SystemParam, prelude::*};
    use serde::Deserialize;

    pub struct ServerConfigPlugin;

    impl Plugin for ServerConfigPlugin {
        fn build(&self, app: &mut App) {
            app.register_type::<ServerConfig>()
                .add_plugins((super::RonAssetPlugin::<ServerConfig>::new(&["server.ron"]),));
        }
    }

    #[derive(Debug, Default, Reflect, Clone, Asset, Deserialize)]
    pub struct ServerConfig {
        pub ip: String,
        pub port: u16,
        pub tick_rate: u64,
        pub timeout_first_contact: u64, // in milliseconds
    }

    #[derive(SystemParam)]
    pub struct ServerConfigSystemParam<'w> {
        config_asset: Res<'w, super::MyConfigAsset>,
        server_configs: Res<'w, Assets<ServerConfig>>,
    }

    impl<'w> ServerConfigSystemParam<'w> {
        pub fn server_config(&self) -> &ServerConfig {
            self.server_configs
                .get(self.config_asset.server.id())
                .expect("Server config not loaded")
        }
    }
}

#[cfg(feature = "spectator_client")]
pub mod spectator_client_config {
    use bevy::{ecs::system::SystemParam, prelude::*};
    use serde::Deserialize;

    pub struct ClientConfigPlugin;

    impl Plugin for ClientConfigPlugin {
        fn build(&self, app: &mut App) {
            app.register_type::<ClientConfig>()
                .add_plugins((super::RonAssetPlugin::<ClientConfig>::new(&["client.ron"]),));
        }
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

    #[derive(SystemParam)]
    pub struct ClientConfigSystemParam<'w> {
        config_asset: Res<'w, super::MyConfigAsset>,
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
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize, PartialEq)]
pub struct TankConfigs {
    pub tanks: HashMap<TankType, TankConfig>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TankConfig {
    /// The speed at which the tank at maximum moves per tick
    pub move_speed: f32,
    /// The speed at which the body of the tank at maximum rotates per tick in radians
    pub body_rotation_speed: f32,
    /// Yaw rotation speed of the turret in radians per tick
    pub turret_yaw_rotation_speed: f32,
    /// Pitch rotation speed of the turret in radians per tick
    pub turret_pitch_rotation_speed: f32,
    pub turret_max_pitch: f32,
    pub turret_min_pitch: f32,
    /// The maximum height this tank can "climb"
    pub max_slope: f32,
    /// The size of the tank (Vec3, x = width, y = height, z = depth)
    /// full-extents for x (width), z (depth) and y (height)
    pub size: Vec3,
    /// Shooting cooldown in ticks. The tank can only shoot again after this many ticks.
    pub shoot_cooldown: u32,
    pub projectile_damage: f32,
    pub projectile_speed: f32,
    /// The lifetime of the projectile in ticks
    pub projectile_lifetime: u32,
    /// The size of the projectile (Vec3, x = width, y = height, z = depth)
    /// full-extents for x (width), z (depth) and y (height)
    pub projectile_size: Vec3,
    pub projectile_gravity: f32,
    /// The maximum amount of health this tank can have
    pub max_health: f32,
    /// The armor of the tank on each side. value between 0 and 1
    /// 0 = no armor, 1 = full armor (no damage when no armor penetration)
    pub armor: HashMap<Side, f32>,
    /// The time in ticks it takes for the tank to respawn after dying
    pub respawn_timer: u32,
}

#[derive(SystemParam)]
pub struct TankConfigSystemParam<'w> {
    config_asset: Res<'w, MyConfigAsset>,
    tank_configs: Res<'w, Assets<TankConfigs>>,
}

impl<'w> TankConfigSystemParam<'w> {
    pub fn get_tank_type_config(&self, tank_type: &TankType) -> Option<&TankConfig> {
        self.tank_configs
            .get(self.config_asset.tank.id())
            .and_then(|tank_configs| tank_configs.tanks.get(tank_type))
    }

    pub fn tank_configs(&self) -> &TankConfigs {
        self.tank_configs
            .get(self.config_asset.tank.id())
            .expect("Tank configs not loaded")
    }
}
