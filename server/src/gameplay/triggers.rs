use bevy::prelude::*;

#[derive(Debug, Reflect, Event)]
pub struct StartNextTickProcessingTrigger;

#[derive(Debug, Reflect, Event)]
pub struct CollectAndTriggerMessagesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct StartNextSimulationStepTrigger;

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
