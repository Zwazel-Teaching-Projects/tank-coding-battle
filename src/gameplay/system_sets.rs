use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyGameplaySet {
    CollectBotInput,
    ApplyBotInput,
    UpdateGameState,
    TickIncrease,
}
