use asset_handling::MyAssetHandlingPlugin;
use bevy::prelude::*;
use gameplay::MyGameplayPlugin;
use main_state::MyMainState;
use networking::MyNetworkingPlugin;

pub mod asset_handling;
pub mod gameplay;
pub mod main_state;
pub mod networking;

pub struct MyServerPlugin;

impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MyAssetHandlingPlugin, MyGameplayPlugin, MyNetworkingPlugin))
            .init_state::<MyMainState>();

        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<MyMainState>,
        );
    }
}
