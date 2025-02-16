use bevy::prelude::*;

#[derive(Debug, Reflect, Event)]
pub struct StartNextTickProcessingTrigger;

#[derive(Debug, Reflect, Event)]
pub struct StartNextSimulationStepTrigger;

#[derive(Debug, Reflect, Event)]
pub struct NextSimulationStepDoneTrigger;

#[derive(Debug, Reflect, Event)]
pub struct SendOutgoingMessagesTrigger;

#[derive(Debug, Reflect, Event)]
pub struct UpdateClientGameStatesTrigger {
    pub lobby: Entity,
}
