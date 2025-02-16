use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Event)]
pub struct StartNextTickProcessingTrigger;

#[derive(Debug, Default, Reflect, Event)]
pub struct StartNextSimulationStepTrigger;

#[derive(Debug, Default, Reflect, Event)]
pub struct NextSimulationStepDoneTrigger;

#[derive(Debug, Default, Reflect, Event)]
pub struct SendOutgoingMessagesTrigger;

#[derive(Debug, Default, Reflect, Event)]
pub struct UpdateClientGameStatesTrigger;
