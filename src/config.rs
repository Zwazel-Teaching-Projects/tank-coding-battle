use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::main_state::MyMainState;

pub struct MyConfigPlugin;

impl Plugin for MyConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<MyConfig>::new(&["config.ron"]))
            .register_type::<MyConfig>()
            .register_type::<MyConfigHandle>()
            .register_type::<TeamConfig>()
            .add_systems(Startup, load_config)
            .add_systems(
                Update,
                insert_config.run_if(in_state(MyMainState::SettingUp)),
            );
    }
}

fn load_config(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(MyConfigHandle(asset_server.load("config.ron")));
}

fn insert_config(
    config_handle: Res<MyConfigHandle>,
    mut configs: ResMut<Assets<MyConfig>>,
    mut commands: Commands,
    mut state: ResMut<NextState<MyMainState>>,
) {
    if let Some(config) = configs.remove(config_handle.id()) {
        commands.insert_resource(config.clone());
        commands.remove_resource::<MyConfigHandle>();
        state.set(MyMainState::Ready);
    }
}

#[derive(Debug, Reflect, Clone, Deref, DerefMut, Resource)]
#[reflect(Resource)]
struct MyConfigHandle(pub Handle<MyConfig>);

#[derive(Debug, Default, Deserialize, Asset, Reflect, Resource, Clone)]
#[reflect(Resource)]
pub struct MyConfig {
    pub server_ip: String,
    pub server_port: u16,
    pub teams: Vec<TeamConfig>,
    pub tick_rate: f32,
}

#[derive(Debug, Deserialize, Clone, Reflect, Default)]
pub struct TeamConfig {
    pub name: String,
    pub color: Color,
    pub max_players: usize,
}
