use bevy::prelude::*;
use shared::networking::messages::message_container::GameStartsTrigger;

use crate::game_state::MyGameState;

pub fn game_starts(_: Trigger<GameStartsTrigger>, mut game_state: ResMut<NextState<MyGameState>>) {
    game_state.set(MyGameState::GameStarted);
}
