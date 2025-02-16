use bevy::prelude::*;
use shared::networking::networking_state::MyNetworkingState;

#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(MyNetworkingState = MyNetworkingState::Running)]
pub enum MyGameState {
    #[default]
    SettingUp,
    GameToldToStart,
    GameStarted,
}
