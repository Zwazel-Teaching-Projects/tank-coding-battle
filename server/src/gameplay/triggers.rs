use bevy::prelude::*;

#[derive(Debug, Reflect, Event)]
pub struct StartNextTickProcessingTrigger;

#[derive(Debug, Reflect, Event)]
pub struct CollectAndTriggerMessagesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct StartNextSimulationStepTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateLobbyGameStateTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateClientGameStatesTrigger {
    pub lobby: Entity,
}

#[derive(Debug, Reflect, Event)]
pub struct AddStateUpdateToQueue;

#[derive(Debug, Reflect, Event)]
pub struct SendOutgoingMessagesTrigger;
