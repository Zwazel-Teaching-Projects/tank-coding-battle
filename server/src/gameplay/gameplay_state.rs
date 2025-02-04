use bevy::prelude::*;
use shared::main_state::MyMainState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, SubStates)]
#[source(MyMainState = MyMainState::Ready)]
pub enum MyGameplayState {
    #[default]
    WaitingForBots,
    Running,
}
