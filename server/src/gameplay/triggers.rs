use bevy::prelude::*;

#[derive(Debug, Reflect, Event)]
pub struct StartNextTickProcessingTrigger;

#[derive(Debug, Reflect, Event)]
pub struct CollectAndTriggerMessagesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct MovePorjectilesSimulationStepTrigger;

#[derive(Debug, Reflect, Event)]
pub struct CheckForCollisionsTrigger;

#[derive(Debug, Reflect, Event)]
pub struct DespawnOutOfBoundsProjectilesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct CheckHealthTrigger;

#[derive(Debug, Reflect, Event)]
pub struct MoveFlagsSimulationStepTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateLobbyGameStateTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateClientGameStatesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct AddStateUpdateToQueue;

#[derive(Debug, Reflect, Event)]
pub struct SendOutgoingMessagesTrigger;
