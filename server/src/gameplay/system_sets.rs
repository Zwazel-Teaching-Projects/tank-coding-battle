use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyGameplaySet {
    TickTimerProcessing,
    ProcessMessagesBeforeLobbyReady,
    IncrementTick,
    ProcessCommands,
    RunSimulationStep,
    SimulationStepDone,
    UpdatingGameStates,
}
