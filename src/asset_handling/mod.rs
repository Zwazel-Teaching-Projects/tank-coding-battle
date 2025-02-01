use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use config::MyConfigPlugin;
use maps::MyMapPlugin;

use crate::main_state::MyMainState;

pub mod config;
pub mod maps;

pub struct MyAssetHandlingPlugin;

impl Plugin for MyAssetHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(MyMainState::SettingUp).continue_to_state(MyMainState::Ready),
        )
        .add_plugins((MyConfigPlugin, MyMapPlugin));
    }
}
