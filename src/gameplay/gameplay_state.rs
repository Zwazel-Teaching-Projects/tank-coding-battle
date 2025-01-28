use bevy::prelude::*;

use crate::main_state::MyMainState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, SubStates)]
#[source(MyMainState = MyMainState::Ready)]
pub enum MyGameplayState {
    #[default]
    WaitingForBots,
    Running,
}
