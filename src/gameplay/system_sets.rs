use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyGameplaySet {
    TickTimerProcessing,
    CollectBotInput,
    ApplyBotInput,
    UpdateGameState,
    IncrementTick,
}
