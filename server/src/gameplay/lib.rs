use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Event)]
pub struct StartNextTickProcessing;

#[derive(Debug, Default, Reflect, Event)]
pub struct TickIncreasedTrigger;
