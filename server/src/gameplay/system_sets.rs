use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyGameplaySet {
    TickTimerProcessing,
    IncrementTick,
    RunSimulationStep,
    SimulationStepDone,
}
