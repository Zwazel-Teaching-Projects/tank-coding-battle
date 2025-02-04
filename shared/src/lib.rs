use asset_handling::MyAssetHandlingPlugin;
use bevy::prelude::*;
use game::MySharedGamePlugin;
use main_state::MyMainState;
use networking::MySharedNetworkingPlugin;

pub mod asset_handling;
pub mod game;
pub mod main_state;
pub mod networking;

pub struct MySharedPlugin;

impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MySharedGamePlugin,
            MySharedNetworkingPlugin,
            MyAssetHandlingPlugin,
        ))
        .init_state::<MyMainState>();

        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<MyMainState>,
        );
    }
}
