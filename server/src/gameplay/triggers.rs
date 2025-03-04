use bevy::prelude::*;

#[derive(Debug, Reflect, Event)]
pub struct StartNextTickProcessingTrigger;

#[derive(Debug, Reflect, Event)]
pub struct CollectAndTriggerMessagesTrigger;

/// In this trigger, we simulate the tick for all entities.
/// E.g. we move the entities.
#[derive(Debug, Reflect, Event)]
pub struct StartNextSimulationStepTrigger;

/// In this trigger, we finish the simulation step.
/// E.g. we check for collisions.
#[derive(Debug, Reflect, Event)]
pub struct FinishedNextSimulationStepTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateLobbyGameStateTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateClientGameStatesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct AddStateUpdateToQueue;

#[derive(Debug, Reflect, Event)]
pub struct SendOutgoingMessagesTrigger;
